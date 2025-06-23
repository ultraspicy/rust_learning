use std::collections::HashMap;

fn main() {}

struct Solution {}

impl Solution {
    pub fn generate_tag(caption: String) -> String {
        let mut iter = caption
            .split_whitespace()
            .flat_map( |word| {
                let mut chars = word.chars();
                chars
                    .next()
                    .map(|first| first.to_uppercase().chain(chars.map(|char| char.to_ascii_lowercase())))
                    .into_iter()
                    .flatten()
            })
            .take(99);

        iter
            .next()
            .map(|char| char.to_lowercase().chain(iter))
            .into_iter()
            .flatten()
            .fold(String::from("#"), |mut acc, ch| {
                acc.push(ch);
                acc
            })
    }
}