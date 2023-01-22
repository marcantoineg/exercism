// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    if magazine.len() < note.len() {
        return false;
    }

    let mut available_words = get_hash_map(magazine);
    let required_words = get_hash_map(note);

    return required_words.iter().all(|(word, required_count)| {
        match available_words.get(word) {
            Some(_) => {
                available_words
                    .entry(word)
                    .and_modify(|v| *v -= required_count);
                available_words[word] >= 0
            }
            _ => false,
        }
    });
}

fn get_hash_map<'a>(list_of_words: &'a [&str]) -> HashMap<&'a str, i32> {
    return list_of_words
        .iter()
        .fold(HashMap::new(), |mut words, word| {
            words.entry(*word).and_modify(|v| *v += 1).or_insert(1);
            return words;
        });
}
