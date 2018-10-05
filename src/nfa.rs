use fnv::FnvHashSet;
use nfa::Transition::{Character, Epsilon};
use std::collections::btree_map::Entry::Vacant;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use State;
use {Automaton, DFA};

mod errors {
    error_chain!{}
}

use errors::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NFA {
    pub start: State,
    pub accept: BTreeSet<State>,
    pub transitions: BTreeMap<(State, Transition), BTreeSet<State>>,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, PartialOrd, Ord)]
pub enum Transition {
    Epsilon,
    Character(char),
}

impl Default for NFA {
    fn default() -> NFA {
        NFA::new(0, btreeset!(0), btreemap!())
    }
}

impl NFA {
    pub fn new(
        start: State,
        accept: BTreeSet<State>,
        transitions: BTreeMap<(State, Transition), BTreeSet<State>>,
    ) -> NFA {
        NFA {
            start,
            accept,
            transitions,
        }
    }

    pub fn states(&self) -> BTreeSet<State> {
        let mut result = BTreeSet::new();

        result.insert(self.start);
        result.extend(self.accept.iter());
        result.extend(self.transitions.iter().map(|(k, _)| k.0));
        for v in self.transitions.values() {
            result.extend(v.iter());
        }

        result
    }

    fn alphabet(&self) -> BTreeSet<char> {
        let mut result = BTreeSet::new();

        for (_, t) in self.transitions.keys() {
            if let Character(c) = *t {
                result.insert(c);
            };
        }

        result
    }

    fn reachable_states(&self, states: &BTreeSet<State>, input: Transition) -> BTreeSet<State> {
        let mut result = BTreeSet::new();
        for s in states {
            if let Some(next_states) = self.transitions.get(&(*s, input)) {
                for ns in next_states {
                    result.insert(ns.clone());
                }
            }
        }

        result
    }

    fn set_epsilon_closure(&self, states: &BTreeSet<State>) -> BTreeSet<State> {
        let mut states = states.clone();
        loop {
            let new_states = self.reachable_states(&states, Epsilon);
            let old_len = states.len();
            states.extend(new_states);
            if old_len == states.len() {
                break;
            }
        }
        states
    }

    fn epsilon_closure(&self, state: State) -> BTreeSet<State> {
        let mut done = BTreeSet::new();
        let mut not_done = BTreeSet::new();
        not_done.insert(state);

        while !not_done.is_empty() {
            let mut to_do = BTreeSet::new();
            for s in &not_done {
                if let Some(epsilon_from_s) = self.transitions.get(&(*s, Epsilon)) {
                    for new_state in epsilon_from_s.iter() {
                        if !done.contains(new_state) {
                            to_do.insert(*new_state);
                        }
                    }
                }
                done.insert(*s);
            }
            not_done = to_do;
        }

        done
    }

    fn find_accept_state(&self, states: &BTreeSet<State>) -> Option<State> {
        states.intersection(&self.accept).cloned().nth(0)
    }

    pub fn to_dfa(&self) -> DFA {
        let alphabet = self.alphabet();

        let mut states = BTreeMap::new();
        let mut accept = BTreeSet::new();
        let mut transitions = BTreeMap::new();

        let mut id = 0;
        let mut get_id = || {
            id += 1;
            id - 1
        };
        let mut queue = VecDeque::new();

        let init_state = self.epsilon_closure(self.start);
        queue.push_back((get_id(), init_state.clone()));
        states.insert(init_state.into_iter().collect(), 0);
        while let Some((cur_id, cur_state)) = queue.pop_front() {
            for a in &alphabet {
                let mut new_state = self.reachable_states(&cur_state, Character(*a));
                new_state = self.set_epsilon_closure(&new_state);

                if !new_state.is_empty() {
                    if let Vacant(entry) = states.entry(new_state.clone()) {
                        let id = get_id();
                        if self.find_accept_state(&new_state).is_some() {
                            accept.insert(id);
                        }
                        queue.push_back((id, new_state.clone()));
                        entry.insert(id);
                    }

                    let id = &states[&new_state];
                    transitions.insert((cur_id, *a), *id);
                }
            }
        }

        DFA::new(0, accept, transitions)
    }

    pub fn run_backtracking(&self, s: &str) -> bool {
        let s: Vec<char> = s.chars().collect();

        let mut queue = VecDeque::new();
        queue.push_back((self.start, 0));

        while let Some((state, pos)) = queue.pop_front() {
            if let Some(set) = self.transitions.get(&(state, Epsilon)) {
                for item in set {
                    queue.push_back((*item, pos));
                }
            }

            if pos == s.len() {
                if self.accept.contains(&state) {
                    return true;
                }
            } else if let Some(set) = self.transitions.get(&(state, Character(s[pos]))) {
                for item in set {
                    queue.push_back((*item, pos + 1));
                }
            }
        }
        false
    }

    fn epsilon_closure_thompson(&self, state: State) -> FnvHashSet<State> {
        let mut done = FnvHashSet::default();
        let mut not_done = FnvHashSet::default();
        not_done.insert(state);

        while !not_done.is_empty() {
            let mut to_do = FnvHashSet::default();
            for s in &not_done {
                if let Some(epsilon_from_s) = self.transitions.get(&(*s, Epsilon)) {
                    for new_state in epsilon_from_s.iter() {
                        if !done.contains(new_state) {
                            to_do.insert(*new_state);
                        }
                    }
                }
                done.insert(*s);
            }
            not_done = to_do;
        }

        done
    }

