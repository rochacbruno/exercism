use std::collections::HashSet;

fn sorted(input: &str) -> String {
    let mut sorted_input: Vec<char> = input.to_lowercase().chars().collect();
    sorted_input.sort();
    sorted_input.iter().collect()
}


pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
   possible_anagrams
    .iter()
    .filter(|candidate| word.to_lowercase() != candidate.to_lowercase())
    .filter(|candidate| sorted(candidate) == sorted(word))
    .copied()
    .collect::<HashSet<&'a str>>()
}