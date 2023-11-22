//! Display attributes and information about the generated word list

pub mod uniquely_decodable;
use crate::count_characters;
use crate::display_information::uniquely_decodable::is_uniquely_decodable;
use crate::parse_delimiter;
use crate::split_and_vectorize;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct ListAttributes {
    pub list_length: usize,
    pub mean_word_length: f32,
    pub entropy_per_word: f64,
    pub shortest_word_length: usize,
    pub shortest_word_example: String,
    pub longest_word_length: usize,
    pub longest_word_example: String,

    pub is_free_of_prefix_words: Option<bool>,
    pub is_free_of_suffix_words: Option<bool>,

    pub is_uniquely_decodable: Option<bool>,

    pub efficiency_per_character: f64,
    pub assumed_entropy_per_character: f64,
    pub is_above_brute_force_line: bool,
    pub is_above_shannon_line: bool,
    pub shortest_edit_distance: Option<usize>,
    pub mean_edit_distance: Option<f64>,
    pub longest_shared_prefix: Option<usize>,
    pub unique_character_prefix: Option<usize>,
    pub kraft_mcmillan: KraftMcmillanOutcome,
    pub samples: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum KraftMcmillanOutcome {
    Satisfied,
    NotSatisfied,
}
impl fmt::Display for KraftMcmillanOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KraftMcmillanOutcome::Satisfied => write!(f, "satisfied"),
            KraftMcmillanOutcome::NotSatisfied => write!(f, "not satisfied"),
        }
    }
}

fn make_attributes(list: &[String], level: u8, samples: bool) -> ListAttributes {
    let samples = if samples {
        Some(generate_samples(list))
    } else {
        None
    };
    let shortest_word_example = list
        .iter()
        .min_by(|a, b| count_characters(a).cmp(&count_characters(b)))
        .unwrap()
        .to_string();
    let longest_word_example = list
        .iter()
        .max_by(|a, b| count_characters(a).cmp(&count_characters(b)))
        .unwrap()
        .to_string();

    let is_free_of_prefix_words = if level >= 2 {
        Some(!has_prefix_words(list))
    } else {
        None
    };

    let is_free_of_suffix_words = if level >= 2 {
        Some(!has_suffix_words(list))
    } else {
        None
    };

    let is_uniquely_decodable = if level >= 2 {
        Some(is_uniquely_decodable(list))
    } else {
        None
    };

    let shortest_edit_distance = if level >= 3 {
        Some(find_shortest_edit_distance(list))
    } else {
        None
    };
    let mean_edit_distance = if level >= 3 {
        Some(find_mean_edit_distance(list))
    } else {
        None
    };

    let longest_shared_prefix = if level >= 4 {
        Some(find_longest_shared_prefix(
            list,
            Some(count_characters(&longest_word_example)),
        ))
    } else {
        None
    };
    let unique_character_prefix = if level >= 4 {
        // By definition, unique_character_prefix == longest_shared_prefix + 1
        // We have to use map in case longest_shared_prefix is None, which is
        // unlikely, but technically possible.
        longest_shared_prefix.map(|longest_shared_prefix| longest_shared_prefix + 1)
    } else {
        None
    };
    ListAttributes {
        list_length: list.len(),
        mean_word_length: mean_word_length(list),
        entropy_per_word: calc_entropy_per_word(list.len()),
        shortest_word_length: count_characters(&shortest_word_example),
        shortest_word_example,
        longest_word_length: count_characters(&longest_word_example),
        longest_word_example,
        efficiency_per_character: efficiency_per_character(list),
        assumed_entropy_per_character: assumed_entropy_per_character(list),
        is_above_brute_force_line: is_above_brute_force_line(list),
        is_above_shannon_line: is_above_shannon_line(list),
        is_free_of_prefix_words,
        is_free_of_suffix_words,
        is_uniquely_decodable,
        shortest_edit_distance,
        mean_edit_distance,
        longest_shared_prefix,
        unique_character_prefix,
        kraft_mcmillan: satisfies_kraft_mcmillan(list),
        samples,
    }
}

