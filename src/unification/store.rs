use std::collections::HashMap;

use failure::Error;

use common::{Functor, HeapCell, Structure, Term, Variable};

/// The heap, as well as some "small" data that are not in the numbered
/// registers.
#[derive(Debug)]
pub struct Store {
    heap: Vec<HeapCell>,

    /// Whether unification has failed.
    pub fail: bool,

    /// The mode the unification instructions operate in.
    pub mode: Mode,

    /// The location at which unification is occurring.
    pub s: usize,
}

impl Store {
    /// Creates a new, empty Store.
    pub fn new() -> Store {
        Store {
            fail: false,
            mode: Mode::Write,
            heap: Vec::new(),
            s: 0,
        }
    }

    /// Returns the address a cell derefs to.
    pub fn deref(&self, addr: usize) -> usize {
        match self.get(addr) {
            HeapCell::Ref(a) if a != addr => self.deref(a),
            _ => addr,
        }
    }

    /// Retrieves the value of a heap cell. Panics if the address is out of
    /// bounds.
    pub fn get(&self, addr: usize) -> HeapCell {
        self.heap[addr]
    }

    /// Given the address of a functor cell, returns the functor stored in it.
    /// Panics if given an address that doesn't point to a functor.
    pub fn get_functor(&self, addr: usize) -> Functor {
        match self.get(addr) {
            HeapCell::Functor(f) => f,
            cell => {
                panic!("Expecting {} to be a functor, found {:?}", addr, cell)
            }
        }
    }

    /// Returns the address that will be returned by the next call to push.
    pub fn next_addr(&self) -> usize {
        self.heap.len()
    }

    /// Adds a cell to the end of the heap, returning its address.
    pub fn push(&mut self, cell: HeapCell) -> usize {
        let n = self.heap.len();
        self.heap.push(cell);
        n
    }

    /// A helper that pushes a cell to the heap, obtaining the cell by calling
    /// a function that receives that cell's address.
    pub fn push_with<F>(&mut self, f: F) -> usize
    where
        F: FnOnce(usize) -> HeapCell,
    {
        let n = self.next_addr();
        let n2 = self.push(f(n));
        assert_eq!(n, n2);
        n2
    }

    /// Resets the heap to its initial state, without ceding its allocation.
    pub fn reset(&mut self) {
        self.fail = false;
        self.heap.clear();
        self.s = 0;
    }

    /// Binds one term to another. At least one given address must deref to a
    /// self-referential (unbound) `Ref` cell.
    pub fn bind(&mut self, a: usize, b: usize) {
        let da = self.deref(a);
        let db = self.deref(b);
        if self.get(da).is_ref() {
            self.heap[da] = HeapCell::Ref(db);
        } else {
            assert!(self.get(db).is_ref());
            self.heap[db] = HeapCell::Ref(da);
        }
    }

    /// Extracts a Term from the given HeapCell, which may be from a register.
    /// If it is not, the heap index of the cell should be passed in as the
    /// last argument.
    pub fn extract_term(
        &self,
        idx: usize,
        names: Option<&HashMap<usize, Variable>>,
    ) -> Result<Term, Error> {
        match self.heap[idx] {
            HeapCell::Functor(f) => {
                bail!("Found functor data {} where a term was expected", f)
            }
            HeapCell::Ref(n) => if idx == n {
                let var = names
                    .and_then(|names| names.get(&n).map(|v| v.clone()))
                    .unwrap_or_else(|| {
                        Variable::from_str(format!("_{}", n)).unwrap()
                    });
                Ok(Term::Variable(var))
            } else {
                self.extract_term(n, names)
            },
            HeapCell::Str(f_idx) => {
                let Functor(atom, arity) = match self.heap[f_idx] {
                    HeapCell::Functor(f) => f,
                    cell => {
                        bail!("Found {:?} where functor was expected", cell)
                    }
                };
                let mut subterms = vec![];
                for i in 0..arity {
                    subterms.push(self.extract_term(f_idx + i + 1, names)?);
                }
                Ok(Term::Structure(Structure(atom, subterms)))
            }
        }
    }
}

/// The mode the interpreter is in, for the unify instructions.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    /// Read mode, in which unification with existing values is attempted.
    Read,

    /// Write mode, in which an exemplar of the value is built.
    Write,
}
