use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let word_sorted = sort_str(&word_lowercase);

    inputs
        .iter()
        .filter(|&input|{
            input.len() == word.len() &&
            word_lowercase != input.to_lowercase() &&
            word_sorted == sort_str(&input.to_lowercase())
        })
        .cloned()
        .collect()

}

pub fn sort_str(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars
}