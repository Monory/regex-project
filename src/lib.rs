#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate maplit;
extern crate fnv;

pub mod ast;
pub mod dfa;
pub mod nfa;

pub use ast::{Token, AST};
pub use dfa::DFA;
pub use nfa::{Transition, NFA};

mod errors {
    error_chain!{}
}

use errors::*;

type State = i32;

pub trait Automaton {
    fn run(&self, s: &str) -> bool;
    fn write_graphviz(&self, filename: &str) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
//    use test::Bencher;

//    #[bench]
//    fn bench_nfa_exponential(b: &mut Bencher) {
//        let ast = AST::new(
//            Token::Concat,
//            Some(vec![
//                AST::new(
//                    Token::Concat,
//                    Some(vec![
//                        AST::new(Token::Literal('x'), None),
//                        AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                        AST::new(Token::Literal('x'), None),
//                        AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                    ]),
//                ),
//                AST::new(
//                    Token::Star,
//                    Some(vec![AST::new(
//                        Token::Concat,
//                        Some(vec![
//                            AST::new(Token::Literal('x'), None),
//                            AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                            AST::new(Token::Literal('x'), None),
//                            AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                        ]),
//                    )]),
//                ),
//                AST::new(Token::Literal('y'), None),
//            ]),
//        );
//
//        let nfa = ast.into_nfa();
//
//        b.iter(|| {
//            nfa.run_backtracking("xxxxxxxxxxxxxxx");
//        });
//    }
//
//    #[bench]
//    fn bench_nfa_thompson(b: &mut Bencher) {
//        let ast = AST::new(
//            Token::Concat,
//            Some(vec![
//                AST::new(
//                    Token::Concat,
//                    Some(vec![
//                        AST::new(Token::Literal('x'), None),
//                        AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                        AST::new(Token::Literal('x'), None),
//                        AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                    ]),
//                ),
//                AST::new(
//                    Token::Star,
//                    Some(vec![AST::new(
//                        Token::Concat,
//                        Some(vec![
//                            AST::new(Token::Literal('x'), None),
//                            AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                            AST::new(Token::Literal('x'), None),
//                            AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                        ]),
//                    )]),
//                ),
//                AST::new(Token::Literal('y'), None),
//            ]),
//        );
//
//        let nfa = ast.into_nfa();
//
//        b.iter(|| {
//            nfa.run("xxxxxxxxxxxxxxx");
//        });
//    }
//
//    #[bench]
//    fn bench_dfa_failure(b: &mut Bencher) {
//        let ast = AST::new(
//            Token::Concat,
//            Some(vec![
//                AST::new(
//                    Token::Concat,
//                    Some(vec![
//                        AST::new(Token::Literal('x'), None),
//                        AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                        AST::new(Token::Literal('x'), None),
//                        AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                    ]),
//                ),
//                AST::new(
//                    Token::Star,
//                    Some(vec![AST::new(
//                        Token::Concat,
//                        Some(vec![
//                            AST::new(Token::Literal('x'), None),
//                            AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                            AST::new(Token::Literal('x'), None),
//                            AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
//                        ]),
//                    )]),
//                ),
//                AST::new(Token::Literal('y'), None),
//            ]),
//        );
//
//        let nfa = ast.into_nfa();
//
//        let dfa = nfa.to_dfa();
//
//        b.iter(|| {
//            dfa.run("xxxxxxxxxxxxxxx");
//        });
//    }
}