/// If user gets a passphrase consisting entirely of shortest words,
/// it's theoretically possible that we could OVERESTIMATE entropy
/// per word. We can deterimine if we've done this by comparing out
/// entropy estimate against a simple brute force attack of all lowercase
/// English letters, under which we assume each character adds roughly 4.7 bits of entropy.
/// Note that this slightly obscure method of calculation ensures that floating-point arithmetic is
/// not used, thus ensuring a higher level of accuracy.
fn is_above_brute_force_line(list: &[String]) -> bool {
    let g: i32 = 26; // roughly: assumed alphabet length
    let shortest_word_length = get_shortest_word_length(list) as u32;
    let list_length = list.len() as i32;
    list_length as f64 <= g.pow(shortest_word_length).into()
}

/// In 1951, Claude Shannon estimated that English words only have
/// about 2.6 bits of entropy per character, rather than (roughly) 4.7 bits per character.
/// <https://www.princeton.edu/~wbialek/rome/refs/shannon_51.pdf>
/// Thus, this is a more difficult line for a given list to pass above than
/// the "brute force" line described above.
fn is_above_shannon_line(list: &[String]) -> bool {
    let shortest_word_length = get_shortest_word_length(list) as u32;
    let g: f64 = 6.1; // 2**2.6 is 6.1 when we maintain correct number of significant digits.
    let list_length = list.len() as i32;
    list_length as f64 <= g.powf(shortest_word_length.into())
}

/// This is a large and long function that prints all of the attributes of
/// the generated (new) list.
///
/// We just want to "display" this information, rather than print it to files
/// or stdout, so we use `eprintln!`
pub fn display_list_information(
    list: &[String],
    level: u8,
    attributes_as_json: bool,
    ignore_ending_metadata_delimiter: Option<char>,
    ignore_starting_metadata_delimiter: Option<char>,
    samples: bool,
) {
    let list = make_list_free_of_metadata(
        list,
        ignore_starting_metadata_delimiter,
        ignore_ending_metadata_delimiter,
    );
    let list_attributes = make_attributes(&list, level, samples);
    if attributes_as_json {
        print_attributes_as_json(&list_attributes);
    } else {
        if level >= 1 {
            eprintln!("Attributes of new list");
            eprintln!("----------------------");
            eprintln!(
                "List length               : {} words",
                list_attributes.list_length
            );
            eprintln!(
                "Mean word length          : {:.2} characters",
                list_attributes.mean_word_length
            );
            eprintln!(
                "Length of shortest word   : {} characters ({})",
                list_attributes.shortest_word_length, list_attributes.shortest_word_example
            );
            eprintln!(
                "Length of longest word    : {} characters ({})",
                list_attributes.longest_word_length, list_attributes.longest_word_example
            );
            if let Some(is_free_of_prefix_words) = list_attributes.is_free_of_prefix_words {
                eprintln!("Free of prefix words?     : {}", is_free_of_prefix_words);
            }
            if let Some(is_free_of_suffix_words) = list_attributes.is_free_of_suffix_words {
                eprintln!("Free of suffix words?     : {:?}", is_free_of_suffix_words);
            }

            // At least for now, this one is EXPENSIVE
            if let Some(is_uniquely_decodable) = list_attributes.is_uniquely_decodable {
                eprintln!("Uniquely decodable?       : {:?}", is_uniquely_decodable);
            }

            eprintln!(
                "Entropy per word          : {:.3} bits",
                list_attributes.entropy_per_word
            );
            eprintln!(
                "Efficiency per character  : {:.3} bits",
                list_attributes.efficiency_per_character
            );
            eprintln!(
                "Assumed entropy per char  : {:.3} bits",
                list_attributes.assumed_entropy_per_character
            );
            eprintln!(
                "Above brute force line?   : {}",
                list_attributes.is_above_brute_force_line
            );

            if level >= 4 {
                eprintln!(
                    "Above Shannon line?       : {}",
                    list_attributes.is_above_shannon_line
                );
            }

            if let Some(shortest_edit_distance) = list_attributes.shortest_edit_distance {
                eprintln!("Shortest edit distance    : {}", shortest_edit_distance)
            }
            if let Some(mean_edit_distance) = list_attributes.mean_edit_distance {
                eprintln!("Mean edit distance        : {:.3}", mean_edit_distance)
            }

            if let Some(longest_shared_prefix) = list_attributes.longest_shared_prefix {
                eprintln!("Longest shared prefix     : {}", longest_shared_prefix)
            }
            // Numbers of characters required to definitely get to a unique
            // prefix
            if let Some(unique_character_prefix) = list_attributes.unique_character_prefix {
                eprintln!("Unique character prefix   : {}", unique_character_prefix)
            }

            if level >= 4 {
                eprintln!(
                    "Kraft-McMillan inequality : {}",
                    list_attributes.kraft_mcmillan
                );
            }
        }
        if let Some(samples) = list_attributes.samples {
            print_samples(samples)
        }
    }
}

