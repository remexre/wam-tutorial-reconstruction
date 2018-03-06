use std::collections::{HashMap, HashSet};

use common::{FlatTermValue, Functor, Structure, Term, Variable};

use super::control::Instruction;

fn compile(
    seen: &mut HashSet<usize>,
    reg: usize,
    functor: Option<Functor>,
) -> Instruction {
    let new = seen.insert(reg);
    if let Some(functor) = functor {
        Instruction::PutStructure(functor, reg)
    } else if new {
        Instruction::SetVariable(reg)
    } else {
        Instruction::SetValue(reg)
    }
}

/// This is just a topological sort implemented via a depth-first search. It
/// should be called on each index of `flats`.
fn compile_visitor(
    code: &mut Vec<Instruction>,
    seen: &mut HashSet<usize>,
    flats: &mut Vec<Option<FlatTermValue>>,
    vars: &mut HashMap<Variable, usize>,
    current: usize,
) {
    if let Some(val) = flats[current].take() {
        match val {
            FlatTermValue::Structure(a, ref args) => {
                for &arg in args {
                    compile_visitor(code, seen, flats, vars, arg);
                }
                code.push(compile(seen, current, Some(Functor(a, args.len()))));
                for &arg in args {
                    code.push(compile(seen, arg, None));
                }
            }
            FlatTermValue::Variable(Some(var)) => {
                let overwrote = vars.insert(var, current);
                assert!(overwrote.is_none());
            }
            FlatTermValue::Variable(None) => {}
        }
    }
}

/// Compiles an argument.
pub fn compile_argument(
    code: &mut Vec<Instruction>,
    seen: &mut HashSet<usize>,
    flats: &mut Vec<Option<FlatTermValue>>,
    vars: &mut HashMap<Variable, usize>,
    current: usize,
) {
    if let Some(val) = flats[current].take() {
        match val {
            FlatTermValue::Structure(_, _) => {
                flats[current] = Some(val);
                compile_visitor(code, seen, flats, vars, current);
            }
            FlatTermValue::Variable(Some(var)) => unimplemented!(),
            FlatTermValue::Variable(None) => unimplemented!(),
        }
    }
}

/// Compiles a term into instructions that will construct the term on the
/// heap, storing the root into the given register number.
pub fn compile_query(
    query: Structure,
) -> (Vec<Instruction>, HashMap<Variable, usize>) {
    let mut flat = query.flatten().0.into_iter().map(Some).collect::<Vec<_>>();
    let mut seen = HashSet::with_capacity(flat.len());
    let mut code = Vec::new();
    let mut vars = HashMap::new();

    for i in 0..query.functor().1 {
        compile_argument(&mut code, &mut seen, &mut flat, &mut vars, i);
    }
    assert!(flat.iter().all(Option::is_none));

    if code.is_empty() {
        panic!("code is empty")
    }

    (code, vars)
}
