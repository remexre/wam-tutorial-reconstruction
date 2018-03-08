use std::collections::{HashMap, HashSet};

use common::{Structure, Term, Variable};

use super::flatten::{flatten, FlatTerm};
use super::super::{Instruction, Location};

/// Compiles a query or rule into a series of instructions and a mapping
/// between named variables and the registers the variables are stored in. If
/// the head is `None`, compile as a query. Otherwise, compiles as a rule.
///
/// Note that if compiled as a query, all variables are marked as permanent and
/// the stack frame is not deallocated after running the code.
pub fn compile(
    head: Option<&Structure>,
    body: &[Structure],
) -> (Vec<Instruction>, Vec<Variable>) {
    let vars = find_variables(head, body);
    let permanent: Vec<Variable> = if head.is_some() {
        vars.into_iter()
            .filter(|&(_, n)| n > 1)
            .map(|(v, _)| v)
            .collect()
    } else {
        vars.into_iter().map(|p| p.0).collect()
    };
    //let permanent_map =
    //permanent.iter().enumerate().map(|(i, &v)| (v, i)).collect();

    let mut code = vec![Instruction::Allocate(permanent.len())];
    //let mut seen_vars = HashSet::new();

    //let mut vars = HashMap::new();
    // TODO: Clean this up.
    let body = if let Some(head) = head {
        /*
        compile_head(
            &mut code,
            &mut seen_vars,
            &mut vars,
            &permanent_map,
            &head,
        );
        compile_body(
            &mut code,
            &mut seen_vars,
            &mut vars,
            &permanent_map,
            &body[0],
        );
        */
        &body[1..]
    } else {
        body
    };
    for s in body {
        //vars.clear();
        // compile_body(&mut code, &mut seen_vars, &mut vars, &permanent_map, s);
    }
    if head.is_none() {
        code.push(Instruction::Deallocate);
    }

    (code, permanent)
}

fn compile_head(
    code: &mut Vec<Instruction>,
    seen: &mut HashSet<usize>,
    vars: &mut HashMap<Variable, Location>,
    permanent: &HashMap<Variable, usize>,
    s: &Structure,
) {
    for (i, f) in flatten(s).into_iter().enumerate() {
        match f {
            FlatTerm::Functor(a, is) => unimplemented!(),
            FlatTerm::Ref(n) => unimplemented!(),
            FlatTerm::Variable(Some(v)) => unimplemented!(),
            FlatTerm::Variable(None) => unimplemented!(),
        }
    }
}

fn compile_body(
    code: &mut Vec<Instruction>,
    seen_vars: &mut HashSet<Variable>,
    vars: &mut HashMap<Variable, usize>,
    permanent: &HashMap<Variable, usize>,
    s: &Structure,
) {
    for (i, t) in s.1.iter().enumerate() {
        match *t {
            Term::Anonymous => unimplemented!(),
            Term::Structure(Structure(a, ref ts)) => unimplemented!(),
            Term::Variable(v) => {
                let seen = !seen_vars.insert(v);
                if let Some(&loc) = permanent.get(&v) {
                    let loc = Location::Local(loc);
                    code.push(if seen {
                        Instruction::PutValue(loc, i)
                    } else {
                        Instruction::PutVariable(loc, i)
                    });
                } else {
                    unimplemented!()
                }
            }
        }
    }
    code.push(Instruction::Call(s.functor()));
}

fn find_variables(
    head: Option<&Structure>,
    body: &[Structure],
) -> HashMap<Variable, usize> {
    fn add_structure_variables(
        vars: &mut HashMap<Variable, usize>,
        s: &Structure,
    ) {
        let mut set = HashSet::new();
        for t in &s.1 {
            find_term_variables(&mut set, t);
        }
        for var in set {
            *vars.entry(var).or_insert(0) += 1;
        }
    }

    fn find_term_variables(vars: &mut HashSet<Variable>, t: &Term) {
        match *t {
            Term::Anonymous => {}
            Term::Variable(var) => {
                vars.insert(var);
            }
            Term::Structure(Structure(_, ref ts)) => for t in ts {
                find_term_variables(vars, t);
            },
        }
    }

    let mut vars = HashMap::new();
    if let Some(head) = head {
        let mut set = HashSet::new();
        for t in &head.1 {
            find_term_variables(&mut set, t);
        }
        for t in &body[0].1 {
            find_term_variables(&mut set, t);
        }
        for var in set {
            *vars.entry(var).or_insert(0) += 1;
        }
        for s in &body[1..] {
            add_structure_variables(&mut vars, s);
        }
    } else {
        for s in body {
            add_structure_variables(&mut vars, s);
        }
    }
    vars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn figure_3_1() {
        let code = compile(
            Some(&Structure(
                atom!(p),
                vec![
                    Term::Variable(variable!("X")),
                    Term::Variable(variable!("Y")),
                ],
            )),
            &[
                Structure(
                    atom!(q),
                    vec![
                        Term::Variable(variable!("X")),
                        Term::Variable(variable!("Z")),
                    ],
                ),
                Structure(
                    atom!(r),
                    vec![
                        Term::Variable(variable!("Z")),
                        Term::Variable(variable!("Y")),
                    ],
                ),
            ],
        );
        assert_eq!(
            code,
            (
                vec![
                    Instruction::Allocate(2),
                    Instruction::GetVariable(Location::Register(2), 0),
                    Instruction::GetVariable(Location::Local(0), 1),
                    Instruction::PutValue(Location::Register(2), 0),
                    Instruction::PutVariable(Location::Local(1), 1),
                    Instruction::Call(functor!(q / 2)),
                    Instruction::PutValue(Location::Local(1), 0),
                    Instruction::PutValue(Location::Local(0), 1),
                    Instruction::Call(functor!(r / 2)),
                    Instruction::Deallocate,
                ],
                vec![variable!("Y"), variable!("Z")]
            )
        )
    }
}
