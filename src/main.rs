use clap::Parser;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use tidy::*;
pub mod display_information;
pub mod input_validations;
use crate::file_readers::*;
use crate::file_writer::*;
use crate::input_validations::*;
use crate::parsers::*;

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
    #[clap(short = 'A', long = "attributes", action = clap::ArgAction::Count)]
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
    /// Note that duplicate lines and blank lines will still be removed.
    #[clap(short = 'O', long = "no-sort")]
    no_alpha_sort: bool,

    /// Normalize Unicode of all characters of all words. Accepts nfc, nfd, nfkc, or nfkd (case
    /// insensitive).
    /// May negatively affect Tidy's performance.
    #[clap(short = 'z', long = "normalization-form")]
    normalization_form: Option<String>,

    /// Specify a locale for words on the list. Aids with sorting. Examples: en-US, es-ES
    #[clap(long = "locale", default_value = "en-US")]
    locale: String,

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
    /// If two or more word lists are inputted, it will combine arbitrarily and then take first N words.
    #[clap(long = "take-first", value_parser=eval_list_length)]
    take_first: Option<usize>,

    /// Only take a random N number of words from inputted word list.
    /// If two or more word lists are inputted, it will
    /// combine arbitrarily and then take a random N words. If you're looking to cut a list exactly
    /// to a specified size, consider print-rand or whittle-to options.
    #[clap(long = "take-rand", value_parser=eval_list_length)]
    take_rand: Option<usize>,

    /// Whittle list exactly to a specified length, only taking minimum number of words
    /// from the beginning of inputted list(s).
    /// If the outputted list is not exactly the specified length, it will try again by taking a
    /// different amount of words form input list(s). As a result, this using this option may cause
    /// Tidy to take a moment to produce the finished list.
    /// Can accept expressions in the form of base**exponent (helpful for generating diceware lists).
    ///
    /// This option should generally only be used if all of the following conditions are met:
    /// (a) the inputted word list is sorted by desirability (e.g. ordered by word frequency);
    /// (b) the user is either removing prefix words, removing suffix words, or doing a Schlinkert prune;
    /// (c) the user needs the resulting list to be a specified length.
    ///
    /// Optionally can also take a "starting point" after a comma. For example,
    /// --whittle-to 7776,15000 would start by taking the first 15,000 words
    /// from the inputted list(s) as a first attempt at making a list of 7,776 words, iterating
    /// if necessary.
    #[clap(short = 'W', long = "whittle-to")]
    whittle_to: Option<String>,

    /// Just before printing generated list, cut list down
    /// to a set number of words. Can accept expressions in the
    /// form of base**exponent (helpful for generating diceware lists).
    /// Cuts are done randomly.
    #[clap(long = "print-rand", value_parser=eval_list_length)]
    print_rand: Option<usize>,

    /// Just before printing generated list, cut list down
    /// to a set number of words. Can accept expressions in the
    /// form of base**exponent (helpful for generating diceware lists).
    /// Words are selected from the beginning of processed list, and before it is sorted alphabetically.
    #[clap(long = "print-first", value_parser=eval_list_length)]
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
    #[clap(short = 'r', long = "reject")]
    reject_list: Option<Vec<PathBuf>>,

    /// Path(s) for optional list of approved words. Can accept multiple
    /// files.
    #[clap(short = 'a', long = "approve")]
    approved_list: Option<Vec<PathBuf>>,

    /// Path(s) to file(s) containing homophone pairs. There must be one pair
    /// of homophones per line, separated by a comma (sun,son).
    #[clap(long = "homophones")]
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

    /// Print playing card abbreviation next to each word.
    /// Strongly recommend only use on lists with lengths that are powers
    /// of 26 (that have 26^1, 26^2, 26^3, etc.)
    #[clap(long = "cards")]
    cards: bool,

    /// Path for outputted list file. If none given, generated word list
    /// will be printed to terminal.
    #[clap(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Force overwrite of output file if it exists.
    #[clap(short = 'f', long = "force")]
    force_overwrite: bool,

    /// Word list input files. Can be more than one, in which case
    /// they'll be combined and de-duplicated. Requires at least
    /// one file.
    #[clap(name = "Inputted Word Lists", required = true)]
    inputted_word_list: Vec<PathBuf>,
}