    fn step_thompson(&self, clist: &FnvHashSet<State>, c: char) -> FnvHashSet<State> {
        let mut result = FnvHashSet::default();

        for s in clist.iter() {
            if let Some(ns) = self.transitions.get(&(*s, Character(c))) {
                for n in ns {
                    result.extend(self.epsilon_closure_thompson(*n));
                }
            }
        }

        result
    }
}

impl Automaton for NFA {
    fn run(&self, s: &str) -> bool {
        let mut clist: FnvHashSet<State> = FnvHashSet::default();

        clist.insert(self.start);

        for c in s.chars() {
            //            println!("{}", clist.len());
            clist = self.step_thompson(&clist, c);
        }

        //        println!("{:?}", clist);
        let accept: FnvHashSet<_> = self.accept.iter().cloned().collect();
        clist.intersection(&accept).count() != 0
    }

    fn write_graphviz(&self, filename: &str) -> Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path).chain_err(|| "unable to create file")?;

        writeln!(&mut file, "digraph nfa {{").chain_err(|| "")?;
        writeln!(&mut file, "\trankdir=LR;").chain_err(|| "")?;
        write!(&mut file, "\tnode [shape = doublecircle]; ").chain_err(|| "")?;
        for state in &self.accept {
            write!(&mut file, "{} ", state).chain_err(|| "")?;
        }
        writeln!(&mut file, ";\n\tnode [shape = circle];").chain_err(|| "")?;
        for (t, states) in &self.transitions {
            for s in states {
                let label = match t.1 {
                    Epsilon => "ε".to_string(),
                    Character(c) => c.to_string(),
                };

                writeln!(&mut file, "\t{} -> {} [ label = \"{}\"]", t.0, s, label)
                    .chain_err(|| "")?;
            }
        }

        writeln!(&mut file, "}}").chain_err(|| "")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_nfa() {
        let nfa = NFA::new(
            0,
            btreeset!(1),
            btreemap!(
                (0, Character('a')) => btreeset!(0, 1),
                (1, Character('a')) => btreeset!(1),
            ),
        );

        assert!(nfa.run_backtracking("a"));
        assert!(nfa.run_backtracking("aa"));
        assert!(!nfa.run_backtracking(""));
        assert!(!nfa.run_backtracking("ab"));
    }

    #[test]
    fn epsilons() {
        let nfa = NFA::new(
            0,
            btreeset!(1),
            btreemap!(
                (0, Character('a')) => btreeset!(0),
                (0, Epsilon) => btreeset!(1),
            ),
        );

        assert!(nfa.run_backtracking("a"));
        assert!(nfa.run_backtracking("aa"));
        assert!(nfa.run_backtracking(""));
        assert!(!nfa.run_backtracking("b"));
    }

    #[test]
    fn states() {
        let nfa = NFA::new(
            0,
            btreeset!(3),
            btreemap!(
                (0, Epsilon) => btreeset!(1, 2),
                (1, Epsilon) => btreeset!(3),
                (2, Character('a')) => btreeset!(3),
            ),
        );

        assert_eq!(nfa.states(), btreeset!(0, 1, 2, 3));
    }

    #[ignore]
    #[test]
    fn graphviz() -> Result<()> {
        let nfa = NFA::new(
            0,
            btreeset!(9),
            btreemap!(
                (0, Epsilon) => btreeset!(1, 3),
                (1, Character('a')) => btreeset!(2),
                (2, Epsilon) => btreeset!(5),
                (3, Character('b')) => btreeset!(4),
                (4, Epsilon) => btreeset!(5),
                (5, Epsilon) => btreeset!(6, 8),
                (6, Character('a')) => btreeset!(7),
                (7, Epsilon) => btreeset!(6, 8),
                (8, Character('b')) => btreeset!(9),
            ),
        );

        nfa.write_graphviz("graphs/nfa.dot")
    }

    #[test]
    fn epsilon_closure() {
        let nfa = NFA::new(
            0,
            btreeset!(3),
            btreemap!(
                (0, Epsilon) => btreeset!(1, 2),
                (1, Epsilon) => btreeset!(3),
                (2, Character('a')) => btreeset!(3),
            ),
        );

        assert_eq!(nfa.epsilon_closure(0), btreeset!(0, 1, 2, 3));
        assert_eq!(nfa.epsilon_closure(1), btreeset!(1, 3));
        assert_eq!(nfa.epsilon_closure(2), btreeset!(2));
        assert_eq!(nfa.epsilon_closure(3), btreeset!(3));
    }

    #[test]
    fn alphabet() {
        let nfa = NFA::new(
            0,
            btreeset!(3),
            btreemap!(
                (0, Epsilon) => btreeset!(1, 2),
                (1, Epsilon) => btreeset!(3),
                (2, Character('a')) => btreeset!(3),
            ),
        );

        assert_eq!(nfa.alphabet(), btreeset!('a'));
    }

    #[test]
    fn dfa() {
        let nfa = NFA::new(
            0,
            btreeset!(9),
            btreemap!(
                (0, Epsilon) => btreeset!(1, 3),
                (1, Character('a')) => btreeset!(2),
                (2, Epsilon) => btreeset!(5),
                (3, Character('b')) => btreeset!(4),
                (4, Epsilon) => btreeset!(5),
                (5, Epsilon) => btreeset!(6, 8),
                (6, Character('a')) => btreeset!(7),
                (7, Epsilon) => btreeset!(6, 8),
                (8, Character('b')) => btreeset!(9),
            ),
        );

        let dfa = nfa.to_dfa();

        assert!(!dfa.run("baa"));
        assert!(dfa.run("baab"));
    }

    #[test]
    fn default() {
        assert_eq!(NFA::default(), NFA::new(0, btreeset!(0), btreemap!()));
    }
}
