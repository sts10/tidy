use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
pub mod display_information;
pub mod edit_distance;
pub mod list_manipulations;
pub mod sardinas_patterson_pruning;
use crate::list_manipulations::*;

#[derive(Default, Debug, Clone)]
pub struct TidyRequest {
    pub list: Vec<String>,
    pub take_first: Option<usize>,
    pub take_rand: Option<usize>,
    pub sort_alphabetically: bool,
    pub ignore_after_delimiter: Option<char>,
    pub ignore_before_delimiter: Option<char>,
    pub to_lowercase: bool,
    pub should_straighten_quotes: bool,
    pub should_remove_prefix_words: bool,
    pub should_remove_suffix_words: bool,
    pub should_schlinkert_prune: bool,
    pub should_remove_nonalphanumeric: bool,
    pub should_delete_nonalphanumeric: bool,
    pub should_remove_nonalphabetic: bool,
    pub should_remove_non_latin_alphabetic: bool,
    pub should_remove_nonascii: bool,
    pub should_remove_integers: bool,
    pub should_delete_integers: bool,
    pub should_delete_after_first_delimiter: Option<char>,
    pub should_delete_before_first_delimiter: Option<char>,
    pub reject_list: Option<Vec<String>>,
    pub approved_list: Option<Vec<String>>,
    pub homophones_list: Option<Vec<(String, String)>>,
    pub minimum_length: Option<usize>,
    pub maximum_length: Option<usize>,
    pub maximum_shared_prefix_length: Option<usize>,
    pub minimum_edit_distance: Option<usize>,
    pub cut_to: Option<usize>,
}

#[derive(PartialEq)]
enum MetadataPosition {
    Start,
    End,
}

/// Takes a slice of `PathBuf`s representing the word list(s)
/// that the user has inputted to the program. Then iterates
/// through each file and addes each line to Vec<String>. (Blank
/// lines and duplicate links will be handled elsewhere.)
pub fn make_vec_from_filenames(filenames: &[PathBuf]) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    for filename in filenames {
        let f = match File::open(filename) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file {:?}: {}", filename, e),
        };
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!(
                        "Error reading a line from file {:?}: {}\nWill continue reading file.",
                        filename, e
                    );
                    continue;
                }
            };
            word_list.push(l);
        }
    }
    word_list
}

/// Like `make_vec_from_filenames`, this function takes a slice of `PathBuf`s of
/// files. But in this case these files represent lists of homophones that the
/// user wants to make sure aren't both on the resulting list.
///
/// These homophone files are expected to be formatted such that each line of the file
/// is `homophone1,homophone2`.
///
/// This function produces a Vector of tuples of strings, representing the
/// homophone pairs.
pub fn read_homophones_list_from_filenames(filenames: &[PathBuf]) -> Vec<(String, String)> {
    let mut homophones_list: Vec<(String, String)> = vec![];
    for filename in filenames {
        let f = match File::open(filename) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file {:?}: {}", filename, e),
        };
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!(
                        "Error reading a line from file {:?}: {}\nWill continue reading file.",
                        filename, e
                    );
                    continue;
                }
            };
            let pair: (String, String) = (
                split_and_vectorize(&l, ",")[0].trim().to_string(),
                split_and_vectorize(&l, ",")[1].trim().to_string(),
            );
            homophones_list.push(pair);
        }
    }
    homophones_list
}

/// Simple helper function that splits a `str` by a given substring `str`,
/// Then returns a Vector of `str`s.
/// ```
/// use tidy::split_and_vectorize;
/// assert_eq!(split_and_vectorize("a:b:c",":"), vec!["a","b","c"]);
/// ```
/// I find this a handy general helper function.
pub fn split_and_vectorize<'a>(string_to_split: &'a str, splitter: &str) -> Vec<&'a str> {
    string_to_split.split(splitter).collect()
}

