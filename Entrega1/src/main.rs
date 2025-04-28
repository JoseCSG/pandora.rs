mod lexer;
mod tokens;
use grammar::ExprParser;
use lalrpop_util::lalrpop_mod;
use lexer::Lexer;

fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
pub fn lexer_test_one() {
    let source = r#"
        program main;
        var int 123 , float 12.123 , int naomi ;
        var int i = 0;
        while ( i < 3 ) {
            print(i)
        }
        
        end
    "#;
    let test_one = lex(source);
    assert_eq!(
        normalize(&test_one),
        normalize(
            "Program Main Semicolon
            Var IntDatatype Number Comma FloatDatatype Float Comma IntDatatype Id Semicolon
            Var IntDatatype Id Equal Number Semicolon
            While LParen Id LessThan Number RParen LBrace Id LParen Id RParen RBrace 
            End"
        )
    );
}

lalrpop_mod!(pub grammar);

fn main() {
    let source = r#"
        program main;
        var int 123 , float 12.123 , int naomi ;
        var int i = 0;
        while ( i < 3 ) {
            print(i)
        }
        
        end
    "#;

    let source2 = r#"
        3 + 5
    "#;

    let result = Lexer::lex(source);
    let lexer = Lexer::new(source2);
    let parser = ExprParser::new();
    let result2 = parser.parse(lexer);
    println!("{}", result);
    println!("{:?}", result2.unwrap());
}
