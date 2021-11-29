// use tidy::assumed_entropy_per_letter;
use crate::calc_entropy;
// We just want to "display" this information, rather than print it to files,
// so we use eprintln!
pub fn display_list_information(list: &[String], level: u8) {
    eprintln!("Attributes of new list");
    eprintln!("----------------------");
    let list_length = list.len();
    eprintln!("List length               : {}", list_length);
    let shortest_word = list.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap();
    eprintln!(
        "Length of shortest word   : {} ({})",
        shortest_word.chars().count(),
        shortest_word
    );
    let longest_word = list.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();
    eprintln!(
        "Length of longest word    : {} ({})",
        longest_word.chars().count(),
        longest_word
    );
    let free_of_prefix_words = !has_prefix_words(list);
    eprintln!("Free of prefix words      : {}", free_of_prefix_words);

    let entropy_per_word = calc_entropy(list.len());
    eprintln!("Entropy per word          : {:.4}", entropy_per_word);
    let assumed_entropy_per_letter = assumed_entropy_per_letter(list);
    eprintln!(
        "Assumed entropy per letter: {:.4}",
        assumed_entropy_per_letter
    );
    // If user gets a passphrase consisting entirely of shortest words,
    // it's theoretically possible that we could OVERESTIMATE entropy
    // per word. We can deterimine if we've done this by comparing out
    // entropy estimate against a simple brute force attack, under which
    // we assume each letter adds 4.7 bits of entropy.
    eprintln!(
        "Above brute force line    : {}",
        assumed_entropy_per_letter <= 4.7
    );

    // In 1951, Claude Shannon estimated that English words only have
    // about 2.62 bits of entropy per letter, rather than 4.7 bits per letter.
    // https://www.princeton.edu/~wbialek/rome/refs/shannon_51.pdf
    // Thus, this is a more difficult line for a given list to pass above than
    // the "brute force" line described above.
    eprintln!(
        "Above Shannon line        : {}",
        assumed_entropy_per_letter <= 2.62
    );

    if level >= 2 {
        eprintln!(
            "Shortest edit distance    : {}",
            find_shortest_edit_distance(list)
        );

        let longest_shared_prefix = find_longest_shared_prefix(list);
        eprintln!("Longest shared prefix     : {}", longest_shared_prefix);
        // Numbers of characters required to definitely get to a unique
        // prefix
        eprintln!("Unique character prefix   : {}", longest_shared_prefix + 1);
    }
}

use crate::edit_distance::find_edit_distance;
fn find_shortest_edit_distance(list: &[String]) -> usize {
    let mut shortest_edit_distance = u32::max_value();
    // I think I can cheat and only go through half of the
    // list here
    for word1 in list[0..(list.len() / 2)].iter() {
        for word2 in list {
            if word1 != word2 {
                let this_edit_distance = find_edit_distance(word1, word2);
                if this_edit_distance < shortest_edit_distance {
                    shortest_edit_distance = this_edit_distance;
                }
                // If we're found an edit distance of 1, we know that'll be the
                // shortest possible (since Tidy removes duplicates by default, so
                // a shortest_edit_distance of 0 is NOT possbile)
                if shortest_edit_distance == 1 {
                    return 1;
                }
            }
        }
    }
    shortest_edit_distance.try_into().unwrap()
}

pub fn find_longest_shared_prefix(list: &[String]) -> usize {
    let mut longest_shared_prefix = 0;
    for word1 in list {
        for word2 in list {
            if word1 != word2 {
                // Here we convert from zero-indexed first different to
                // the (1-indexed) length of the long shared prefix, so we don't
                // need a `- 1`.
                let this_shared_prefix_length =
                    find_first_different_character_zero_indexed(word1, word2);
                if this_shared_prefix_length > longest_shared_prefix {
                    longest_shared_prefix = this_shared_prefix_length;
                }
            }
        }
    }
    longest_shared_prefix
}

pub fn find_first_different_character_zero_indexed(word1: &str, word2: &str) -> usize {
    for (i, c1) in word1.chars().enumerate() {
        match word2.chars().nth(i) {
            Some(c2) => {
                if c1 != c2 {
                    return i;
                } else {
                    continue;
                }
            }
            // word1 is longer than word2
            None => {
                return word2.chars().count();
            }
        }
    }
    // Fall back to shorter word length
    if word1.chars().count() < word2.chars().count() {
        word1.chars().count()
    } else {
        word2.chars().count()
    }
}

fn has_prefix_words(list: &[String]) -> bool {
    for word1 in list {
        for word2 in list {
            if word1 != word2 && word1.starts_with(word2) {
                return true;
            }
        }
    }
    false
}

pub fn assumed_entropy_per_letter(list: &[String]) -> f64 {
    let shortest_word_length = get_shortest_word_length(list) as f64;
    let assumed_entropy_per_word = calc_entropy(list.len());

    assumed_entropy_per_word / shortest_word_length
}

pub fn get_shortest_word_length(list: &[String]) -> usize {
    let mut shortest_word_length: usize = usize::max_value();
    for word in list {
        if word.chars().count() < shortest_word_length {
            shortest_word_length = word.chars().count();
        }
    }
    shortest_word_length
}