/// This is the large, key function of the program. It takes
/// a `TidyRequest` object -- which includes the word list --
/// and performs whatever functions the user has requesteed to
/// perform on the list.
pub fn tidy_list(req: TidyRequest) -> Vec<String> {
    // First, we need to do the two truncations
    let mut list_to_tidy = req.list.clone();
    list_to_tidy = match req.take_first {
        Some(amount_to_take) => {
            list_to_tidy.truncate(amount_to_take);
            list_to_tidy
        }
        None => list_to_tidy,
    };
    list_to_tidy = match req.take_rand {
        Some(amount_to_take) => {
            let mut rng = thread_rng();
            list_to_tidy.shuffle(&mut rng);
            list_to_tidy.truncate(amount_to_take);
            list_to_tidy
        }
        None => list_to_tidy,
    };
    let mut tidied_list = vec![];
    // Now we go word-by-word
    for word in &list_to_tidy {
        // METADATA-IGNORING WORD REMOVALS
        // If user chose to ignore metadata, split word vs. metadata on the first comma
        // found.
        // We'll then do removals operations on the "word", ignoriong metadata.
        // Later, we'll re-add the metadata to the word.

        // We need delimiter to have a broad scope so that we can use it
        // when we re-add the metadata at the end. Default to comma, but can be changed
        // in match statement here.
        let (mut new_word, delimiter, metadata, metadata_position) =
            match (req.ignore_after_delimiter, req.ignore_before_delimiter) {
                (Some(delimiter), None) => {
                    // Parse delimiter. Currently this converts 's' to ' '
                    // and 't' to '\t'.
                    let delimiter = parse_delimiter(delimiter).unwrap();
                    let split_vec = split_and_vectorize(word, &delimiter.to_string());
                    if split_vec.len() == 1 {
                        eprintln!("No metadata found for word: {:?}", word);
                        (word.to_string(), Some(delimiter), None, None)
                    } else {
                        (
                            split_vec[0].to_string(),
                            Some(delimiter),
                            Some(split_vec[1]),
                            Some(MetadataPosition::End),
                        )
                    }
                }
                (None, Some(delimiter)) => {
                    let delimiter = parse_delimiter(delimiter).unwrap();
                    let split_vec = split_and_vectorize(word, &delimiter.to_string());
                    if split_vec.len() == 1 {
                        eprintln!("No metadata found for word: {:?}", word);
                        (word.to_string(), Some(delimiter), None, None)
                    } else {
                        (
                            split_vec[1].to_string(),
                            Some(delimiter),
                            Some(split_vec[0]),
                            Some(MetadataPosition::Start),
                        )
                    }
                }
                (Some(ref _delimiter1), Some(ref _delimiter2)) => {
                    // This situation should be caught and handled better
                    // in src/main.rs, so this is really just in case.
                    panic!("Can't ignore metadata on both sides currently")
                }
                (None, None) => (word.to_string(), None, None, None),
            };

        new_word = new_word.trim_start().trim_end().to_string();

        // WORD MODIFICATIONS
        // For logic reasons, it's crucial that Tidy perform these word
        // modifications BEFORE it runs word removals.
        // If user has chosen to Ignore Metadata, we're guranteed
        // that all of these will be None, so we don't have to worry
        // about metadata loss due to de-duplication caused by word modification.
        new_word = match req.should_delete_before_first_delimiter {
            Some(delimiter) => delete_before_first_char(&new_word, delimiter).to_string(),
            None => new_word,
        };
        new_word = match req.should_delete_after_first_delimiter {
            Some(delimiter) => delete_after_first_char(&new_word, delimiter).to_string(),
            None => new_word,
        };
        if req.should_delete_integers {
            new_word = delete_integers(new_word.to_string());
        }
        if req.should_delete_nonalphanumeric {
            new_word = delete_nonalphanumeric(new_word.to_string());
        }
        if req.to_lowercase {
            new_word = new_word.to_ascii_lowercase();
        }
        if req.should_straighten_quotes {
            new_word = straighten_quotes(&new_word).to_string();
        }

        new_word = new_word.trim_start().trim_end().to_string();

        // WORD REMOVALS
        // Now that the words have been modified, we can move on to
        // word removals.
        // IF user has chosen to ignore any metadata, these should be the
        // first edits that we do.
        if req.should_remove_nonascii {
            // https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii
            if new_word.chars().any(|chr| !chr.is_ascii()) {
                new_word = "".to_string();
                // new_word = None
            }
        }
        if req.should_remove_nonalphanumeric && new_word.chars().any(|c| !c.is_alphanumeric()) {
            new_word = "".to_string();
        }
        if req.should_remove_nonalphabetic && new_word.chars().any(|c| !c.is_alphabetic()) {
            new_word = "".to_string();
        }
        if req.should_remove_non_latin_alphabetic
            && new_word.chars().any(|chr| !is_latin_alphabetic(chr as u16))
        {
            new_word = "".to_string();
        }
        if req.should_remove_integers && new_word.chars().any(|c| c.is_numeric()) {
            new_word = "".to_string();
        }

        match req.reject_list {
            Some(ref reject_list) => {
                if reject_list.contains(&new_word) {
                    new_word = "".to_string();
                }
            }
            None => (),
        };

        match req.approved_list {
            Some(ref approved_list) => {
                if !approved_list.contains(&new_word) {
                    new_word = "".to_string();
                }
            }
            None => (),
        };

        match req.minimum_length {
            Some(minimum_length) => {
                if new_word.chars().count() < minimum_length {
                    new_word = "".to_string();
                }
            }
            None => (),
        };
        match req.maximum_length {
            Some(maximum_length) => {
                if new_word.chars().count() > maximum_length {
                    new_word = "".to_string();
                }
            }
            None => (),
        };

        // trim whitespace
        new_word = new_word.trim_start().trim_end().to_string();

        // If there was metadata, re-add it to the word now.
        if new_word != "" {
            match metadata {
                Some(metadata) => {
                    if metadata_position == Some(MetadataPosition::End) {
                        new_word = new_word + &delimiter.unwrap().to_string() + metadata;
                    } else if metadata_position == Some(MetadataPosition::Start) {
                        new_word =
                            metadata.to_owned() + &delimiter.unwrap().to_string() + &new_word;
                    }
                }
                None => (),
            };
        }

        // trim whitespace
        new_word = new_word.trim_start().trim_end().to_string();
        // check if blank
        if new_word != "" {
            tidied_list.push(new_word);
        }
    }
    // Now truncate list, if requested
    // Some operations are just a bit too complex for
    // me to figure out how to do on a per-word basis
    // at this time. Maybe something to revisit in the
    // future
    tidied_list = match req.homophones_list {
        Some(homophones_list) => remove_homophones(tidied_list, homophones_list),
        None => tidied_list,
    };
    // I think this is a good order for these next few operations,
    // but I'm not super confident
    tidied_list = match req.maximum_shared_prefix_length {
        Some(maximum_shared_prefix_length) => {
            guarantee_maximum_prefix_length(&tidied_list, maximum_shared_prefix_length)
        }
        None => tidied_list,
    };
    tidied_list = match req.minimum_edit_distance {
        Some(minimum_edit_distance) => {
            enfore_minimum_edit_distance(tidied_list, minimum_edit_distance)
        }
        None => tidied_list,
    };
    tidied_list = if req.should_remove_suffix_words {
        remove_suffix_words(dedup_without_sorting(&mut tidied_list))
    } else {
        tidied_list
    };
    tidied_list = if req.should_remove_prefix_words {
        remove_prefix_words(dedup_without_sorting(&mut tidied_list))
    } else {
        tidied_list
    };
    tidied_list = if req.should_schlinkert_prune {
        schlinkert_prune(&dedup_without_sorting(&mut tidied_list))
    } else {
        tidied_list
    };

    // Sort and dedup here
    if req.sort_alphabetically {
        tidied_list.sort();
    }
    tidied_list = dedup_without_sorting(&mut tidied_list);

    // User can cut words from nearly finished list.
    // Does so randomly.
    tidied_list = match req.cut_to {
        Some(amount_to_cut) => {
            let mut rng = thread_rng();
            tidied_list.shuffle(&mut rng);
            tidied_list.truncate(amount_to_cut);
            tidied_list
        }
        None => tidied_list,
    };
    // Finally, sort and dedup list one more time
    // (probably unneccesary, since we've only cut
    // words since that last time we sorted and de-duped.)
    if req.sort_alphabetically {
        tidied_list.sort();
    }
    tidied_list = dedup_without_sorting(&mut tidied_list);
    tidied_list
}

