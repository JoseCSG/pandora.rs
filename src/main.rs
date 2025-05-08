#![allow(warnings)]

mod lexer;
mod semantic_cube;
mod semantic_tables;
mod tokens;
mod utils;

use grammar::ProgramParser;
use lalrpop_util::lalrpop_mod;
use lexer::Lexer;
use semantic_tables::FunctionTable;
use std::collections::HashMap;
use utils::stack::Stack;

lalrpop_mod!(pub grammar);

fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn main() {
    let mut source = std::fs::read_to_string("tests/test3.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();

    let mut scope_stack: Stack<String> = Stack::new();
    let cubo = semantic_cube::CuboSemantico::new();
    let mut function_table: FunctionTable = HashMap::new();

    let result = parser.parse(&cubo, &mut function_table, &mut scope_stack, lexer);
    print!("{:?}", result.unwrap());
}
