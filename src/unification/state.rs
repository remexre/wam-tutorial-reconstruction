use failure::Error;

use common::{Functor, Term, Variable};

#[derive(Debug)]
pub struct State {
    pub heap: Vec<HeapCell>,
}

impl State {
    pub fn new() -> State {
        State { heap: Vec::new() }
    }

    pub fn extract_term(&self, idx: usize) -> Result<Term, Error> {
        match self.heap[idx] {
            HeapCell::Functor(f) => bail!("Found functor {} where a term was expected", f),
            HeapCell::Ref(n) => if n == idx {
                Ok(Term::Variable(Variable::from_str(format!("_{}", n)).unwrap()))
            } else {
                self.extract_term(n)
            },
            HeapCell::Str(f_idx) => {
                let functor = match self.heap[f_idx] {
                    HeapCell::Functor(f) => f,
                    cell => bail!("Found {:?} where functor was expected", cell),
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
