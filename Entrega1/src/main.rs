mod lexer;
mod tokens;
use grammar::ProgramParser;
use lalrpop_util::lalrpop_mod;
use lexer::Lexer;

lalrpop_mod!(pub grammar);

/* fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}
 */

fn main() {
    let source = r#"
        program prueba;
        main {
            while ( i < 3 ) do {
                print(i);
                i = i + 1;
            }        
        }
        end
    "#;

    let result = Lexer::lex(source);
    let lexer = Lexer::new(source);
    let parser = ProgramParser::new();
    let result2 = parser.parse(lexer);
    println!("{}", result);
    println!("{:?}", result2.unwrap());
}
