use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: & 'a [&str]) -> HashSet<&'a str> {
    let mut sorted_word = word.to_lowercase().chars().collect::<Vec<char>>();
    sorted_word.sort_unstable();

    let hits = possible_anagrams.iter()
    .filter(|s| s.to_lowercase() != word.to_lowercase())
    .filter(|s| {
        let mut sorted_anagram = s.to_lowercase().chars().collect::<Vec<char>>();
        sorted_anagram.sort_unstable();
        
        sorted_anagram == sorted_word
    })
    .cloned();

    HashSet::from_iter(hits)
}
