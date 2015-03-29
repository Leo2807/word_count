
#![feature(collections)]

mod pivot_sort;
mod word_index;

pub fn count_words (input: &str, mut word_index: &mut Vec<word_index::IndexedWord>) {

    let mut current_word = String::new();

    for character in input.chars() {

        // char is part of a word
        if character.is_alphanumeric() {
            current_word.push(character);

        // multiple non-word characters
        } else if current_word.is_empty()  {
            continue;

        // new word
        } else {
            word_index::add_word(current_word, &mut word_index);
            current_word = String::new();
        }
    }

    // sort the output
    pivot_sort::pivot_sort_high_to_low(&mut word_index);
}

impl pivot_sort::Sortable for word_index::IndexedWord {
    fn weight(&self) -> i64 {
        self.appeared
    }
}
