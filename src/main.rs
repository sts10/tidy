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
    #[structopt(short = "p", long = "remove_prefix")]
    remove_prefix_words: bool,

    /// Remove all integers from words
    #[structopt(short = "i", long = "remove_integers")]
    remove_integers: bool,

    /// Strip through first tab
    #[structopt(short = "t", long = "remove_through_tab")]
    remove_through_first_tab: bool,

    /// Minimum word length
    #[structopt(short = "m", long = "minimum")]
    minimum_length: Option<usize>,

    /// Path for optional list of words to reject
    #[structopt(short = "r", long = "reject", parse(from_os_str))]
    reject_list: Option<PathBuf>,

    /// Path for outputted list file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,

    /// Word list input files
    #[structopt(name = "Inputted Word Lists", parse(from_os_str))]
    inputted_word_list: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    if opt.verbose {
        println!("Received options: {:?}", opt);
    }

    let word_list: Vec<String> = make_vec(&opt.inputted_word_list);

    let reject_list: Option<Vec<String>> = match opt.reject_list {
        Some(list) => Some(make_vec(&[list])),
        None => None,
    };

    let tidied_list = tidy_list(
        word_list,
        opt.to_lowercase,
        opt.remove_prefix_words,
        opt.remove_integers,
        opt.remove_through_first_tab,
        reject_list,
        opt.minimum_length,
    );

    let mut f = File::create(opt.output).expect("Unable to create file");
    for i in &tidied_list {
        writeln!(f, "{}", i).expect("Unable to write data to file");
    }
    if opt.display_entropy || opt.verbose {
        println!("Done");
    }
    if opt.display_entropy {
        display_list_information(tidied_list.len(), opt.remove_prefix_words);
    }
}

fn display_list_information(list_len: usize, removed_prefix_words: bool) {
    println!("New list is {} lines long.", list_len);
    if removed_prefix_words {
        println!("Assuming you choose words randomly, each word adds approximately {:.4} bits of entropy.",
        calc_entropy(list_len)
    );
    } else {
        println!("Assuming you choose words randomly and you use a separator between the words, each word adds approximately {:.4} bits of entropy.",
            calc_entropy(list_len)
        );
    }
}
