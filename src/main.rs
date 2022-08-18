use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use tidy::*;
pub mod display_information;
pub mod input_validations;
use crate::dice::print_as_dice;
use crate::display_information::display_list_information;
use crate::display_information::generate_samples;
use crate::file_readers::*;
use crate::input_validations::*;

/// Parse user's input to a handful of options, either directly as a `usize`,
/// or, if they entered Python exponent notation (base**exponent), which
/// we'll need to evaluate as an exponent. Either way, return a `usize`
/// or `expect`/`panic!`.
///  
/// This is useful when making lists fit to a specific amount of dice and
/// dice sides. (As an example, five rolls of a six-sided dice would be: 6**5).
fn eval_list_length(input: &str) -> usize {
    match input.split("**").collect::<Vec<&str>>().as_slice() {
        [] => panic!("Please specify a number."),
        [num_string] => num_string
            .parse::<usize>()
            .expect("Unable to parse print-rand! Enter a number or a base**exponent"),
        [base_string, exponent_string] => {
            let base: usize = base_string
                .parse::<usize>()
                .expect("Unable to parse base of print-rand!");
            let exponent: u32 = exponent_string
                .parse::<u32>()
                .expect("Unable to parse exponent of print-rand!");
            base.pow(exponent)
        }
        _ => panic!("You can only specify one exponent! Use format: base**exponent"),
    }
}

/// Combine and clean word lists
#[derive(Parser, Debug)]
#[clap(version, about, name = "tidy")]
struct Args {
    /// Do not print any extra information
    #[clap(long = "quiet")]
    quiet: bool,

    /// Dry run. Don't write new list to file or terminal.
    #[clap(long = "dry-run")]
    dry_run: bool,

    /// Debug mode
    #[clap(long = "debug")]
    debug: bool,

    /// Print attributes about new list to terminal. Can be used more than once
    /// to print more attributes. Some attributes may take a nontrivial amount
    /// of time to calculate.
    #[clap(short = 'A', long = "attributes", parse(from_occurrences))]
    attributes: u8,

    /// Print a handful of pseudorandomly selected words from the created list
    /// to the terminal. Should NOT be used as secure passphrases.
    #[clap(short = 's', long = "samples")]
    samples: bool,

    /// Ignore characters after the first instance of the specified delimiter until the end of line, treating
    /// anything before the delimiter as a word. Delimiter must be a single character (e.g., ','). Use 't'
    /// for tab and 's' for space. Helpful for ignoring metadata like word frequencies.
    /// Works with attribute analysis and most word removal options, but not with word modifications
    /// (like to lowercase). May not be used together with -d, -D or -G options.
    #[clap(short = 'g', long = "ignore-after")]
    ignore_after_delimiter: Option<char>,

    /// Ignore characters before and including the first instance of the specified delimiter, treating
    /// anything after the delimiter as a word. Delimiter must be a single character (e.g., ','). Use 't'
    /// for tab and 's' for space. Helpful for ignoring metadata like word frequencies.
    /// Works with attribute analysis and most word removal options, but not with word modifications
    /// (like to lowercase). May not be used together with -d, -D or -g options.
    #[clap(short = 'G', long = "ignore-before")]
    ignore_before_delimiter: Option<char>,

    /// Do NOT sort outputted list alphabetically. Preserves original list order.
    /// Note that duplicates lines and blank lines will still be removed.
    #[clap(short = 'O', long = "no-sort")]
    no_alpha_sort: bool,

    /// Lowercase all words on new list
    #[clap(short = 'l', long = "lowercase")]
    to_lowercase: bool,

    /// Replace “smart” quotation marks, both “double” and ‘single’,
    /// with their "straight" versions
    #[clap(short = 'q', long = "straighten")]
    straighten_quotes: bool,

    /// Remove prefix words from new list
    #[clap(short = 'P', long = "remove-prefix")]
    remove_prefix_words: bool,

    /// Remove suffix words from new list
    #[clap(short = 'S', long = "remove-suffix")]
    remove_suffix_words: bool,

    /// Use Sardinas-Patterson algorithm to remove words to make list
    /// uniquely decodable. Experimental!
    #[clap(short = 'K', long = "schlinkert-prune")]
    schlinkert_prune: bool,

