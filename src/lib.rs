#![unstable]
#![feature(collections)]
//! This is a library to count the words in a text.
//!
//! Words will be sorted by the amount of times they appear in a piece of text.

pub mod pivot_sort;
pub mod word_index;

/// Count the appearances of each word in a string.
///
/// # Arguments
///
/// * `input`        The string containing the text you want to analyze.
///
/// * `word_index`   A reference to a vector of IndexedWords. This will be filled with the counted
///                 words.
///
/// # Examples
///
/// ```
/// let mut word_index = Vec::new();
/// let input = "I like cookies. Mmm... Cookies.";
///
/// lib_word_count::count_words(input, &mut word_index);
///
/// assert_eq!(word_index.len(), 4);
///
/// assert_eq!(word_index[0].word, "cookies".to_string());
/// assert_eq!(word_index[1].word, "i".to_string());
/// assert_eq!(word_index[2].word, "like".to_string());
/// assert_eq!(word_index[3].word, "mmm".to_string());
///
/// assert_eq!(word_index[0].appeared, 2);
/// assert_eq!(word_index[1].appeared, 1);
/// assert_eq!(word_index[2].appeared, 1);
/// assert_eq!(word_index[3].appeared, 1);
/// ```
#[unstable]
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

/// Words are sorted by the amount of times they appeared.
impl pivot_sort::Sortable for word_index::IndexedWord {
    fn weight(&self) -> i64 {
        self.appeared
    }
}