fn main() {
    let opt = Args::parse();
    if opt.debug {
        eprintln!("Received args: {:?}", opt);
    }

    // Some initial validations
    match validate_dice_sides(opt.dice_sides) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    if opt.cards && opt.dice_sides.is_some() {
        eprintln!("Erroro: Cannot use dice and cards. Must be either cards or dice or neither.");
        process::exit(1);
    }

    match validate_list_truncation_options(
        &opt.whittle_to,
        opt.print_rand,
        opt.take_first,
        opt.take_rand,
    ) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    // Check if output file exists
    if let Some(ref output_file_name) = opt.output {
        if !opt.force_overwrite && Path::new(output_file_name).exists() {
            eprintln!(
                "Specified output file already exists. Use --force flag to force an overwrite."
            );
            return;
        }
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

    // OK let's do this. Make a Tidy request.
    // While it's not declared as mutable here, we will reassign it
    // it later, unfortunately.
    let this_tidy_request = TidyRequest {
        list: make_vec_from_filenames(
            &opt.inputted_word_list,
            opt.skip_rows_start,
            opt.skip_rows_end,
        ),
        take_first: opt.take_first,
        take_rand: opt.take_rand,
        sort_alphabetically: !opt.no_alpha_sort,
        ignore_after_delimiter: opt.ignore_after_delimiter,
        ignore_before_delimiter: opt.ignore_before_delimiter,
        to_lowercase: opt.to_lowercase,
        normalization_form: opt.normalization_form,
        locale: opt.locale,
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
        should_delete_after_first_delimiter: opt.delete_after_delimiter,
        should_delete_before_first_delimiter: opt.delete_before_delimiter,

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

    let (ignore_before_delimiter, ignore_after_delimiter) = match validate_and_parse_ignore_options(
        &this_tidy_request,
        opt.dice_sides,
        opt.print_dice_sides_as_their_base,
    ) {
        Ok((ignore_before_delimiter, ignore_after_delimiter)) => {
            (ignore_before_delimiter, ignore_after_delimiter)
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // Parse provided "whittle string" for a length_to_whittle_to and an
    // optional starting point.
    let (mut this_tidy_request, length_to_whittle_to, starting_point) =
        match parse_whittle_options(this_tidy_request, opt.whittle_to) {
            Ok((this_tidy_request, length_to_whittle_to, starting_point)) => {
                (this_tidy_request, length_to_whittle_to, starting_point)
            }
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };

    // Finally get to actually tidy the inputted_word_list
    // If we have a length_to_whittle_to and a starting_point, we know we're
    // whittling, which is (still) a bit too complicated for my tastes. But we
    // need a while loop here.
    let mut this_list_length = 0;
    let tidied_list = match (length_to_whittle_to, starting_point) {
        (Some(our_length_to_whittle_to), Some(mut our_starting_point)) => {
            let mut this_tidied_list = vec![];
            while this_list_length != our_length_to_whittle_to {
                // Edit this_tidy_request to have our new starting point
                this_tidy_request.take_first = Some(our_starting_point);

                // This clone might be too expensice. maybe tidy_list can take a
                // reference?
                this_tidied_list = tidy_list(this_tidy_request.clone());

                this_list_length = this_tidied_list.len();
                our_starting_point = get_new_starting_point_guess(
                    our_starting_point,
                    this_list_length,
                    our_length_to_whittle_to,
                );
                if opt.debug {
                    eprintln!(
                        "Whittled list to {}. Will try again, taking {} words.",
                        this_list_length, our_starting_point
                    );
                }
            }
            // Out of the loop, which means the list is the user-specified
            // length. return this verison of the list.
            this_tidied_list
        }
        (_, _) => {
            // In all other cases, `whittle_to` option not specified, so
            // proceed as normal, sending all parameters in this_tidied_list
            // as they are just once.
            tidy_list(this_tidy_request)
        }
    };

    // Next, we figure out what to print where
    let this_print_request = PrintRequest {
        tidied_list,
        dry_run: opt.dry_run,
        quiet: opt.quiet,
        output: opt.output,
        cards: opt.cards,
        dice_sides: opt.dice_sides,
        print_dice_sides_as_their_base: opt.print_dice_sides_as_their_base,
        attributes: opt.attributes,
        samples: opt.samples,
        ignore_before_delimiter,
        ignore_after_delimiter,
    };
    print_list(this_print_request);
}