    /// Remove all words with non-alphanumeric characters from new list. Words with diacritics will
    /// remain
    #[clap(short = 'N', long = "remove-nonalphanumeric")]
    remove_nonalphanumeric: bool,

    /// Delete all non-alphanumeric characters from all words on new list. Characters with diacritics
    /// will remain
    #[clap(short = 'n', long = "delete-nonalphanumeric")]
    delete_nonalphanumeric: bool,

    /// Remove all words with non-alphabetic characters from new list. Words with diacritcis and
    /// other non-Latin characters will remain.
    #[clap(long = "remove-nonalphabetic")]
    remove_nonalphabetic: bool,

    /// Remove all words with any characters not in the Latin alphabet (A through Z and a through
    /// z). All words with accented or diacritic characters will be removed, as well as
    /// any words with puncuation and internal whitespace.
    #[clap(short = 'L', long = "remove-non-latin-alphabetic")]
    remove_non_latin_alphabetic: bool,

    /// Remove all words that have any non-ASCII characters from new list
    #[clap(short = 'C', long = "remove-nonascii")]
    remove_nonascii: bool,

    /// Remove all words with integers in them from list
    #[clap(short = 'I', long = "remove-integers")]
    remove_integers: bool,

    /// Delete all integers from all words on new list
    #[clap(short = 'i', long = "delete-integers")]
    delete_integers: bool,

    /// Delete all characters after the first instance of the specified delimiter until the end of line
    /// (including the delimiter). Delimiter must be a single character (e.g., ','). Use 't' for tab and
    /// 's' for space. May not be used together with -g or -G options.
    #[clap(short = 'd', long = "delete-after")]
    delete_after_delimiter: Option<char>,

    /// Delete all characters before and including the first instance of the specified delimiter. Delimiter
    /// must be a single character (e.g., ','). Use 't' for tab and 's' for space. May not be used
    /// together with -g or -G options.
    #[clap(short = 'D', long = "delete-before")]
    delete_before_delimiter: Option<char>,

    /// Only take first N words from inputted word list.
    /// If two or more word lists are inputted, it will
    /// combine arbitrarily and then take first N words.
    #[clap(long = "take-first")]
    take_first: Option<usize>,

    /// Only take a random N number of words from inputted word list.
    /// If two or more word lists are inputted, it will
    /// combine arbitrarily and then take a random N words. If you're looking to cut a list exactly
    /// to a specified size, consider print-rand or whittle-to options.
    #[clap(long = "take-rand")]
    take_rand: Option<usize>,

    /// Whittle list exactly to a specified length, only taking minimum number of words
    /// from the beginning of inputted list(s).
    /// If the outputted list is not exactly the specified length, it will try again by taking a
    /// different amount of words form input list(s). As a result, this using this option may cause
    /// Tidy to take a moment to produce the finished list.
    /// Can accept expressions in the form of base**exponent (helpful for generating diceware lists).
    ///
    /// This option should generally only be used if the following conditions are met:
    ///
    /// (a) the inputted word list is sorted by desirability (e.g. ordered by word frequency);
    /// (b) the user is either removing prefix words, removing suffix words, and/or doing a Schlinkert prune;
    /// (c) the user needs the resulting list to be an exact length.
    ///
    /// Optionally can also take and a rough "starting point", after a comma.
    /// For example, --whittle-to 7776,15000 would start by taking the first
    /// 15,000 words from the inputted list(s), then keep iterating from there.
    #[clap(short = 'W', long = "whittle-to")]
    whittle_to: Option<String>,

    /// Just before printing generated list, cut list down
    /// to a set number of words. Can accept expressions in the
    /// form of base**exponent (helpful for generating diceware lists).
    /// Cuts are done randomly.
    #[clap(long = "print-rand", parse(from_str = eval_list_length))]
    print_rand: Option<usize>,

    /// Just before printing generated list, cut list down
    /// to a set number of words. Can accept expressions in the
    /// form of base**exponent (helpful for generating diceware lists).
    /// Words are selected from the beginning of processed list, and before it is sorted alphabetically.
    #[clap(long = "print-first", parse(from_str = eval_list_length))]
    print_first: Option<usize>,

