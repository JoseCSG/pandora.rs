#![allow(warnings)]

mod compiler;
mod utils;

use compiler::lexer::Lexer;
use compiler::program_manager::ProgramManager;
use compiler::semantic_tables::FunctionTable;
use grammar::ProgramParser;
use lalrpop_util::lalrpop_mod;
use std::collections::HashMap;
use utils::stack::Stack;

lalrpop_mod!(pub grammar);

fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn main() {
    let mut source = std::fs::read_to_string("tests/test1.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();

    let mut program_manager = ProgramManager::new();

    parser.parse(&mut program_manager, lexer);
    program_manager.quadruplets.print_elements();
    program_manager.run_program();
}
