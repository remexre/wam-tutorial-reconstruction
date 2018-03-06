use std::collections::{HashMap, HashSet};

use common::{Atom, Structure, Term, Variable};

use super::super::Instruction;

/// A type for flattened terms.
#[derive(Clone, Debug, Eq, PartialEq)]
enum FlatTerm {
    Functor(Atom, Vec<usize>),
    Ref(usize),
    Variable,
}

/// Compiles a query or rule into a series of instructions and a mapping
/// between named variables and the registers the variables are stored in. If
/// the head is `None`, compile as a query. Otherwise, compiles as a rule.
///
/// Note that if compiled as a query, all variables are marked as permanent.
pub fn compile(
    head: Option<&Structure>,
    body: &[Structure],
) -> (Vec<Instruction>, HashMap<Variable, usize>) {
    let vars = find_variables(head, body);
    let permanent: Vec<Variable> = if let Some(head) = head {
        vars.into_iter()
            .filter(|&(_, n)| n > 1)
            .map(|(v, _)| v)
            .collect()
    } else {
        vars.into_iter().map(|p| p.0).collect()
    };

    let mut code = vec![Instruction::Allocate(permanent.len())];
    let mut vars = HashMap::new();

    if let Some(head) = head {
        compile_head(&mut code, &mut vars, &permanent, &head);
    }
    for s in body {
        compile_body(&mut code, &mut vars, &permanent, s);
    }

    code.push(Instruction::Deallocate);
    (code, vars)
}

fn compile_head(
    code: &mut Vec<Instruction>,
    vars: &mut HashMap<Variable, usize>,
    permanent: &[Variable],
    head: &Structure,
) {
    unimplemented!(
        "compile_head\n\t{:?}\n\t{:?}\n\t{:?}\n\t{:?}",
        code,
        vars,
        permanent,
        head
    )
}

fn compile_body(
    code: &mut Vec<Instruction>,
    vars: &mut HashMap<Variable, usize>,
    permanent: &[Variable],
    s: &Structure,
) {
    unimplemented!(
        "compile_body\n\t{:?}\n\t{:?}\n\t{:?}\n\t{:?}",
        code,
        vars,
        permanent,
        s
    );
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