    /// Set minimum word length
    #[clap(short = 'm', long = "minimum-word-length")]
    minimum_length: Option<usize>,

    /// Set maximum word length
    #[clap(short = 'M', long = "maximum-word-length")]
    maximum_length: Option<usize>,

    /// Set minimum edit distance between words, which
    /// can reduce the cost of typos when entering words
    #[clap(short = 'e', long = "minimum-edit-distance")]
    minimum_edit_distance: Option<usize>,

    /// Set number of leading characters to get to a unique prefix,
    /// which can aid auto-complete functionality.
    /// Setting this value to say, 4, means that knowing the first
    /// 4 characters of any word on the generated list is enough
    /// to know which word it is.
    #[clap(short = 'x', long = "shared-prefix-length")]
    maximum_shared_prefix_length: Option<usize>,

    /// Skip first number of lines from inputted files. Useful for dealing with headers like from
    /// PGP signatures
    #[clap(long = "skip-rows-start")]
    skip_rows_start: Option<usize>,

    /// Skip last number of lines from inputted files. Useful for dealing with footers like from
    /// PGP signatures.
    #[clap(long = "skip-rows-end")]
    skip_rows_end: Option<usize>,

    /// Path(s) for optional list of words to reject. Can accept multiple
    /// files.
    #[clap(short = 'r', long = "reject", parse(from_os_str))]
    reject_list: Option<Vec<PathBuf>>,

    /// Path(s) for optional list of approved words. Can accept multiple
    /// files.
    #[clap(short = 'a', long = "approve", parse(from_os_str))]
    approved_list: Option<Vec<PathBuf>>,

    /// Path(s) to file(s) containing homophone pairs. There must be one pair
    /// of homophones per line, separated by a comma (sun,son).
    #[clap(short = 'h', long = "homophones", parse(from_os_str))]
    homophones_list: Option<Vec<PathBuf>>,

    /// Print dice roll before word in output. Set number of sides
    /// of dice. Must be between 2 and 36. Use 6 for normal dice.
    #[clap(long = "dice")]
    dice_sides: Option<u8>,

    /// When printing dice roll before word in output, print dice values
    /// according to the base selected through --dice option. Effectively
    /// this means that letters will be used to represent numbers higher
    /// than 9. Note that this option also 0-indexes the dice values.
    /// This setting defaults to `false`, which will 1-indexed dice values,
    /// and use double-digit numbers when necessary (e.g. 18-03-08).
    #[clap(long = "sides-as-base")]
    print_dice_sides_as_their_base: bool,

    /// Path for outputted list file. If none given, generated word list
    /// will be printed to terminal.
    #[clap(short = 'o', long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Force overwrite of output file if it exists.
    #[clap(short = 'f', long = "force")]
    force_overwrite: bool,

    /// Word list input files. Can be more than one, in which case
    /// they'll be combined and de-duplicated. Requires at least
    /// one file.
    #[clap(name = "Inputted Word Lists", parse(from_os_str), required = true)]
    inputted_word_list: Vec<PathBuf>,
}

