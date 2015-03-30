#![stable]
//! This module is used to index words and does the heavy lifting of our program.

/// A counted word.
///
/// This struct contains members for storing a word and the number of times it appeared. This
/// struct is intended for use with [`add_word`](fn.add_word.html).
///
/// # Examples
///
/// ```
/// use lib_word_count::word_index;
///
/// let indexed_word = word_index::IndexedWord{
///     word: "Text".to_string(),
///     appeared: 12
/// };
///
/// assert_eq!(indexed_word.word, "Text".to_string());
/// assert_eq!(indexed_word.appeared, 12i64);
/// ```
#[derive(Debug, PartialEq)]
#[stable]
pub struct IndexedWord {
    /// The word that's indexed.
    pub word: String,
    /// The amount of times this word appeared.
    pub appeared: i64
}

/// Add a word to a given index.
///
/// This function prevents duplicates and increments the count of the word appearances
/// automatically. The vector will be modified accordingly.
///
/// # Arguments
///
/// * `word`    A string containing the word to add.
///
/// * `index`   A reference to a vector containing all the indexed words.
///
/// # Examples
///
/// ```
/// use lib_word_count::word_index;
///
/// let mut index = Vec::new();
///
/// word_index::add_word("Hello".to_string(), &mut index);
/// word_index::add_word("hELLO".to_string(), &mut index);
/// word_index::add_word("World".to_string(), &mut index);
/// word_index::add_word("HELLO".to_string(), &mut index);
/// word_index::add_word("PFUDOR".to_string(), &mut index);
///
/// assert_eq!(index[0], word_index::IndexedWord{
///     word: "hello".to_string(),
///     appeared: 3
/// });
/// assert_eq!(index[1], word_index::IndexedWord{
///     word: "world".to_string(),
///     appeared: 1
/// });
/// assert_eq!(index[2], word_index::IndexedWord{
///     word: "pfudor".to_string(),
///     appeared: 1
/// });
/// ```
#[stable]
pub fn add_word(word: String, index: &mut Vec<IndexedWord>) {

    for indexed_word in index.iter_mut() {
        if word.to_lowercase() == indexed_word.word {
            indexed_word.appeared += 1;
            return;
        }
    }

    let new_word = IndexedWord{
        word: word.to_lowercase(),
        appeared: 1
    };

    index.push(new_word);
}
