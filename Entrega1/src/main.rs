mod lexer;
use lexer::lex;

#[test]
pub fn lexer_test_one() {
    assert!(grammar::TermParser::new().parse("22").is_ok());
}

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
    let res = lex(source);
    println!("{}", res);
}
