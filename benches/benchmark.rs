#[macro_use]
extern crate criterion;
extern crate regexlib;

use criterion::Criterion;
use criterion::Fun;
use regexlib::*;

fn bench_adversarial_backtracking(c: &mut Criterion) {
    // (x+)+y
    let ast = AST::new(
        Token::Concat,
        Some(vec![
            AST::new(
                Token::Concat,
                Some(vec![
                    AST::new(Token::Literal('x'), None),
                    AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
                    AST::new(Token::Literal('x'), None),
                    AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
                ]),
            ),
            AST::new(
                Token::Star,
                Some(vec![AST::new(
                    Token::Concat,
                    Some(vec![
                        AST::new(Token::Literal('x'), None),
                        AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
                        AST::new(Token::Literal('x'), None),
                        AST::new(Token::Star, Some(vec![AST::new(Token::Literal('x'), None)])),
                    ]),
                )]),
            ),
            AST::new(Token::Literal('y'), None),
        ]),
    );

    let nfa = ast.into_nfa();
    let nfa2 = nfa.clone();
    let dfa = nfa.to_dfa();

    let functions = vec!(
        Fun::new("Backtracking NFA", move |b, i| b.iter(|| nfa.run_backtracking(*i))),
        Fun::new("NFA", move |b, i| b.iter(|| nfa2.run(*i))),
        Fun::new("DFA", move |b, i| b.iter(|| dfa.run(*i))),
    );

    c.bench_functions("Adversarial", functions, "xxxxxxxxx");
}

criterion_group!(benches, bench_adversarial_backtracking);
criterion_main!(benches);