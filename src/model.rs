use std::collections::HashMap;
use std::collections::HashSet;

use counter::Counter;
use thiserror::Error;
use weighted_rand::{builder::NewBuilder, builder::WalkerTableBuilder, table::WalkerTable};

pub struct CharCounter {
    counts: HashMap<(Option<char>, Option<char>), Counter<Option<char>, u32>>,
    wordset: HashSet<String>,
}

impl CharCounter {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
            wordset: HashSet::new(),
        }
    }

    pub fn feed_word(&mut self, word: impl AsRef<str>) {
        let word = word.as_ref().to_string();

        let mut chars = vec![None, None];
        chars.extend(word.chars().map(|c| Some(c)));
        chars.push(None);

        for i in 0..chars.len() - 2 {
            self.increment((chars[i], chars[i + 1]), chars[i + 2]);
        }
        self.wordset.insert(word);
    }

    fn increment(&mut self, k: (Option<char>, Option<char>), v: Option<char>) {
        self.counts.entry(k).or_insert(Counter::new())[&v] += 1;
    }
}

/// WalkerTable only returns the index of the selected weight, so we also need to keep a vec of `Option<char>`s to select from.
struct CharChooser {
    chars: Vec<Option<char>>,
    chooser: WalkerTable,
}

impl CharChooser {
    fn new(counter: Counter<Option<char>, u32>) -> Self {
        let map = counter.into_map();
        let (chars, counts) = map.into_iter().unzip::<Option<char>, u32, _, Vec<_>>();
        let wt = WalkerTableBuilder::new(&counts).build();
        Self { chars, chooser: wt }
    }

    fn next(&self) -> Option<char> {
        self.chars[self.chooser.next()]
    }
}

/// A Markov chain, for characters. I'm here all week. Try the fish.
pub struct CharkovChain {
    chain: HashMap<(Option<char>, Option<char>), CharChooser>,
    words: HashSet<String>,
}

#[derive(Debug, Error)]
pub enum WordGenerationError {
    #[error("Exeeded maximum number of iterations (which is {0})")]
    IterationsExceeded(usize),
    #[error("Inconsistency in markov chain data")]
    InvalidMarkovChain,
}

impl CharkovChain {
    pub fn word(&self, min_len: usize, max_len: usize) -> Result<String, WordGenerationError> {
        for _ in 0..1000 {
            let candidate = self.candidate_word()?;
            if min_len <= candidate.len() && candidate.len() <= max_len && !self.words.contains(&candidate) {
                return Ok(candidate);
            }
        }
        Err(WordGenerationError::IterationsExceeded(1000))
    }

    fn candidate_word(&self) -> Result<String, WordGenerationError> {
        let mut result = String::new();
        let mut state = (None, None);
        loop {
            if let Some(c) = self
                .chain
                .get(&state)
                .ok_or(WordGenerationError::InvalidMarkovChain)?
                .next()
            {
                result.push(c);
                state = (state.1, Some(c));
            } else {
                break;
            }
        }
        Ok(result)
    }
}

impl From<CharCounter> for CharkovChain {
    fn from(counter: CharCounter) -> Self {
        Self {
            chain: counter
                .counts
                .into_iter()
                .map(|(k, v)| (k, CharChooser::new(v)))
                .collect(),
            words: counter.wordset,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{CharCounter, CharkovChain};

    #[test]
    fn test_feed_word() {
        let mut cc = CharCounter::new();
        cc.feed_word("a");

        assert_eq!(
            cc.counts.get(&(None, None)).unwrap().most_common_ordered(),
            vec![(Some('a'), 1)]
        );
        assert_eq!(
            cc.counts
                .get(&(None, Some('a')))
                .unwrap()
                .most_common_ordered(),
            vec![(None, 1)]
        );

        cc.feed_word("ab");
        assert_eq!(
            cc.counts.get(&(None, None)).unwrap().most_common_ordered(),
            vec![(Some('a'), 2)]
        );
        assert_eq!(
            cc.counts
                .get(&(None, Some('a')))
                .unwrap()
                .most_common_ordered(),
            vec![(None, 1), (Some('b'), 1)]
        );
        assert_eq!(
            cc.counts
                .get(&(Some('a'), Some('b')))
                .unwrap()
                .most_common_ordered(),
            vec![(None, 1)]
        );

        cc.feed_word("abc");
        assert_eq!(
            cc.counts.get(&(None, None)).unwrap().most_common_ordered(),
            vec![(Some('a'), 3)]
        );
        assert_eq!(
            cc.counts
                .get(&(None, Some('a')))
                .unwrap()
                .most_common_ordered(),
            vec![(Some('b'), 2), (None, 1)]
        );
        assert_eq!(
            cc.counts
                .get(&(Some('a'), Some('b')))
                .unwrap()
                .most_common_ordered(),
            vec![(None, 1), (Some('c'), 1)]
        );
        assert_eq!(
            cc.counts
                .get(&(Some('b'), Some('c')))
                .unwrap()
                .most_common_ordered(),
            vec![(None, 1)]
        );
    }

    #[test]
    fn test_word() {
        let mut cc = CharCounter::new();
        cc.feed_word("babel");
        cc.feed_word("table");
        let charkov: CharkovChain = cc.into();

        let possible_words: HashSet<&str> = vec!["babel", "table", "bable", "tabel"]
            .into_iter()
            .collect();

        let mut found = HashSet::new();
        for _ in 0..200 {
            let word = charkov.word().unwrap();
            assert!(possible_words.contains(&word as &str));
            found.insert(word);
        }
        let mut generated = found.into_iter().collect::<Vec<_>>();
        generated.sort();
        assert_eq!(generated, vec!["bable", "tabel"]); // Just make sure all possible words were generated. There's the possibility for this to fail!
    }
}
