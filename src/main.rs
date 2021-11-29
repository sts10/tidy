extern crate structopt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use tidy::*;

/// tidy: Combine and clean word lists
#[derive(StructOpt, Debug)]
#[structopt(name = "tidy")]
struct Opt {
    /// Do not print any extra information
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Dry run. Don't write new list to file or terminal.
    #[structopt(long = "dry-run")]
    dry_run: bool,

    /// Print attributes about new list to terminal. Can be used more than once
    /// to print more attributes.
    #[structopt(short = "A", long = "attributes", parse(from_occurrences))]
    attributes: u8,

    /// Lowercase all words
    #[structopt(short = "l", long = "lowercase")]
    to_lowercase: bool,

    /// Remove prefix words from list
    #[structopt(short = "P", long = "remove-prefix")]
    remove_prefix_words: bool,

    /// Remove all words with non-alphanumeric characters from list
    #[structopt(short = "N", long = "remove-nonalphanumeric")]
    remove_nonalphanumeric: bool,

    /// Delete all non-alphanumeric characters from list
    #[structopt(short = "n", long = "delete-nonalphanumeric")]
    delete_nonalphanumeric: bool,

    /// Remove all words with non-alphabetic characters from list
    /// (leaving only words composed entirely of letters [A-Z] or [a-z])
    #[structopt(short = "L", long = "remove-nonalphabetic")]
    remove_nonalphabetic: bool,

    /// Remove all words with integers in them from list
    #[structopt(short = "I", long = "remove-integers")]
    remove_integers: bool,

    /// Delete all integers from words
    #[structopt(short = "i", long = "delete-integers")]
    delete_integers: bool,

    /// Delete characters through first tab
    #[structopt(short = "t", long = "delete-through-tab")]
    delete_through_first_tab: bool,

    /// Delete characters through first space
    #[structopt(short = "s", long = "delete-through-space")]
    delete_through_first_space: bool,

    /// Only take first N words from inputted word list.
    /// If two or more word lists are inputted, it will
    /// combine arbitrarily and then take first N words.
    #[structopt(long = "take-first")]
    take_first: Option<usize>,

    /// Set minimum word length
    #[structopt(short = "m", long = "minimum-word-length")]
    minimum_length: Option<usize>,

    /// Set maximum word length
    #[structopt(long = "maxium-word-length")]
    maximum_length: Option<usize>,

    /// Set minimum edit distance between words, which
    /// can reduce the cost of typos when entering words
    #[structopt(short = "d", long = "minimum-edit-distance")]
    minimum_edit_distance: Option<usize>,

    /// Set unique prefix length, which can aid auto-complete
    /// functionality
    #[structopt(short = "u", long = "unique-prefix-length")]
    unique_prefix_length: Option<usize>,

    /// Path for optional list of words to reject
    #[structopt(short = "r", long = "reject", parse(from_os_str))]
    reject_list: Option<PathBuf>,

    /// Path for optional list of approved words
    #[structopt(short = "a", long = "approve", parse(from_os_str))]
    approved_list: Option<PathBuf>,

    /// Path for optional list of homophone pairs. One pair
    /// per line, separated by a comma
    #[structopt(short = "h", long = "homophones", parse(from_os_str))]
    homophones_list: Option<PathBuf>,

    /// Print dice roll next to word in output. Set number of sides
    /// of dice. Must be between 2 and 9. Use 6 for normal dice.
    #[structopt(short = "D", long = "dice")]
    dice_sides: Option<u8>,

    /// Path for outputted list file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Word list input files
    #[structopt(name = "Inputted Word Lists", parse(from_os_str))]
    inputted_word_list: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    eprintln!("Opts: {:?}", opt);

    // Validate dice_sides
    if let Some(dice_sides) = opt.dice_sides {
        if dice_sides < 2 || dice_sides > 9 {
            eprintln!("Specified number of dice sides must be between 2 and 9.");
            return;
        }
    }
    let this_tidy_request = TidyRequest {
        list: make_vec_from_filenames(&opt.inputted_word_list),
        take_first: opt.take_first,
        to_lowercase: opt.to_lowercase,
        should_remove_prefix_words: opt.remove_prefix_words,
        should_remove_integers: opt.remove_integers,
        should_delete_integers: opt.delete_integers,
        should_remove_nonalphabetic: opt.remove_nonalphabetic,
        should_remove_nonalphanumeric: opt.remove_nonalphanumeric,
        should_delete_nonalphanumeric: opt.delete_nonalphanumeric,
        should_delete_through_first_tab: opt.delete_through_first_tab,
        should_delete_through_first_space: opt.delete_through_first_space,
        reject_list: opt.reject_list.map(|list| make_vec_from_filenames(&[list])),
        approved_list: opt
            .approved_list
            .map(|list| make_vec_from_filenames(&[list])),
        homophones_list: opt.homophones_list.map(|homophones_list_file| {
            read_homophones_list_from_filenames(&[homophones_list_file])
        }),
        minimum_length: opt.minimum_length,
        maximum_length: opt.maximum_length,
        unique_prefix_length: opt.unique_prefix_length,
        minimum_edit_distance: opt.minimum_edit_distance,
    };

    let tidied_list = tidy_list(this_tidy_request);

    if !opt.dry_run {
        eprintln!("Printing new list...");
        match opt.output {
            Some(output) => {
                let mut f = File::create(output).expect("Unable to create file");
                for (i, word) in tidied_list.iter().enumerate() {
                    if let Some(dice_sides) = opt.dice_sides {
                        write!(f, "{}\t", print_as_dice(i, dice_sides, tidied_list.len()),)
                            .unwrap();
                    }
                    writeln!(f, "{}", word).expect("Unable to write data to file");
                }
            }
            // If no output file destination, print resulting like, word by word
            // to println (which goes to stdout, allowing for use of > on command like)
            None => {
                for (i, word) in tidied_list.iter().enumerate() {
                    if let Some(dice_sides) = opt.dice_sides {
                        print!("{:}\t", print_as_dice(i, dice_sides, tidied_list.len()));
                    }
                    println!("{}", word);
                }
            }
        }
    }
    if opt.attributes > 0 && !opt.quiet {
        eprintln!("----------------\nDone making list\n");
        display_list_information(&tidied_list, opt.attributes);
    }
}

// We just want to "display" this information, rather than print it to files,
// so we use eprintln!
fn display_list_information(list: &[String], level: u8) {
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
        "Assumed entropy per letter: {:.2}",
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
        let shortest_edit_distance = find_shortest_edit_distance(list);
        eprintln!("Shortest edit distance    : {}", shortest_edit_distance);
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
