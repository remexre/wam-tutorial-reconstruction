use common::FlatTerm;

use super::control::Instruction;

/// Compiles a "program" (a term to unify against) into instructions.
pub fn compile_program(flat: FlatTerm) -> Vec<Instruction> {
    unimplemented!("compile {:#?}", flat)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::example_term;

    #[test]
    fn compiles_example_term() {
        assert_eq!(
            compile_program(example_term().flatten()),
            vec![unimplemented!()]
        );
    }
}
