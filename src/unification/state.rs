use failure::Error;

use common::{Functor, Term, Variable};

#[derive(Debug)]
pub struct State {
    heap: Vec<HeapCell>,
    pub mode: Mode,
}

impl State {
    /// Creates a new, empty State.
    pub fn new() -> State {
        State {
            mode: Mode::Write,
            heap: Vec::new(),
        }
    }

    /// Recursively retrieves a value from a heap cell.
    pub fn deref(&self, addr: usize) -> HeapCell {
        match self.get(addr) {
            HeapCell::Ref(a) if a != addr => self.deref(a),
            cell => cell,
        }
    }

    /// Retrieves the value of a heap cell. Panics if the address is out of
    /// bounds.
    pub fn get(&self, addr: usize) -> HeapCell {
        self.heap[addr]
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

    /// Extracts a Term from the given HeapCell, which may be from a register.
    /// If it is not, the heap index of the cell should be passed in as the
    /// last argument.
    pub fn extract_term(&self, idx: usize) -> Result<Term, Error> {
        match self.heap[idx] {
            HeapCell::Functor(f) => {
                bail!("Found functor data {} where a term was expected", f)
            }
            HeapCell::Ref(n) => if idx == n {
                let var = Variable::from_str(format!("_{}", n)).unwrap();
                Ok(Term::Variable(var))
            } else {
                self.extract_term(n)
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
                    subterms.push(self.extract_term(f_idx + i + 1)?);
                }
                Ok(Term::Structure(atom, subterms))
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum HeapCell {
    Functor(Functor),
    Ref(usize),
    Str(usize),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Read,
    Write,
}
