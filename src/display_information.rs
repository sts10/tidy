//! Display attributes and information about the generated word list

/// This is a large and long function that prints all of the attributes of
/// the generated (new) list.
///
/// We just want to "display" this information, rather than print it to files
/// or stdout, so we use `eprintln!`
pub fn display_list_information(list: &[String], level: u8) {
    eprintln!("Attributes of new list");
    eprintln!("----------------------");
    let list_length = list.len();
    eprintln!("List length               : {}", list_length);
    eprintln!("Mean word length          : {:.2}", mean_word_length(list));
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

    let entropy_per_word = calc_entropy_per_word(list.len());
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

/// Calculate the entropy per word of a word list, given its size.
/// Entropy is meausred in bits, hence use of constant `2`.
///
/// Returns `f64` because this value to return (bits of entropy per
/// word) will most likely not be a whole bumber (which is fine!)
pub fn calc_entropy_per_word(list_size: usize) -> f64 {
    (list_size as f64).ln() / (2_f64.ln() as f64)
}

use crate::edit_distance::find_edit_distance;
/// This function finds the shortest edit distance between
/// any two words on the list.
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

/// Nested loops in this function get the `longest_shared_prefix`
/// between any two words on the given list. Returns length of this
/// longest shared prefix, a notable cryptographic metric.
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

/// Given 2 words, finds the index of the first character that is
/// **different** within them.
/// ```
/// use tidy::display_information::find_first_different_character_zero_indexed;
///
/// assert_eq!(
///     find_first_different_character_zero_indexed("hello", "help"), 3
///     // First **different** character is `l` vs. `p`.
/// );
///
/// // Handles words of different length by falling back to the length of the shorter
/// // of the two words:
/// assert_eq!(
///     find_first_different_character_zero_indexed("zip", "zippy"), 3
/// );
/// assert_eq!(
///     find_first_different_character_zero_indexed("zippy", "zip"), 3
/// );
/// ```
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

/// Checks if a list has any words that are prefixs of other
/// words on the list.
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

/// Assuming that users get a passphrase consisting solely of
/// the shortest word on the list, we want to check against
/// a brute-force attack in exactly that situation. To do so,
/// we calculate a value I'm calling "assumed entropy per letter".
///
/// If this value is above `log2(26)` or about `4.7` bits, there's a chance
/// that we'd _over_-estimate the entropy of passphrases created
/// using the word list.
pub fn assumed_entropy_per_letter(list: &[String]) -> f64 {
    let shortest_word_length = get_shortest_word_length(list) as f64;
    let assumed_entropy_per_word = calc_entropy_per_word(list.len());

    assumed_entropy_per_word / shortest_word_length
}

/// A simple helper function that gets the shortest word on
/// a list. Uses `.chars().count()` rather than `len()` to
/// handle non-English letters.
pub fn get_shortest_word_length(list: &[String]) -> usize {
    let mut shortest_word_length: usize = usize::max_value();
    for word in list {
        if word.chars().count() < shortest_word_length {
            shortest_word_length = word.chars().count();
        }
    }
    shortest_word_length
}

/// Calculates mean (or average) word length of given word
/// list
pub fn mean_word_length(list: &[String]) -> f32 {
    list.iter().map(|word| word.chars().count()).sum::<usize>() as f32 / list.len() as f32
}
