pub mod expression;
pub mod value;

lalrpop_mod!(#[allow(clippy::all)] #[allow(dead_code)] pub lr_lang); // synthesized by LALRPOP

#[cfg(test)]
pub mod test {
    use crate::ast::lr_lang;
    use crate::ast::expression::Expr;
    use std::fs;
    use std::path::Path;

    const TEST_PROGRAMS_DIR: &str = env!("CARGO_MANIFEST_DIR");

    pub fn get_test_program(folder: &str, file_name: &str) -> Box<Expr> {
        let path = Path::new(TEST_PROGRAMS_DIR)
            .join("test_programs")
            .join(folder)
            .join(file_name);
        let program_text = fs::read_to_string(path).expect("Unable to read the program file");
        lr_lang::ExprParser::new()
            .parse(&program_text)
            .expect("Unable to parse the program file")
    }
}