use radix_fmt::*; // https://stackoverflow.com/a/50278316
/// Print dice rolls before each corresponding word. Note
/// that the `n` parameter should be zero-indexed. A tab (`\t`)
/// is printed between the dice roll and the word.
///
/// The `base` parameter represents the number of sides of the
/// dice, which can be set from 2 to 9.
///
/// Here's an example of an outputted word list with base 6:
/// ```text
/// 11111    aback
/// 11112    abandons
/// 11113    abated
/// 11114    abbey
/// 11115    abbot
/// 11116    abbreviated
/// 11121    abdomen
/// 11122    abducted
/// 11123    aberrant
/// 11124    abide
/// 11125    ability
/// 11126    abject
/// 11131    abnormally
/// // etc.
/// ```
///
/// If this base is between 4 and 8,
/// this function assumes the user will be using actual dice, which are index at 1.
/// Thus, `if 4 <= base && base <= 8`, we add `1` to each digit of the dice
/// roll before printing it.
///
/// I wish I could replicate this radix function easily without the dependency,
/// but that doesn't seem [very easy](https://stackoverflow.com/a/50278316).
pub fn print_as_dice(n: usize, base: u8, list_length: usize, use_letters: bool) -> String {
    // Set width for zero-padding

    // First, get the literal width of the largest number we'll be printing.
    // This is, by definition the length of the list.
    // We want the length of the number in the base we want to print all
    // the numbers, so use radix function.
    let n_as_base = radix(n, base);

    // Pad dice roll numbers with zeros
    let n_width = n_as_base.to_string().len();
    let pad_width = radix(list_length - 1, base).to_string().len();

    let mut padded_n = String::new();
    for _i in n_width..pad_width {
        padded_n.push('0');
    }
    // Now that we have the appropriate number of zeros
    // in `padded_n`, it's time to add our number
    padded_n += &n_as_base.to_string();

    // Print the dice rolls in slightly different ways,
    // depending on the value of the base.
    if use_letters {
        // We'll use zero-indexed values if sides_as_letters is
        // selected
        match base {
            // Values of 0 and 1 should have been caught earlier,
            // so we'll panic! if we have them here
            0 | 1 => panic!("Too few dice sides entered"),
            2..=36 => padded_n
                .chars()
                .map(|ch| ch.to_string().to_uppercase())
                .collect::<String>()
                .trim()
                .to_string(),
            _ => panic!("Amount of dice sides received is too high"),
        }
    } else {
        // We'll use 1-indexed values if sides_as_letters is NOT
        // selected
        match base {
            0 | 1 => panic!("Too few dice sides entered"),
            2..=9 => padded_n
                .chars()
                .map(|ch| (ch.to_string().parse::<usize>().unwrap() + 1).to_string())
                .collect::<String>(),
            10..=36 => padded_n
                .chars()
                .map(|ch| char_to_digit(ch) + "-")
                .collect::<String>()
                .trim_end_matches('-')
                .trim()
                .to_string(),
            _ => panic!("Amount of dice sides received is too high"),
        }
    }
}

