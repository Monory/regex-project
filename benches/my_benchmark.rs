#[macro_use]
extern crate criterion;
extern crate regexlib;

use criterion::Criterion;
use criterion::Fun;

use regexlib::*;

fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_slow(n-1) + fibonacci_slow(n-2),
    }
}

fn fibonacci_fast(n: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    let mut c = 0u64;

    if n == 0 {
        return 0
    }

    for _ in 0..(n+1) {
        c = a + b;
        a = b;
        b = c;
    }
    return b;
}

fn fibonaccis(c: &mut Criterion) {
    let fib_slow = Fun::new("Recursive", |b, i| b.iter(|| fibonacci_slow(*i)));
    let fib_fast = Fun::new("Iterative", |b, i| b.iter(|| fibonacci_fast(*i)));

    let functions = vec!(fib_slow, fib_fast);

    c.bench_functions("Fibonacci", functions, 20);
}

fn test_speed(c: &mut Criterion) {
    c.bench_function_over_inputs("Recursive inputs", |b, &length| {
        b.iter(|| fibonacci_slow(length));
    }, (1..=10));
}


fn bench_nfa_exponential(c: &mut Criterion) {
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

    c.bench_function_over_inputs(
        "Backtracking NFA",
        move |b, &input| {
            b.iter(|| nfa.run_backtracking(&input));
        },
        vec![
            "xxx",
            "xxxx",
            "xxxxx",
            "xxxxxx",
            "xxxxxxx",
            "xxxxxxxx",
            "xxxxxxxxx",
            "xxxxxxxxxx"
        ]
    );

//    b.iter(|| {
//        nfa.run_backtracking("xxxxxxxxxxxxxxx");
//    });
}


//fn criterion_benchmark(c: &mut Criterion) {
//    c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
//}

criterion_group!(benches, bench_nfa_exponential);
criterion_main!(benches);