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
        debug_assert_eq!(n, n2);
        n2
    }

    /// Extracts a Term from a given location in the heap.
    pub fn extract_term(&self, idx: usize) -> Result<Term, Error> {
        match self.heap[idx] {
            HeapCell::Functor(f) => {
                bail!("Found functor {} where a term was expected", f)
            }
            HeapCell::Ref(n) => if n == idx {
                Ok(Term::Variable(
                    Variable::from_str(format!("_{}", n)).unwrap(),
                ))
            } else {
                self.extract_term(n)
            },
            HeapCell::Str(f_idx) => {
                let functor = match self.heap[f_idx] {
                    HeapCell::Functor(f) => f,
                    cell => {
                        bail!("Found {:?} where functor was expected", cell)
                    }
                };
                println!("{}", functor);
                //
                unimplemented!()
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
