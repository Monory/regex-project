use ast::Token::*;
use {State, Transition, NFA};

mod errors {
    error_chain!{}
}

use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct AST {
    token: Token,
    children: Option<Vec<AST>>,
}

#[derive(Clone)]
pub enum Token {
    Concat,
    Or,
    Star,
    Literal(char),
    Epsilon,
}

impl AST {
    pub fn new(token: Token, children: Option<Vec<AST>>) -> AST {
        AST { token, children }
    }

    pub fn into_nfa(self) -> NFA {
        match self.token {
            Concat => AST::concat_nfa(
                self.children
                    .unwrap()
                    .into_iter()
                    .map(|x| x.into_nfa())
                    .collect(),
            ),
            Or => AST::or_nfa(
                self.children
                    .unwrap()
                    .into_iter()
                    .map(|x| x.into_nfa())
                    .collect(),
            ),
            Star => AST::star_nfa(
                self.children
                    .unwrap()
                    .into_iter()
                    .nth(0)
                    .unwrap()
                    .into_nfa(),
            ),
            Literal(c) => AST::literal_nfa(c),
            Epsilon => AST::epsilon_nfa(),
        }
    }

    fn concat_nfa(nfas: Vec<NFA>) -> NFA {
        nfas.into_iter().fold(NFA::default(), |mut acc, mut x| {
            let max_state = *acc.states().iter().max().unwrap();
            AST::increase_nfa_states(&mut x, max_state + 1);

            for ((s, t), ns) in x.transitions {
                if s == x.start {
                    for f in &acc.accept {
                        acc.transitions
                            .entry((*f, t))
                            .or_insert(btreeset!())
                            .extend(&ns);
                    }
                } else {
                    acc.transitions.insert((s, t), ns);
                }
            }

            acc.accept = x.accept;

            acc
        })
    }

    fn or_nfa(mut nfas: Vec<NFA>) -> NFA {
        let mut max_state = -1;
        let mut old_starts = BTreeSet::new();
        let mut old_accepts: BTreeSet<State> = BTreeSet::new();

        for nfa in &mut nfas {
            AST::increase_nfa_states(nfa, max_state + 1);
            old_starts.insert(nfa.start);
            old_accepts.extend(nfa.accept.iter());
            max_state = *nfa.states().iter().max().unwrap();
        }

        let start = max_state + 1;
        let accept = max_state + 2;
        let mut transitions = BTreeMap::new();

        for nfa in nfas {
            transitions.extend(nfa.transitions);
        }

        transitions.insert((start, Transition::Epsilon), old_starts);
        for a in old_accepts {
            transitions
                .entry((a, Transition::Epsilon))
                .or_insert(btreeset!())
                .insert(accept);
        }

        NFA::new(start, btreeset!(accept), transitions)
    }

    fn star_nfa(mut nfa: NFA) -> NFA {
        let max_state = *nfa.states().iter().max().unwrap();

        let new_start = max_state + 1;
        let new_accept = max_state + 2;

        nfa.transitions.insert(
            (new_start, Transition::Epsilon),
            btreeset!(nfa.start, new_accept),
        );
        for s in &nfa.accept {
            nfa.transitions
                .entry((*s, Transition::Epsilon))
                .or_insert(btreeset!())
                .extend(btreeset!(nfa.start, new_accept));
        }

        nfa.start = new_start;
        nfa.accept = btreeset!(new_accept);

        nfa
    }

    fn literal_nfa(c: char) -> NFA {
        NFA::new(
            0,
            btreeset!(1),
            btreemap!(
                (0, Transition::Character(c)) => btreeset!(1)
            ),
        )
    }

    fn epsilon_nfa() -> NFA {
        NFA::new(
            0,
            btreeset!(1),
            btreemap!(
                (0, Transition::Epsilon) => btreeset!(1)
            ),
        )
    }

    /// In order to use two different NFAs together, we can increase ids of states of one of them,
    /// so that all of states of first NFA will be lower than some N, and of the second will be
    /// higher than that N.
    ///
    /// This way they will coexist without conflicts, and additional glue can be added easily.
    fn increase_nfa_states(nfa: &mut NFA, n: State) {
        nfa.start += n;
        nfa.accept = nfa.accept.iter().map(|x| *x + n).collect();

        let mut new_transitions = BTreeMap::new();
        for ((k, t), v) in &nfa.transitions {
            let new_value = v.iter().map(|x| x + n).collect();
            new_transitions.insert((k + n, *t), new_value);
        }
        nfa.transitions = new_transitions;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_nfa() {
        let nfa = AST::literal_nfa('a');

        assert_eq!(
            nfa,
            NFA::new(
                0,
                btreeset!(1),
                btreemap!((0, Transition::Character('a')) => btreeset ! (1)),
            )
        );
    }

    #[test]
    fn increase_nfa_states() {
        let mut nfa = AST::literal_nfa('a');
        AST::increase_nfa_states(&mut nfa, 5);

        assert_eq!(
            nfa,
            NFA::new(
                5,
                btreeset!(6),
                btreemap!((5, Transition::Character('a')) => btreeset ! (6)),
            )
        );
    }

    #[test]
    fn star_nfa() {
        let nfa = NFA::new(
            0,
            btreeset!(1),
            btreemap!((0, Transition::Character('a')) => btreeset! (1)),
        );

        assert_eq!(
            AST::star_nfa(nfa),
            NFA::new(
                2,
                btreeset!(3),
                btreemap!(
                    (0, Transition::Character('a')) => btreeset ! (1),
                    (1, Transition::Epsilon) => btreeset !(0, 3),
                    (2, Transition::Epsilon) => btreeset ! (0, 3),
                ),
            )
        );
    }

    #[test]
    fn or_nfa() {
        let mut nfas = Vec::new();

        nfas.push(NFA::new(
            0,
            btreeset!(1),
            btreemap!((0, Transition::Character('a')) => btreeset ! (1)),
        ));

        nfas.push(NFA::new(
            0,
            btreeset!(1),
            btreemap!((0, Transition::Character('b')) => btreeset ! (1)),
        ));

        assert_eq!(
            AST::or_nfa(nfas),
            NFA::new(
                4,
                btreeset!(5),
                btreemap!(
                    (0, Transition::Character('a')) => btreeset ! (1),
                    (1, Transition::Epsilon) => btreeset ! (5),
                    (2, Transition::Character('b')) => btreeset ! (3),
                    (3, Transition::Epsilon) => btreeset ! (5),
                    (4, Transition::Epsilon) => btreeset ! (0, 2),
                ),
            )
        );
    }

    #[test]
    fn concat_nfa() {
        assert_eq!(
            NFA::new(
                0,
                btreeset!(4),
                btreemap!(
                    (0, Transition::Character('a')) => btreeset!(2),
                    (2, Transition::Character('b')) => btreeset!(4),
                ),
            ),
            AST::concat_nfa(vec![AST::literal_nfa('a'), AST::literal_nfa('b')])
        );
    }

    #[test]
    fn to_nfa() {
        let _ast = AST::new(
            Or,
            Some(vec![
                AST::new(Literal('a'), None),
                AST::new(Literal('b'), None),
            ]),
        );
    }
}
