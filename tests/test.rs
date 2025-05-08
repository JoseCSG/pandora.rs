use pandora::{semantic_cube, semantic_tables, Lexer, ProgramParser, Stack};
use semantic_tables::FunctionTable;
use std::collections::HashMap;

fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn test_one() {
    let mut source = std::fs::read_to_string("tests/test1.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();
    let mut scope_stack: Stack<String> = Stack::new();
    let cubo = semantic_cube::CuboSemantico::new();
    let mut function_table: FunctionTable = HashMap::new();

    let result = parser.parse(&cubo, &mut function_table, &mut scope_stack, lexer);

    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_two() {
    let mut source = std::fs::read_to_string("tests/test2.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();

    let mut scope_stack: Stack<String> = Stack::new();
    let cubo = semantic_cube::CuboSemantico::new();
    let mut function_table: FunctionTable = HashMap::new();

    let result = parser.parse(&cubo, &mut function_table, &mut scope_stack, lexer);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_three() {
    let mut source = std::fs::read_to_string("tests/test3.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();

    let mut scope_stack: Stack<String> = Stack::new();
    let cubo = semantic_cube::CuboSemantico::new();
    let mut function_table: FunctionTable = HashMap::new();

    let result = parser.parse(&cubo, &mut function_table, &mut scope_stack, lexer);

    assert_eq!(result.is_err(), true);
}

#[test]
fn test_four() {
    let mut source = std::fs::read_to_string("tests/test4.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();
    let mut scope_stack: Stack<String> = Stack::new();
    let cubo = semantic_cube::CuboSemantico::new();
    let mut function_table: FunctionTable = HashMap::new();

    let result = parser.parse(&cubo, &mut function_table, &mut scope_stack, lexer);

    assert_eq!(result.is_err(), true);
}

#[test]
fn test_five() {
    let mut source = std::fs::read_to_string("tests/test5.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();

    let mut scope_stack: Stack<String> = Stack::new();
    let cubo = semantic_cube::CuboSemantico::new();
    let mut function_table: FunctionTable = HashMap::new();

    let result = parser.parse(&cubo, &mut function_table, &mut scope_stack, lexer);

    assert_eq!(result.is_ok(), true);
}