/// Convert 0-z inputted character to a 1-indexed, padded string ("01" to "36")
fn char_to_digit(ch: char) -> String {
    match ch {
        '0' => "01",
        '1' => "02",
        '2' => "03",
        '3' => "04",
        '4' => "05",
        '5' => "06",
        '6' => "07",
        '7' => "08",
        '8' => "09",
        '9' => "10",
        'a' => "11",
        'b' => "12",
        'c' => "13",
        'd' => "14",
        'e' => "15",
        'f' => "16",
        'g' => "17",
        'h' => "18",
        'i' => "19",
        'j' => "20",
        'k' => "21",
        'l' => "22",
        'm' => "23",
        'n' => "24",
        'o' => "25",
        'p' => "26",
        'q' => "27",
        'r' => "28",
        's' => "29",
        't' => "30",
        'u' => "31",
        'v' => "32",
        'w' => "33",
        'x' => "34",
        'y' => "35",
        'z' => "36",
        _ => panic!("Unable to convert this dice number from a letter to a number."),
    }
    .to_string()
}

/// Little helper function that allows users to write out whitespace
/// delimiters "s" and "t", rather than having to enter the whitespace
/// characters literally.
pub fn parse_delimiter(delimiter: char) -> Option<char> {
    if delimiter == 's' {
        Some(' ')
    } else if delimiter == 't' {
        Some('\t')
    } else {
        Some(delimiter)
    }
}

/// Used for the to_widdle option
pub fn get_new_starting_point_guess(
    previous_starting_point: usize,
    this_list_length: usize,
    length_to_widdle_to: usize,
) -> usize {
    let mut starting_point = previous_starting_point;
    if this_list_length > length_to_widdle_to {
        // We're too high!
        let difference = this_list_length - length_to_widdle_to;
        let multiplier = starting_point as f64 / length_to_widdle_to as f64;
        let change = (difference as f64 * multiplier).floor() as usize;
        starting_point -= change;
    } else {
        // We're too low!
        let difference = length_to_widdle_to - this_list_length;
        let multiplier = starting_point as f64 / length_to_widdle_to as f64;
        let change = (difference as f64 * multiplier).floor() as usize;
        starting_point += change;
    }
    starting_point
}
