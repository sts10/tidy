extern crate structopt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use tidy::*;
pub mod display_information;
use crate::display_information::display_list_information;

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

    /// Set number of leading characters to get to a unique prefix,
    /// which can aid auto-complete functionality.
    /// Setting this value to say, 4, means that knowing the first
    /// 4 characters of any word on the generated list is enough
    /// to know which word it is.
    #[structopt(short = "x", long = "shared-prefix-length")]
    maximum_shared_prefix_length: Option<usize>,

    /// Path for optional list of words to reject
    #[structopt(short = "r", long = "reject", parse(from_os_str))]
    reject_list: Option<PathBuf>,

    /// Path for optional list of approved words
    #[structopt(short = "a", long = "approve", parse(from_os_str))]
    approved_list: Option<PathBuf>,

    /// Path for file with a list of homophone pairs. There must be one pair
    /// of homophones per line, separated by a comma.
    #[structopt(short = "h", long = "homophones", parse(from_os_str))]
    homophones_list: Option<PathBuf>,

    /// Print dice roll next to word in output. Set number of sides
    /// of dice. Must be between 2 and 9. Use 6 for normal dice.
    #[structopt(short = "D", long = "dice")]
    dice_sides: Option<u8>,

    /// Path for outputted list file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Word list input files. Can be more than one, in which case
    /// they'll be combined and de-duplicated.
    #[structopt(name = "Inputted Word Lists", parse(from_os_str))]
    inputted_word_list: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    // Validate dice_sides
    if let Some(dice_sides) = opt.dice_sides {
        if !(2 < dice_sides && dice_sides < 9) {
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
        maximum_shared_prefix_length: opt.maximum_shared_prefix_length,
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
                            .expect("Unable to write dice roll to file");
                    }
                    writeln!(f, "{}", word).expect("Unable to write word to file");
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
