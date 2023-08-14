pub mod ast;
pub mod compiler;

lalrpop_mod!(#[allow(clippy::all)] #[allow(dead_code)] pub lr_lang); // synthesized by LALRPOP