fn print_attributes_as_json(list_attributes: &ListAttributes) {
    let json = serde_json::to_string(&list_attributes).unwrap();
    eprintln!("{}", json);
}

fn make_list_free_of_metadata(
    list: &[String],
    ignore_ending_metadata_delimiter: Option<char>,
    ignore_starting_metadata_delimiter: Option<char>,
) -> Vec<String> {
    match (
        ignore_ending_metadata_delimiter,
        ignore_starting_metadata_delimiter,
    ) {
        (Some(delimiter), None) => {
            let delimiter = parse_delimiter(delimiter).unwrap();
            let mut just_the_words = vec![];
            for word in list {
                let split_vec = split_and_vectorize(word, &delimiter.to_string());
                just_the_words.push(split_vec[1].to_string());
            }
            just_the_words
        }
        (None, Some(delimiter)) => {
            let delimiter = parse_delimiter(delimiter).unwrap();
            let mut just_the_words = vec![];
            for word in list {
                let split_vec = split_and_vectorize(word, &delimiter.to_string());
                just_the_words.push(split_vec[0].to_string());
            }
            just_the_words
        }
        (Some(ref _delimiter1), Some(ref _delimiter2)) => {
            panic!("Can't ignore metadata on both sides currently")
        }
        (None, None) => list.to_vec(),
    }
}

use rand::seq::SliceRandom;
/// Print 5 sample 6-word passphrases from the newly created
/// word list.
pub fn generate_samples(list: &[String]) -> Vec<String> {
    let mut samples: Vec<String> = vec![];
    for _n in 0..30 {
        match list.choose(&mut rand::thread_rng()) {
            Some(word) => samples.push(word.to_string()),
            None => panic!("Couldn't pick a random word"),
        }
    }
    samples
}

/// Calculate the entropy per word of a word list, given its size.
/// We want this entropy value measured in bits, hence the use
/// of log2()
///
/// Returns `f64` because this value to return (bits of entropy per
/// word) will most likely not be a whole number (which is fine!)
pub fn calc_entropy_per_word(list_length: usize) -> f64 {
    (list_length as f64).log2()
}

use crate::edit_distance::find_edit_distance;
/// Calculate the shortest edit distance between any two words on the list.
fn find_shortest_edit_distance(list: &[String]) -> usize {
    // This use of max_value is smelly, but not sure I know how to do it better.
    let mut shortest_edit_distance = u32::max_value();
    // I think I can cheat and only go through half of the list here
    for word1 in list[0..(list.len() / 2)].iter() {
        for word2 in list {
            if word1 != word2 {
                let this_edit_distance = find_edit_distance(word1, word2);
                if this_edit_distance < shortest_edit_distance {
                    shortest_edit_distance = this_edit_distance;
                }
                // If we're found an edit distance of 1, we know that'll be the
                // shortest possible (since Tidy removes duplicates by default, so
                // a shortest_edit_distance of 0 is NOT possible)
                if shortest_edit_distance == 1 {
                    return 1;
                }
            }
        }
    }
    shortest_edit_distance.try_into().unwrap()
}

/// Calculate the mean edit distance between all pairs of words on the list.
pub fn find_mean_edit_distance(list: &[String]) -> f64 {
    let mut sum_of_all_edit_distances: f64 = 0.0;
    let mut number_of_edit_distances_measured: f64 = 0.0;
    for (i, word1) in list.iter().enumerate() {
        // The list[0..i] upper-bound in this inner loop is so that we don't do
        // twice as many calls as necessary. Otherwise we would be finding the
        // edit distance from word1 -> word2 and word2 -> word1.
        // This also loop helpfully prevents us from checking a word's edit
        // distance to itself (0).
        for word2 in list[0..i].iter() {
            let this_edit_distance = find_edit_distance(word1, word2);
            number_of_edit_distances_measured += 1.0;
            sum_of_all_edit_distances += this_edit_distance as f64;
        }
    }
    eprintln!(
        "Number of edit distances recorded: {}\nSum of all edit distances: {}",
        number_of_edit_distances_measured, sum_of_all_edit_distances
    );
    (sum_of_all_edit_distances as f64) / (number_of_edit_distances_measured as f64)
}

