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
    /// Prints verbose output, including parameters as received
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Display information about newly created list when done, including entropy-per-word
    #[structopt(short = "e", long = "entropy")]
    display_entropy: bool,

    /// Lowercase all words
    #[structopt(short = "l", long = "lowercase")]
    to_lowercase: bool,

    /// Remove prefix words from list
    #[structopt(short = "P", long = "remove_prefix")]
    remove_prefix_words: bool,

    /// Remove all words with non-alphanumeric characters from list
    #[structopt(short = "N", long = "remove_nonalphanumeric")]
    remove_nonalphanumeric: bool,

    /// Delete all non-alphanumeric characters from list
    #[structopt(short = "n", long = "delete_nonalphanumeric")]
    delete_nonalphanumeric: bool,

    /// Remove all words with integers in them from list
    #[structopt(short = "I", long = "remove_integers")]
    remove_integers: bool,

    /// Delete all integers from words
    #[structopt(short = "i", long = "delete_integers")]
    delete_integers: bool,

    /// Delete characters through first tab
    #[structopt(short = "t", long = "delete_through_tab")]
    delete_through_first_tab: bool,

    /// Delete characters through first space
    #[structopt(short = "s", long = "delete_through_space")]
    delete_through_first_space: bool,

    /// Set minimum word length
    #[structopt(short = "m", long = "minimum")]
    minimum_length: Option<usize>,

    /// Path for optional list of words to reject
    #[structopt(short = "r", long = "reject", parse(from_os_str))]
    reject_list: Option<PathBuf>,

    /// Path for optional list of approved words
    #[structopt(short = "a", long = "approve", parse(from_os_str))]
    approved_list: Option<PathBuf>,

    /// Path for optional list of homophone pairs, separated by a comma
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
    if opt.verbose {
        println!("Received options: {:?}", opt);
    }

    let this_tidy_request = TidyRequest {
        list: make_vec_from_filenames(&opt.inputted_word_list),
        to_lowercase: opt.to_lowercase,
        should_remove_prefix_words: opt.remove_prefix_words,
        should_remove_integers: opt.remove_integers,
        should_delete_integers: opt.delete_integers,
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
    };

    let tidied_list = tidy_list(this_tidy_request);

    if is_below_brute_force_line(&tidied_list) {
        eprintln!("WARNING: The shortest word(s) on this new list is {} and the list is {} words-long. Assuming the list is made up of lowercase English characers, that places it BELOW the brute force line!\nConsider increasing minium word length (-m flag).", get_shortest_word_length(&tidied_list), tidied_list.len());
        if !opt.force_ignore_brute_line {
            eprintln!("You may force an override of this warning by using the --force/-f flag");
            return;
        }
    }

    match opt.output {
        Some(output) => {
            let mut f = File::create(output).expect("Unable to create file");
            for word in &tidied_list {
                writeln!(f, "{}", word).expect("Unable to write data to file");
            }
        }
        // If no output file destination, print resulting like, word by word
        // to println (which goest tostdout, allowing for use of > on command like)
        None => {
            for word in &tidied_list {
                println!("{}", word)
            }
        }
    }
    if opt.verbose {
        eprintln!("Done");
    }
    if opt.display_entropy {
        display_list_information(tidied_list.len(), opt.remove_prefix_words);
    }
}

// We just want to "display" this information, rather than print it to files,
// so we use eprintln!
fn display_list_information(list_len: usize, removed_prefix_words: bool) {
    eprintln!("New list is {} lines long.", list_len);
    if removed_prefix_words {
        eprintln!("Assuming you choose words randomly, each word adds approximately {:.4} bits of entropy.",
            calc_entropy(list_len)
        );
    } else {
        eprintln!("Assuming you choose words randomly and you use a separator between the words, each word adds approximately {:.4} bits of entropy.",
            calc_entropy(list_len)
        );
    }
}
