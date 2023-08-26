pub mod ast;
pub mod compiler;
pub mod frame;
pub mod error;

lalrpop_mod!(#[allow(clippy::all)] #[allow(dead_code)] pub epilang); // synthesized by LALRPOP
