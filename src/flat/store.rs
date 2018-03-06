use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use failure::Error;

use common::{Functor, HeapCell, Structure, Term, Variable};

#[derive(Debug)]
pub struct Heap(Vec<HeapCell>);

impl Heap {
    /// Constructs a new, empty Heap.
    pub fn new() -> Heap {
        Heap(Vec::new())
    }

    /// Allocates a single heap cell whose value is not address-dependent.
    /// Returns the address the cell was allocated at.
    pub fn alloc(&mut self, cell: HeapCell) -> usize {
        self.alloc_with(|_| cell)
    }

    /// Allocates a single heap cell. The address the heap cell will be placed
    /// at is passed to the function. Returns the address the cell was
    /// allocated at.
    pub fn alloc_with<F: FnOnce(usize) -> HeapCell>(&mut self, f: F) -> usize {
        let n = self.0.len();
        self.0.push(f(n));
        n
    }

    /// Binds one term to another. At least one given address must deref to a
    /// self-referential (unbound) `Ref` cell.
    pub fn bind(&mut self, a: usize, b: usize) {
        let da = self.deref(a);
        let db = self.deref(b);
        if self[da].is_ref() {
            self.0[da] = HeapCell::Ref(db);
        } else {
            assert!(self[db].is_ref());
            self.0[db] = HeapCell::Ref(da);
        }
    }

    /// Derefs an address, resolving any `Ref` cells.
    pub fn deref(&self, addr: usize) -> usize {
        match self[addr] {
            HeapCell::Ref(a) if a != addr => self.deref(a),
            _ => addr,
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
        match self[idx] {
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
                let Functor(atom, arity) = match self[f_idx] {
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

    /// Gets the functor stored at the given address. If a functor is not
    /// stored at the address, will panic.
    pub fn get_functor(&self, addr: usize) -> Functor {
        match self[addr] {
            HeapCell::Functor(f) => f,
            cell => {
                panic!("Expecting {} to be a functor, found {:?}", addr, cell)
            }
        }
    }

    /// Clears the heap.
    pub fn reset(&mut self) {
        self.0.clear();
    }
}

impl Index<usize> for Heap {
    type Output = HeapCell;

    fn index(&self, i: usize) -> &HeapCell {
        &self.0[i]
    }
}

impl IndexMut<usize> for Heap {
    fn index_mut(&mut self, i: usize) -> &mut HeapCell {
        &mut self.0[i]
    }
}

#[derive(Debug)]
pub struct Registers(Vec<usize>);

impl Registers {
    /// Constructs a new, empty Registers object.
    pub fn new() -> Registers {
        Registers(Vec::new())
    }

    /// Resets all the registers.
    pub fn reset(&mut self) {
        self.0.clear();
    }
}

impl Index<usize> for Registers {
    type Output = usize;

    fn index(&self, i: usize) -> &usize {
        &self.0[i]
    }
}

impl IndexMut<usize> for Registers {
    fn index_mut(&mut self, i: usize) -> &mut usize {
        while self.0.len() <= i {
            self.0.push(::std::usize::MAX);
        }
        &mut self.0[i]
    }
}
