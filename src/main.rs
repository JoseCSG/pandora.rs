mod lexer;
mod tokens;
use grammar::ProgramParser;
use lalrpop_util::lalrpop_mod;
use lexer::Lexer;

lalrpop_mod!(pub grammar);

fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn test_one() {
    let mut source = std::fs::read_to_string("src/tests/test1.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();
    let result = parser.parse(lexer);

    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), "Programa Valido");
}

#[test]
fn test_two() {
    let mut source = std::fs::read_to_string("src/tests/test2.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();
    let result = parser.parse(lexer);

    assert_eq!(result.is_err(), true);
}

#[test]
fn test_three() {
    let mut source = std::fs::read_to_string("src/tests/test3.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();
    let result = parser.parse(lexer);

    assert_eq!(result.is_err(), false);
}

#[test]
fn test_four() {
    let mut source = std::fs::read_to_string("src/tests/test4.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();
    let result = parser.parse(lexer);

    assert_eq!(result.is_err(), true);
}

#[test]
fn test_five() {
    let mut source = std::fs::read_to_string("src/tests/test5.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();
    let result = parser.parse(lexer);

    assert_eq!(result.is_ok(), true);
}

fn main() {
    let mut source = std::fs::read_to_string("src/tests/test5.pdra").expect("Unable to read file");
    source = normalize(&source);
    let lexer = Lexer::new(&source);
    let parser = ProgramParser::new();
    let result = parser.parse(lexer);
    print!("{}", result.unwrap());
}
