use common::FlatTerm;

use super::control::Instruction;

/// Compiles a "program" (a term to unify against) into instructions.
pub fn compile_program(flat: &FlatTerm) -> Vec<Instruction> {
    unimplemented!("compile {:#?}", flat)
}

#[cfg(test)]
mod tests {
    //use common::FlatTerm;
    use super::*;
    use test_utils::example_term;

    #[test]
    fn compiles_example_term() {
        let program = FlatTerm::flatten_term(example_term());
        let code = compile_program(&program);

        // assert_eq!(code, vec![]);
        unimplemented!("{:#?}", code)
    }
}
