use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub type TermFreq = HashMap<String, usize>;
pub type TermFreqIndex = HashMap<PathBuf, TermFreq>;

pub fn tf(term: &str, document: &TermFreq) -> f32 {
    document.get(term).cloned().unwrap_or(0) as f32
        / document
            .iter()
            .map(|(_term, frequency)| *frequency)
            .sum::<usize>() as f32
}

pub fn idf(term: &str, document: &TermFreqIndex) -> f32 {
    (document.len() as f32
        / document
            .values()
            .filter(|tf| tf.contains_key(term))
            .count()
            .max(1) as f32)
        .log10()
}

pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..]
        }
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self.chop(n)
    }

    pub fn next_token(&mut self) -> Option<String> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()).iter().collect());
        }

        if self.content[0].is_alphabetic() {
            return Some(
                self.chop_while(|x| x.is_alphanumeric())
                    .iter()
                    .map(|x| x.to_ascii_uppercase())
                    .collect(),
            );
        }

        return Some(self.chop(1).iter().collect());
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

pub fn search_query<'a>(tf_index: &'a TermFreqIndex, query: &'a [char]) -> Vec<(&'a Path, f32)> {
    let mut result = Vec::<(&Path, f32)>::new();
    let token = Lexer::new(&query).collect::<Vec<_>>();
    for (path, tf_table) in tf_index {
        let mut rank = 0f32;
        for token in &token {
            rank += tf(&token, &tf_table) * idf(&token, &tf_index)
        }
        result.push((path, rank))
    }
    result.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
    result.reverse();
    result
}
