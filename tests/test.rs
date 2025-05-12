use pandora::{Lexer, ProgramManager, ProgramParser};

fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn test_one() {
    let mut source = std::fs::read_to_string("tests/test1.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();
    let mut manager = ProgramManager::new();

    let result = parser.parse(&mut manager, lexer);

    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_two() {
    let mut source = std::fs::read_to_string("tests/test2.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();

    let mut manager = ProgramManager::new();

    let result = parser.parse(&mut manager, lexer);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_three() {
    let mut source = std::fs::read_to_string("tests/test3.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();

    let mut manager = ProgramManager::new();

    let result = parser.parse(&mut manager, lexer);
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_four() {
    let mut source = std::fs::read_to_string("tests/test4.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();
    let mut manager = ProgramManager::new();

    let result = parser.parse(&mut manager, lexer);

    assert_eq!(result.is_err(), true);
}

#[test]
fn test_five() {
    let mut source = std::fs::read_to_string("tests/test5.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();

    let mut manager = ProgramManager::new();

    let result = parser.parse(&mut manager, lexer);
    assert_eq!(result.is_ok(), true);
}
