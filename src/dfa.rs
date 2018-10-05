use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use {Automaton, State};

mod errors {
    error_chain!{}
}

use errors::*;

#[derive(Debug)]
pub struct DFA {
    start: State,
    accept: BTreeSet<State>,
    transitions: BTreeMap<(State, char), State>,
}

impl DFA {
    pub fn new(
        start: State,
        accept: BTreeSet<State>,
        transitions: BTreeMap<(State, char), State>,
    ) -> DFA {
        DFA {
            start,
            accept,
            transitions,
        }
    }
}

impl Automaton for DFA {
    fn run(&self, s: &str) -> bool {
        let mut state = self.start;
        for c in s.chars() {
            if let Some(new_state) = self.transitions.get(&(state, c)) {
                state = *new_state;
            } else {
                return false;
            }
        }

        self.accept.contains(&state)
    }

    fn write_graphviz(&self, filename: &str) -> Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path).chain_err(|| "unable to create file")?;

        writeln!(&mut file, "digraph dfa {{").chain_err(|| "")?;
        writeln!(&mut file, "\trankdir=LR;").chain_err(|| "")?;
        write!(&mut file, "\tnode [shape = doublecircle]; ").chain_err(|| "")?;
        for state in &self.accept {
            write!(&mut file, "{} ", state).chain_err(|| "")?;
        }
        writeln!(&mut file, ";\n\tnode [shape = circle];").chain_err(|| "")?;
        for (t, s) in &self.transitions {
            writeln!(&mut file, "\t{} -> {} [ label = \"{}\"]", t.0, s, t.1).chain_err(|| "")?;
        }

        writeln!(&mut file, "}}").chain_err(|| "")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_dfa() {
        let dfa = DFA::new(
            0,
            btreeset!(1),
            btreemap!(
                (0, 'a') => 1,
                (1, 'a') => 1,
            ),
        );

        assert!(dfa.run("a"));
        assert!(dfa.run("aa"));
        assert!(!dfa.run(""));
        assert!(!dfa.run("ab"));
    }

    #[ignore]
    #[test]
    fn graphviz() -> Result<()> {
        let dfa = DFA::new(
            0,
            btreeset!(4),
            btreemap!(
                (0, 'a') => 1,
                (0, 'b') => 2,
                (1, 'a') => 3,
                (1, 'b') => 4,
                (2, 'a') => 3,
                (2, 'b') => 4,
                (3, 'a') => 3,
                (3, 'b') => 4
            ),
        );

        dfa.write_graphviz("graphs/dfa.dot")
    }
}
