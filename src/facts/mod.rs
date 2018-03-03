//! M<sub>1</sub>, an extension of M<sub>0</sub> that attempts unification
//! against several facts instead of just one.

mod control;
mod program;
mod query;
mod store;

use std::collections::HashMap;
use std::iter::once;

use failure::Error;

use common::{Functor, HeapCell, Structure, Term, Variable};

pub use self::control::Instruction;
pub use self::program::compile_program;
pub use self::query::compile_query;
use self::store::{Heap, Registers};

/// An abstract machine for M<sub>1</sub>.
#[derive(Debug)]
pub struct Machine {
    /// All stored code.
    code: Vec<Instruction>,

    /// All code labels.
    labels: HashMap<Functor, usize>,

    /// The instruction pointer.
    ip: usize,

    /// The unification pointer.
    s: usize,

    /// Whether unification failed.
    fail: bool,

    /// Whether the machine is in write mode.
    write_mode: bool,

    /// The registers.
    registers: Registers,

    /// The heap.
    heap: Heap,
}

impl Machine {
    /// Compiles a set of facts into a program.
    pub fn new(facts: Vec<Structure>) -> Machine {
        let (code, labels) = compile_program(facts);
        Machine::with_code(code, labels)
    }

    /// Creates a new Machine containing the given code and labels.
    pub fn with_code(
        code: Vec<Instruction>,
        labels: HashMap<Functor, usize>,
    ) -> Machine {
        Machine {
            code,
            labels,
            ip: 0,
            s: 0,
            fail: false,
            write_mode: false,
            registers: Registers::new(),
            heap: Heap::new(),
        }
    }

    /// Resets the state of the machine.
    pub fn reset(&mut self) {
        self.ip = 0;
        self.fail = false;
        self.write_mode = false;
        self.registers.reset();
        self.heap.reset();
    }

    /// Runs a single instruction. Returns whether unification just succeeded.
    pub fn run_instruction(&mut self, instr: Instruction) -> bool {
        trace!("{}", instr);
        match instr {
            Instruction::PutStructure(functor, reg) => {
                let n = self.heap.alloc_with(|n| HeapCell::Str(n + 1));
                self.heap.alloc(HeapCell::Functor(functor));
                self.registers[reg] = n;
                false
            }
            Instruction::SetVariable(reg) => {
                let n = self.heap.alloc_with(|n| HeapCell::Ref(n));
                self.registers[reg] = n;
                false
            }
            Instruction::SetValue(reg) => {
                let cell = self.heap[self.registers[reg]];
                self.heap.alloc(cell);
                false
            }

            Instruction::GetStructure(functor, reg) => {
                let addr = self.heap.deref(self.registers[reg]);
                match self.heap[addr] {
                    HeapCell::Ref(_) => {
                        let n = self.heap.alloc_with(|n| HeapCell::Str(n + 1));
                        self.heap.alloc(HeapCell::Functor(functor));
                        self.heap.bind(addr, n);
                        self.write_mode = true;
                    }
                    HeapCell::Str(a) => match self.heap[a] {
                        HeapCell::Functor(f) if f == functor => {
                            self.s = a + 1;
                            self.write_mode = false;
                        }
                        _ => {
                            self.s = 0;
                            self.fail = true;
                        }
                    },
                    _ => panic!("Invalid deref in {}", instr),
                }
                false
            }
            Instruction::UnifyVariable(reg) => {
                if self.write_mode {
                    self.registers[reg] =
                        self.heap.alloc_with(|n| HeapCell::Ref(n));
                } else {
                    self.registers[reg] = self.s;
                }
                self.s += 1;
                false
            }
            Instruction::UnifyValue(reg) => {
                if self.write_mode {
                    let val = self.heap[self.registers[reg]];
                    self.heap.alloc(val);
                } else {
                    let a1 = self.registers[reg];
                    let a2 = self.s;
                    self.unify(a1, a2);
                }
                self.s += 1;
                false
            }

            Instruction::Proceed => true,

            i => unimplemented!("instruction not implemented {}", i),
        }
    }

    /// Runs a single instruction, based on the current instruction pointer.
    /// Returns whether unification just succeeded.
    pub fn step(&mut self) -> bool {
        let instr = if let Some(instr) = self.code.get(self.ip) {
            *instr
        } else {
            panic!("ip out of bounds")
        };
        self.run_instruction(instr)
    }

    /// Performs unification between two heap terms.
    fn unify(&mut self, a1: usize, a2: usize) {
        let mut pdl = vec![a1, a2];
        while !pdl.is_empty() && !self.fail {
            let d1 = self.heap.deref(pdl.pop().unwrap());
            let d2 = self.heap.deref(pdl.pop().unwrap());
            if d1 != d2 {
                match (self.heap[d1], self.heap[d2]) {
                    (HeapCell::Str(v1), HeapCell::Str(v2)) => {
                        let f1 = self.heap.get_functor(v1);
                        let f2 = self.heap.get_functor(v2);
                        if f1 == f2 {
                            for i in 1..(f1.1 + 1) {
                                pdl.push(v1 + i);
                                pdl.push(v2 + i);
                            }
                        } else {
                            self.fail = true;
                            return;
                        }
                    }
                    _ => self.heap.bind(d1, d2),
                }
            }
        }
    }
}

impl ::Machine for Machine {
    fn run_query<'a>(
        &'a mut self,
        mut query: Vec<Structure>,
    ) -> Box<'a + Iterator<Item = Result<HashMap<Variable, Term>, Error>>> {
        if query.len() != 1 {
            let err =
                format_err!("M1 doesn't support conjunctions in queries.");
            return Box::new(once(Err(err)));
        }
        let query = query.remove(0);

        self.reset();

        let (query_code, vars) = compile_query(query);

        for instr in query_code {
            assert!(!self.run_instruction(instr));
            assert!(!self.fail);
        }

        Box::new(MachineIter {
            machine: self,
            vars,
        })
    }
}

struct MachineIter<'a> {
    machine: &'a mut Machine,
    vars: HashMap<Variable, usize>,
}

impl<'a> Iterator for MachineIter<'a> {
    type Item = Result<HashMap<Variable, Term>, Error>;

    fn next(&mut self) -> Option<Result<HashMap<Variable, Term>, Error>> {
        loop {
            if self.machine.step() {
                unimplemented!("unification succeeded")
            } else if self.machine.fail {
                return None;
            }
        }
    }
}
