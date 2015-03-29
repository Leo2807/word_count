
pub struct IndexedWord {
    pub word: String,
    pub appeared: i64
}

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

    index.insert(0, new_word);
}
