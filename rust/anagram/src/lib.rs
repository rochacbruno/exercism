use std::collections::HashSet;

fn sorted(input: &str) -> Vec<char> {
    let mut sorted_input: Vec<char> = input.chars().collect();
    sorted_input.sort();
    sorted_input
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let prepared_word = word.to_lowercase();
    let sorted_word = sorted(&prepared_word);

    possible_anagrams
        .iter()
        .filter(|candidate| {
            let prepated_candidate = candidate.to_lowercase();
            prepated_candidate != prepared_word && sorted(&prepated_candidate) == sorted_word
        })
        .copied()
        .collect::<HashSet<&'a str>>()
}
