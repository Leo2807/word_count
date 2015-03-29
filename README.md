% Word count

# word_count

This is a program/library to count the words in a string. Use it to analyze
texts of your favorite author.

# Examples

Executable:

```sh
$ echo 'I like cookies. Mmm... Cookies.' | word_count
'cookies':	2
'mmm':		  1
'like':		  1
'i':		    1
```

Library:

```rust
extern crate lib_word_count;

fn main() {

    let mut word_index = Vec::new();
    let input = "I like cookies. Mmm... Cookies.";

    lib_word_count::count_words(input, &mut word_index);

    assert_eq!(word_index.len(), 4);

    assert_eq!(word_index[0].word, "cookies".to_string);
    assert_eq!(word_index[1].word, "i".to_string);
    assert_eq!(word_index[2].word, "like".to_string);
    assert_eq!(word_index[3].word, "mmm".to_string);

    assert_eq!(word_index[0].appeared, 2);
    assert_eq!(word_index[1].appeared, 1);
    assert_eq!(word_index[2].appeared, 1);
    assert_eq!(word_index[3].appeared, 1);
}
```