fn main() {
    let opt = Args::parse();
    if opt.debug {
        eprintln!("Received args: {:?}", opt);
    }

    if !valid_dice_sides(opt.dice_sides) {
        eprintln!("Error: Specified number of dice sides must be between 2 and 36.");
        process::exit(2);
    }

    if !valid_list_truncation_options(
        &opt.whittle_to,
        opt.print_rand,
        opt.take_first,
        opt.take_rand,
    ) {
        process::exit(2);
    }

    // Determine if this is a niche case in which whittle_to would be a smarter choice
    // than (either) print_first or print_rand.
    if (opt.print_first.is_some() || opt.print_rand.is_some())
        && opt.whittle_to.is_none()
        && (opt.remove_prefix_words || opt.remove_suffix_words || opt.schlinkert_prune)
        && !opt.quiet
    {
        if opt.print_first.is_some() {
            eprintln!("RECOMMENDATION: If your input list is sorted by desirability (e.g. word frequency), consider using --whittle-to rather than --print-first if you're removing prefix words, removing suffix words, and/or doing a Schlinkert prune.\n");
        }
        if opt.print_rand.is_some() {
            eprintln!("RECOMMENDATION: If your input list is sorted by desirability (e.g. word frequency), consider using --whittle-to rather than --print-rand if you're removing prefix words, removing suffix words, and/or doing a Schlinkert prune.\n");
        }
    }
    // Warn about the (many!) current limits of the ignore option
    let (ignore_after_delimiter, ignore_before_delimiter) = match (
        opt.ignore_after_delimiter,
        opt.ignore_before_delimiter,
    ) {
        // If given both a from_delimiter and through_delimiter, error out nicely.
        (Some(_after_delimiter), Some(_before_delimiter)) => {
            let err_message = "Can't ignore metadata on both sides.";
            eprintln!("Error: {}", err_message);
            process::exit(2);
        }
        // No ignore delimiters given, so just return None to both
        // variables.
        (None, None) => (None, None),
        // A after_delimiter given, but not a before_delimiter
        (Some(after_delimiter), None) => {
            if opt.to_lowercase
                || opt.straighten_quotes
                || opt.remove_prefix_words
                || opt.remove_suffix_words
                || opt.schlinkert_prune
                || opt.delete_nonalphanumeric
                || opt.delete_integers
                || opt.delete_before_delimiter.is_some()
                || opt.delete_after_delimiter.is_some()
                || opt.minimum_edit_distance.is_some()
                || opt.maximum_shared_prefix_length.is_some()
                || opt.homophones_list.is_some()
                || opt.dice_sides.is_some()
                || opt.print_dice_sides_as_their_base
            {
                let err_message = "--ignore-after option does not work with one of the other options you selected. Please change options. Exiting";
                eprintln!("Error: {}", err_message);
                process::exit(2);
            } else {
                (Some(after_delimiter), None)
            }
        }
        // No after_delimiter given, but a before_delimiter has been given
        (None, Some(before_delimiter)) => {
            if opt.to_lowercase
                || opt.straighten_quotes
                || opt.remove_prefix_words
                || opt.remove_suffix_words
                || opt.schlinkert_prune
                || opt.delete_nonalphanumeric
                || opt.delete_integers
                || opt.delete_after_delimiter.is_some()
                || opt.delete_before_delimiter.is_some()
                || opt.minimum_edit_distance.is_some()
                || opt.maximum_shared_prefix_length.is_some()
                || opt.homophones_list.is_some()
                || opt.dice_sides.is_some()
                || opt.print_dice_sides_as_their_base
            {
                let err_message = "--ignore-before option does not work with one of the other options you selected. Please change options. Exiting";
                eprintln!("Error: {}", err_message);
                process::exit(2);
            } else {
                (None, Some(before_delimiter))
            }
        }
    };

    let delete_after_delimiter = match opt.delete_after_delimiter {
        Some(delimiter) => parse_delimiter(delimiter),
        None => None,
    };
    let delete_before_delimiter = match opt.delete_before_delimiter {
        Some(delimiter) => parse_delimiter(delimiter),
        None => None,
    };

    // Check if output file exists
    if let Some(ref output_file_name) = opt.output {
        if !opt.force_overwrite && Path::new(output_file_name).exists() {
            eprintln!(
                "Specified output file already exists. Use --force flag to force an overwrite."
            );
            return;
        }
    }
    let tidied_list = match opt.whittle_to {
        Some(whittle_to_string) => {
            // Some whittle_to String has been provided, which we need to do a lot of work for
            // First, parse length_to_whittle_to
            let length_to_whittle_to =
                eval_list_length(split_and_vectorize(&whittle_to_string, ",")[0]);
            // Determine initial starting point
            let mut starting_point = if split_and_vectorize(&whittle_to_string, ",").len() == 2 {
                // If user gave us one, use that.
                split_and_vectorize(&whittle_to_string, ",")[1]
                    .parse::<usize>()
                    .unwrap_or((length_to_whittle_to as f64 * 1.4) as usize)
            } else {
                // If not, start with length_to_whittle_to*1.4 as a decent opening guess.
                // Effectively this assumes we'll cut about 30% of words in most
                // Tidy runs.
                (length_to_whittle_to as f64 * 1.4) as usize
            };
            // Give user a heads up that we're working on it.
            eprintln!(
                "Whittling list to {} words. This may take a moment.",
                length_to_whittle_to
            );

            let inputted_word_list = make_vec_from_filenames(
                &opt.inputted_word_list,
                opt.skip_rows_start,
                opt.skip_rows_end,
            );

            let mut this_list_length = 0;
            let mut this_tidied_list = vec![];
            while this_list_length != length_to_whittle_to {
                this_tidied_list =
                    tidy_list(TidyRequest {
                        list: inputted_word_list.clone(),
                        take_first: Some(starting_point),
                        take_rand: None, // Ignore this option in this context (whittling)
                        sort_alphabetically: !opt.no_alpha_sort,
                        ignore_after_delimiter,
                        ignore_before_delimiter,
                        to_lowercase: opt.to_lowercase,
                        should_straighten_quotes: opt.straighten_quotes,
                        should_remove_prefix_words: opt.remove_prefix_words,
                        should_remove_suffix_words: opt.remove_suffix_words,
                        should_schlinkert_prune: opt.schlinkert_prune,
                        should_remove_integers: opt.remove_integers,
                        should_delete_integers: opt.delete_integers,
                        should_remove_nonalphanumeric: opt.remove_nonalphanumeric,
                        should_delete_nonalphanumeric: opt.delete_nonalphanumeric,
                        should_remove_nonalphabetic: opt.remove_nonalphabetic,
                        should_remove_non_latin_alphabetic: opt.remove_non_latin_alphabetic,
                        should_remove_nonascii: opt.remove_nonascii,
                        should_delete_after_first_delimiter: delete_after_delimiter,
                        should_delete_before_first_delimiter: delete_before_delimiter,
                        reject_list: opt.reject_list.as_ref().map(|list_of_files| {
                            make_vec_from_filenames(list_of_files, None, None)
                        }),
                        approved_list: opt.approved_list.as_ref().map(|list_of_files| {
                            make_vec_from_filenames(list_of_files, None, None)
                        }),
                        homophones_list: opt.homophones_list.as_ref().map(|list_of_files| {
                            read_homophones_list_from_filenames(list_of_files)
                        }),
                        minimum_length: opt.minimum_length,
                        maximum_length: opt.maximum_length,
                        maximum_shared_prefix_length: opt.maximum_shared_prefix_length,
                        minimum_edit_distance: opt.minimum_edit_distance,
                        print_rand: None, // Ignore this option in this context (whittling)
                        print_first: None, // Ignore this option in this context (whittling)
                    });

                this_list_length = this_tidied_list.len();
                starting_point = get_new_starting_point_guess(
                    starting_point,
                    this_list_length,
                    length_to_whittle_to,
                );
                // Check if we're now asking for too many words from the original
                // inputted_word_list (which would be a problem!)
                if starting_point > inputted_word_list.len() {
                    eprintln!(
                        "ERROR: Cannot make a list of {} words from the inputted list(s), given the selected options. Please try again, either by changing options or inputting more words.",
                        length_to_whittle_to
                    );
                    process::exit(2);
                }

                if opt.debug {
                    eprintln!(
                        "Whittled list to {}. Will try again, taking {} words.",
                        this_list_length, starting_point
                    );
                }
            }
            // Out of the loop, which means the list is the user-specified
            // length. return this verison of the list.
            this_tidied_list
        }
        None => {
            // `whittle_to` option not specified, so proceed as normal,
            // sening all parameters in TidyRequest
            let this_tidy_request = TidyRequest {
                list: make_vec_from_filenames(
                    &opt.inputted_word_list,
                    opt.skip_rows_start,
                    opt.skip_rows_end,
                ),
                take_first: opt.take_first,
                take_rand: opt.take_rand,
                sort_alphabetically: !opt.no_alpha_sort,
                ignore_after_delimiter,
                ignore_before_delimiter,
                to_lowercase: opt.to_lowercase,
                should_straighten_quotes: opt.straighten_quotes,
                should_remove_prefix_words: opt.remove_prefix_words,
                should_remove_suffix_words: opt.remove_suffix_words,
                should_schlinkert_prune: opt.schlinkert_prune,
                should_remove_integers: opt.remove_integers,
                should_delete_integers: opt.delete_integers,
                should_remove_nonalphanumeric: opt.remove_nonalphanumeric,
                should_delete_nonalphanumeric: opt.delete_nonalphanumeric,
                should_remove_nonalphabetic: opt.remove_nonalphabetic,
                should_remove_non_latin_alphabetic: opt.remove_non_latin_alphabetic,
                should_remove_nonascii: opt.remove_nonascii,
                should_delete_after_first_delimiter: delete_after_delimiter,
                should_delete_before_first_delimiter: delete_before_delimiter,

                // If given more than one file of reject words, combine them
                // right here.
                reject_list: opt
                    .reject_list
                    .map(|list_of_files| make_vec_from_filenames(&list_of_files, None, None)),
                // Likewise with approved word lists
                approved_list: opt
                    .approved_list
                    .map(|list_of_files| make_vec_from_filenames(&list_of_files, None, None)),
                // And homophones
                homophones_list: opt
                    .homophones_list
                    .map(|list_of_files| read_homophones_list_from_filenames(&list_of_files)),
                minimum_length: opt.minimum_length,
                maximum_length: opt.maximum_length,
                maximum_shared_prefix_length: opt.maximum_shared_prefix_length,
                minimum_edit_distance: opt.minimum_edit_distance,
                print_rand: opt.print_rand,
                print_first: opt.print_first,
            };

            tidy_list(this_tidy_request)
        }
    };

    // Next, we figure out what to print where
    if !opt.dry_run {
        if !opt.quiet {
            eprintln!("Printing new list...");
        }
        match opt.output {
            Some(output) => {
                let mut f = File::create(output).expect("Unable to create file");
                for (i, word) in tidied_list.iter().enumerate() {
                    // If user set a number of dice_sides, we'll add the appropriate
                    // dice roll information, then a tab, then the word.
                    if let Some(dice_sides) = opt.dice_sides {
                        write!(
                            f,
                            "{}\t",
                            print_as_dice(
                                i,
                                dice_sides,
                                tidied_list.len(),
                                opt.print_dice_sides_as_their_base
                            ),
                        )
                        .expect("Unable to write dice roll to file");
                    }
                    // Else, just print the word
                    writeln!(f, "{}", word).expect("Unable to write word to file");
                }
            }
            // If no output file destination, print resulting list, word by word,
            // to println (which goes to stdout, allowing use of > on command line)
            None => {
                for (i, word) in tidied_list.iter().enumerate() {
                    if let Some(dice_sides) = opt.dice_sides {
                        print!(
                            "{:}\t",
                            print_as_dice(
                                i,
                                dice_sides,
                                tidied_list.len(),
                                opt.print_dice_sides_as_their_base
                            )
                        );
                    }
                    println!("{}", word);
                }
            }
        }
    }
    if !opt.quiet {
        if !opt.dry_run {
            eprintln!("\nDone making list\n");
        } else {
            eprintln!("Dry run complete");
        }
        if opt.attributes > 0 {
            display_list_information(
                &tidied_list,
                opt.attributes,
                ignore_after_delimiter,
                ignore_before_delimiter,
            );
        }
        if opt.samples {
            let samples = generate_samples(
                &tidied_list,
                ignore_after_delimiter,
                ignore_before_delimiter,
            );
            eprintln!("\nPseudorandomly generated sample passphrases");
            eprintln!("-------------------------------------------");
            for n in 0..30 {
                if n != 0 && n % 6 == 0 {
                    eprintln!();
                }
                eprint!("{} ", samples[n]);
            }
            eprintln!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_parse_print_rand() {
        assert_eq!(eval_list_length("7776"), 7776);
        assert_eq!(eval_list_length("6**5"), 7776);
        assert_eq!(eval_list_length("10000"), 10000);
        assert_eq!(eval_list_length("10**2"), 100);
    }

    #[test]
    #[should_panic]
    fn panics_when_noninteger_is_inputted_to_print_rand() {
        eval_list_length("four");
    }

    #[test]
    #[should_panic]
    fn panics_when_too_many_exponents_inputted_to_print_rand() {
        eval_list_length("2**4**3");
    }
}
