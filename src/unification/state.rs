use failure::Error;

use common::{Functor, Term, Variable};

#[derive(Debug)]
pub struct State {
    heap: Vec<HeapCell>,
}

impl State {
    /// Creates a new, empty State.
    pub fn new() -> State {
        State { heap: Vec::new() }
    }

    /// Deletes the contents of the heap.
    pub fn clear(&mut self) {
        self.heap.clear();
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
    pub fn extract_term(
        &self,
        cell: HeapCell,
        idx: Option<usize>,
    ) -> Result<Term, Error> {
        match cell {
            HeapCell::Functor(f) => {
                bail!("Found functor data {} where a term was expected", f)
            }
            HeapCell::Ref(n) => if idx.map(|i| n == i).unwrap_or(false) {
                let var = Variable::from_str(format!("_{}", n)).unwrap();
                Ok(Term::Variable(var))
            } else {
                self.extract_term_from(n)
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
                    subterms.push(self.extract_term_from(f_idx + i + 1)?);
                }
                Ok(Term::Structure(atom, subterms))
            }
        }
    }

    /// Extracts a Term from a given location in the heap.
    pub fn extract_term_from(&self, idx: usize) -> Result<Term, Error> {
        self.extract_term(self.heap[idx], Some(idx))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum HeapCell {
    Functor(Functor),
    Ref(usize),
    Str(usize),
}
