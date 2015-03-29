//!
//! Main executable.
//!
//! This file compiles to an executable.
//!
//! # Panics
//!
//! Panics when it can't read from stdin.
//!
//! # Examples
//!
//! ```sh
//! $ echo 'I like cookies. Mmm... Cookies.' | word_count
//! 'cookies':	  2
//! 'mmm':		  1
//! 'like':		  1
//! 'i':		  1
//! ```

#![feature(old_io, collections)]

extern crate lib_word_count;
use lib_word_count as word_count;

use std::old_io as io;
use std::old_io::Reader;

#[stable]
fn main() {

    let mut word_index = Vec::new();

    let input = match io::stdio::stdin().read_to_string() {
        Ok(result) => result,
        Err(error) => panic!("Could not read from stdin: {:?}", error)
    };

    word_count::count_words(&input, &mut word_index);

    for indexed_word in word_index {
        if indexed_word.word.len() >= 5 {
            println!("'{}':\t{}", indexed_word.word, indexed_word.appeared);
        } else {
            println!("'{}':\t\t{}", indexed_word.word, indexed_word.appeared)
        }
    }
}
