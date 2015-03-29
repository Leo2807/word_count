
#![feature(io, collections)]

use std::io::Read;
use std::cmp;

struct IndexedWord {
    word: String,
    appeared: i64
}

trait Sortable {
    fn weight(&self) -> i64;
}

fn main() {

    let mut current_word = String::new();
    let mut word_index: Vec<IndexedWord> = Vec::new();

    let stdin = std::io::stdin();

    for result in stdin.lock().chars() {

        match result {
                Err(..) => continue,
                Ok(character) => if character.is_alphanumeric() {
                    current_word.push(character);
                } else if current_word.is_empty()  {
                    continue;
                } else {
                    add_word(current_word, &mut word_index);
                    current_word = String::new();
                },
        }

    }

    pivot_sort_high_to_low(&mut word_index);

    for indexed_word in word_index {
        if indexed_word.word.len() >= 5 {
            println!("'{}':\t{}", indexed_word.word, indexed_word.appeared);
        } else {
            println!("'{}':\t\t{}", indexed_word.word, indexed_word.appeared)
        }
    }
}

fn add_word(word: String, index: &mut Vec<IndexedWord>) {

    for indexed_word in index.iter_mut() {
        if word == indexed_word.word.to_lowercase() {
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

impl Sortable for IndexedWord {
    fn weight(&self) -> i64 {
        self.appeared
    }
}

fn pivot_sort_high_to_low<'a, T: Sortable>(items: &mut Vec<T>) {
    if items.len() == 0 {
        return;
    }

    let pivot = match items.pop() {
        Some(valid_pivot) => valid_pivot,
        None => unreachable!(),
    };
    let pivot_weight = pivot.weight();

    let lower: &mut Vec<T> = &mut Vec::new();
    let higher: &mut Vec<T> = &mut Vec::new();
    let equal: &mut Vec<T> = &mut vec![pivot];

    while let Some(item) = items.pop() {
        match item.weight().cmp(&pivot_weight) {
            cmp::Ordering::Less => lower.push(item),
            cmp::Ordering::Equal => equal.push(item),
            cmp::Ordering::Greater => higher.push(item),
        }
    }

    pivot_sort_high_to_low(higher);
    pivot_sort_high_to_low(lower);

    items.append(higher);
    items.append(equal);
    items.append(lower);
}
