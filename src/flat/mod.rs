//! M<sub>2</sub>, an extension of M<sub>1</sub> that allows for conjunctions
//! (without backtracking).

mod compile;
mod control;
mod store;

use std::collections::HashMap;

use failure::Error;

use common::{Clause, Functor, HeapCell, Structure, Term, Variable};

pub use self::control::{Instruction, Location};
pub use self::compile::{compile_program, compile_query};
use self::store::{Heap, Registers};

/// An abstract machine for M<sub>2</sub>.
#[derive(Debug)]
pub struct Machine {
    /// All stored code.
    code: Vec<Instruction>,

    /// All code labels.
    labels: HashMap<Functor, usize>,

    /// The instruction pointer.
    p: usize,

    /// The stored instruction pointer, aka the continuation point.
    cp: usize,

    /// The unification pointer.
    s: usize,

    /// Whether unification failed.
    fail: bool,

    /// Whether the machine is in write mode.
    write_mode: bool,

    /// The registers.
    registers: Registers,

    /// The stack.
    stack: Vec<usize>,

    /// The heap.
    heap: Heap,
}

impl Machine {
    /// Compiles a set of clauses into a program.
    pub fn new(program: &[Clause]) -> Result<Machine, Error> {
        compile_program(program).map(|(c, l)| Machine::with_code(c, l))
    }

    /// Creates a new Machine containing the given code and labels.
    pub fn with_code(
        code: Vec<Instruction>,
        labels: HashMap<Functor, usize>,
    ) -> Machine {
        Machine {
            code,
            labels,
            p: 0,
            cp: 0,
            s: 0,
            fail: false,
            write_mode: false,
            registers: Registers::new(),
            stack: Vec::new(),
            heap: Heap::new(),
        }
    }

    /// Resets the state of the machine.
    pub fn reset(&mut self) {
        self.p = 0;
        self.cp = 0;
        self.fail = false;
        self.write_mode = false;
        self.registers.reset();
        self.stack.clear();
        self.heap.reset();
    }

    /// Runs a single instruction. Returns whether unification just succeeded.
    pub fn run_instruction(&mut self, instr: Instruction) -> bool {
        trace!("{}", instr);
        match instr {
            Instruction::GetStructure(functor, loc) => {
                let addr = self.heap.deref(self.read(loc));
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

            Instruction::PutStructure(functor, loc) => {
                let n = self.heap.alloc_with(|n| HeapCell::Str(n + 1));
                self.heap.alloc(HeapCell::Functor(functor));
                self.write(loc, n);
                false
            }

            Instruction::UnifyValue(loc) => {
                if self.write_mode {
                    let val = self.heap[self.read(loc)];
                    self.heap.alloc(val);
                } else {
                    let a1 = self.read(loc);
                    let a2 = self.s;
                    self.unify(a1, a2);
                }
                self.s += 1;
                false
            }
            Instruction::UnifyVariable(loc) => {
                let addr = if self.write_mode {
                    self.heap.alloc_with(|n| HeapCell::Ref(n))
                } else {
                    self.s
                };
                self.write(loc, addr);
                self.s += 1;
                false
            }

            Instruction::Call(ref f) => {
                self.cp = self.p + 1;
                self.p = self.labels[f];
                false
            }
            Instruction::Proceed => {
                self.p = self.cp;
                false
            }

            i => unimplemented!("instruction not implemented: {}", i),
        }
    }

    /// Reads a value from the given location. Returns a heap address.
    pub fn read(&self, loc: Location) -> usize {
        match loc {
            Location::Register(n) => self.registers[n],
            Location::Local(n) => unimplemented!("index local"),
        }
    }

    /// Runs a single instruction, based on the current instruction pointer.
    /// Returns whether unification just succeeded.
    pub fn step(&mut self) -> bool {
        let instr = if let Some(instr) = self.code.get(self.p) {
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

    /// Writes a heap address to the given location.
    pub fn write(&mut self, loc: Location, addr: usize) {
        match loc {
            Location::Register(n) => self.registers[n] = addr,
            Location::Local(n) => unimplemented!("index local"),
        }
    }
}

impl ::Machine for Machine {
    fn run_query<'a>(
        &'a mut self,
        query: Vec<Structure>,
    ) -> Box<'a + Iterator<Item = Result<HashMap<Variable, Term>, Error>>> {
        self.reset();

        let (query_code, vars) = compile_query(&query);

        for instr in query_code {
            assert!(!self.run_instruction(instr));
            assert!(!self.fail);
        }

        Box::new(MachineIter {
            machine: self,
            _vars: vars,
        })
    }
}

struct MachineIter<'a> {
    machine: &'a mut Machine,
    _vars: Vec<Variable>,
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

#[cfg(test)]
mod tests {
    use Machine as MachineTrait;
    use super::*;
    use test_utils::{example_program, example_query};

    #[test]
    fn works_for_example_program() {
        let program = vec![Clause(example_program(), vec![])];
        let mut machine =
            Machine::new(&program).expect("Couldn't build machine");
        let matches = machine
            .run_query(vec![example_query()])
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to run query");
        assert_eq!(
            matches,
            vec![
                vec![
                    (
                        variable!("W"),
                        Term::Structure(Structure(
                            atom!(f),
                            vec![Term::Structure(Structure(atom!(a), vec![]))],
                        )),
                    ),
                    (
                        variable!("Z"),
                        Term::Structure(Structure(
                            atom!(f),
                            vec![
                                Term::Structure(Structure(
                                    atom!(f),
                                    vec![
                                        Term::Structure(Structure(
                                            atom!(a),
                                            vec![],
                                        )),
                                    ],
                                )),
                            ],
                        )),
                    ),
                ].into_iter()
                    .collect(),
            ]
        );
    }
}
