use std::collections::HashSet;

use common::{FlatTerm, FlatTermValue, Functor};

use super::control::Instruction;

/// Compiles a term into instructions that will construct the term on the
/// heap, storing the root into the given register number.
pub fn compile_query(flat: &FlatTerm, base: usize) -> Vec<Instruction> {
    let mut instrs = Vec::with_capacity(flat.0.len());
    let mut seen = HashSet::with_capacity(flat.0.len());

    fn compile(
        flattened: &[(usize, FlatTermValue)],
        instrs: &mut Vec<Instruction>,
        seen: &mut HashSet<usize>,
        base: usize,
        i: usize,
    ) -> usize {
        unimplemented!()
    }

    for i in 0..flat.0.len() {
        compile(&flat.0, &mut instrs, &mut seen, base, i);
    }
    assert_eq!(seen.len(), flat.0.len());
    instrs
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::example_term;

    #[test]
    fn compiles_example_term() {
        let query = FlatTerm::flatten_term(example_term());
        let code = compile_query(&query, 1);

        assert_eq!(
            code,
            vec![
                Instruction::PutStructure(Functor("h".into(), 2), 3),
                Instruction::SetVariable(2),
                Instruction::SetVariable(5),
                Instruction::PutStructure(Functor("f".into(), 1), 4),
                Instruction::SetVariable(5),
                Instruction::PutStructure(Functor("p".into(), 3), 1),
                Instruction::SetVariable(2),
                Instruction::SetVariable(3),
                Instruction::SetVariable(4),
            ]
        );
    }
}