/// Nested loops in this function get the `longest_shared_prefix`
/// between any two words on the given list. Returns length of this
/// longest shared prefix, a notable cryptographic metric.
/// Optionally takes longest_word_length to speed up process.
pub fn find_longest_shared_prefix(list: &[String], longest_word_length: Option<usize>) -> usize {
    let mut longest_shared_prefix = 0;

    // If longest_word_length is given, use that. If not,
    // calculate it here.
    let longest_word_length = match longest_word_length {
        Some(longest_word_length) => longest_word_length,
        None => count_characters(
            list.iter()
                .max_by(|a, b| count_characters(a).cmp(&count_characters(b)))
                .unwrap(),
        ),
    };
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
                // If we found a shared prefix that's only one fewer than the longest word on
                // the list, we know this is the longest shared prefix we'll ever find.
                // We can short-circuit return to save time.
                if this_shared_prefix_length == longest_word_length - 1 {
                    return this_shared_prefix_length;
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
                return count_characters(word2);
            }
        }
    }
    // Fall back to shorter word length
    if count_characters(word1) < count_characters(word2) {
        count_characters(word1)
    } else {
        count_characters(word2)
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

/// Checks if a list has any words that are suffixes of other
/// words on the list.
fn has_suffix_words(list: &[String]) -> bool {
    for word1 in list {
        for word2 in list {
            if word1 != word2 && word1.ends_with(word2) {
                return true;
            }
        }
    }
    false
}

/// Assuming that users get a passphrase consisting solely of
/// the shortest word on the list, we want to check against
/// a brute-force attack in exactly that situation. To do so,
/// we calculate a value I'm calling "assumed entropy per character".
///
/// If this value is above `log2(26)` or 4.7 bits, there's a chance
/// that we'd _over_-estimate the entropy of passphrases created
/// using the word list.
pub fn assumed_entropy_per_character(list: &[String]) -> f64 {
    let shortest_word_length = get_shortest_word_length(list) as f64;
    let assumed_entropy_per_word = calc_entropy_per_word(list.len());

    assumed_entropy_per_word / shortest_word_length
}

/// Calculates the "efficiency" of the list.
/// Basically this is the number of bits of entropy generated by
/// the AVERAGE character. Thus it is different from
/// `assumed_entropy_per_word`, which you can think of as
/// the "worst case scenario" (user getting only words of the SHORTEST
/// length in their passphrase).
pub fn efficiency_per_character(list: &[String]) -> f64 {
    let mean_word_length = mean_word_length(list) as f64;
    let entropy_per_word = calc_entropy_per_word(list.len());

    entropy_per_word / mean_word_length
}

/// This function returns a bool based on whether the list fulfills something
/// called the McMillan Inequality
/// See: <https://www.youtube.com/watch?v=yHw1ka-4g0s>
pub fn satisfies_kraft_mcmillan(list: &[String]) -> KraftMcmillanOutcome {
    let alphabet_size = count_unique_characters(list);
    let mut running_total: f64 = 0.0;
    for word in list {
        running_total +=
            1.0 / (alphabet_size.pow(count_characters(word).try_into().unwrap()) as f64);
    }
    if running_total <= 1.0 {
        KraftMcmillanOutcome::Satisfied
    } else {
        KraftMcmillanOutcome::NotSatisfied
    }
}

fn count_unique_characters(list: &[String]) -> usize {
    let mut characters = vec![];
    for word in list {
        for l in word.chars() {
            characters.push(l);
        }
    }
    characters.sort();
    characters.dedup();
    characters.len()
}

/// A simple helper function that gets the shortest word on
/// a list.
pub fn get_shortest_word_length(list: &[String]) -> usize {
    count_characters(
        list.iter()
            .min_by(|a, b| count_characters(a).cmp(&count_characters(b)))
            .unwrap(),
    )
}

/// Calculates mean (or average) word length of given word
/// list
pub fn mean_word_length(list: &[String]) -> f32 {
    list.iter()
        .map(|word| count_characters(word))
        .sum::<usize>() as f32
        / list.len() as f32
}

fn print_samples(samples: Vec<String>) {
    eprintln!("\nWord samples");
    eprintln!("------------");
    for n in 0..30 {
        if n != 0 && n % 6 == 0 {
            // if we're at the end of the 6th word,
            // print a newline
            eprintln!();
        } else if n != 0 {
            // else just print a space to go between each
            // word
            eprint!(" ");
        }
        eprint!("{}", samples[n]);
    }
    eprintln!();
}
