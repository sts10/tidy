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

    /// Only first N words from inputted word list. If two or more
    /// word lists are inputted, it will combine arbitrarily and then
    /// take first N words
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

    /// Force outputting of lists that fall below the brute force line
    #[structopt(short = "f", long = "force")]
    force_ignore_brute_line: bool,

    /// Path for outputted list file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Word list input files
    #[structopt(name = "Inputted Word Lists", parse(from_os_str))]
    inputted_word_list: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    // if opt.verbose {
    //     println!("Received options: {:?}", opt);
    // }

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

    if is_below_brute_force_line(&tidied_list) {
        eprintln!("WARNING: The shortest word(s) on this new list is {} and the list is {} words-long. Assuming the list is made up of lowercase English characers, that places it BELOW the brute force line!\nConsider increasing minium word length (-m flag).", get_shortest_word_length(&tidied_list), tidied_list.len());
        if !opt.force_ignore_brute_line {
            eprintln!("You may force an override of this warning by using the --force/-f flag");
            return;
        }
    }

    if !opt.dry_run {
        match opt.output {
            Some(output) => {
                let mut f = File::create(output).expect("Unable to create file");
                for word in &tidied_list {
                    writeln!(f, "{}", word).expect("Unable to write data to file");
                }
            }
            // If no output file destination, print resulting like, word by word
            // to println (which goes to stdout, allowing for use of > on command like)
            None => {
                for word in &tidied_list {
                    println!("{}", word)
                }
            }
        }
    }
    if !opt.quiet {
        eprintln!("Done making list");
        display_list_information(&tidied_list);
    }
}

// We just want to "display" this information, rather than print it to files,
// so we use eprintln!
fn display_list_information(list: &[String]) {
    eprintln!("Attributes of new list");
    eprintln!("----------------------");
    let list_length = list.len();
    eprintln!("List length           : {}", list_length);
    let entropy_per_word = calc_entropy(list.len());
    eprintln!("Entropy of per word   : {:.4}", entropy_per_word);
    let shortest_word = list.iter().min_by(|a, b| a.len().cmp(&b.len())).unwrap();
    eprintln!(
        "Shortest word         : {} ({})",
        shortest_word.chars().count(),
        shortest_word
    );
    let longest_word = list.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();
    eprintln!(
        "Longest word          : {} ({})",
        longest_word.chars().count(),
        longest_word
    );
    let free_of_prefix_words = !has_prefix_words(list);
    eprintln!("Free of prefix words  : {}", free_of_prefix_words);
    let under_brute_line: bool = is_below_brute_force_line(list);
    eprintln!("Above brute force line: {}", !under_brute_line);
    let shortest_edit_distance = find_shortest_edit_distance(list);
    eprintln!("Shortest edit distance: {}", shortest_edit_distance);
}

use crate::edit_distance::find_edit_distance;
fn find_shortest_edit_distance(list: &[String]) -> usize {
    let mut shortest_edit_distance = u32::max_value();
    // I think I can cheat aand only go through half of the
    // list here
    for word1 in list[0..(list.len() / 2)].iter() {
        for word2 in list {
            if word1 != word2 {
                let this_edit_distance = find_edit_distance(word1, word2);
                if this_edit_distance < shortest_edit_distance {
                    shortest_edit_distance = this_edit_distance;
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
