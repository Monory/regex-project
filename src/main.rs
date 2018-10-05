extern crate regexlib;

use regexlib::*;

pub fn main() {
    // (0|1)*1(0|1)^n
    let ast = AST::new(
        Token::Concat,
        Some(vec![
            AST::new(
                Token::Star,
                Some(vec![
                    AST::new(
                        Token::Or,
                        Some(vec![
                            AST::new(Token::Literal('0'), None),
                            AST::new(Token::Literal('1'), None)
                        ])
                    )
                ])
            ),
            AST::new(Token::Literal('1'), None),
            AST::new(
                Token::Or,
                Some(vec![
                    AST::new(Token::Literal('0'), None),
                    AST::new(Token::Literal('1'), None)
                ])
            ),
            AST::new(
                Token::Or,
                Some(vec![
                    AST::new(Token::Literal('0'), None),
                    AST::new(Token::Literal('1'), None)
                ])
            ),
            AST::new(
                Token::Or,
                Some(vec![
                    AST::new(Token::Literal('0'), None),
                    AST::new(Token::Literal('1'), None)
                ])
            ),
            AST::new(
                Token::Or,
                Some(vec![
                    AST::new(Token::Literal('0'), None),
                    AST::new(Token::Literal('1'), None)
                ])
            ),
        ])
    );

    let nfa = ast.into_nfa();
    nfa.write_graphviz("graphs/01nfa.dot");

    let dfa = nfa.to_dfa();
    dfa.write_graphviz("graphs/01dfa.dot");

    println!("Written!");
}