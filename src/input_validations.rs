pub fn valid_dice_sides(dice_sides: Option<u8>) -> bool {
    if let Some(dice_sides) = dice_sides {
        if !(2 <= dice_sides && dice_sides <= 36) {
            return false;
        }
    }
    true
}

pub fn valid_list_truncation_options(
    whittle_to: &Option<String>,
    cut_to: Option<usize>,
    take_first: Option<usize>,
    take_rand: Option<usize>,
) -> bool {
    // Check for invalid whittle_to requests
    if whittle_to.is_some() && cut_to.is_some() {
        eprintln!("Error: Can not specify BOTH a 'cut to' and 'whittle to' option. Please only use one of these two.");
        false
    } else if whittle_to.is_some() && (take_first.is_some() || take_rand.is_some()) {
        eprintln!("Error: Can not specify BOTH a 'whittle to' amount and a 'take first' or 'take rand' amount. Please only specify a whittle-to amount or a take amount.");
        false
    } else {
        true
    }
}

use crate::TidyRequest;
use std::process;
pub fn validate_and_parse_ignore_options(
    this_tidy_request: &TidyRequest,
    dice_sides: Option<u8>,
    print_dice_sides_as_their_base: bool,
) -> (Option<char>, Option<char>) {
    // Warn about the (many!) current limitations of the 'ignore' options
    match (
        this_tidy_request.ignore_after_delimiter,
        this_tidy_request.ignore_before_delimiter,
    ) {
        // If given both a from_delimiter and through_delimiter, error out nicely.
        (Some(_after_delimiter), Some(_before_delimiter)) => {
            let err_message = "Can't ignore metadata on both sides.";
            eprintln!("Error: {}", err_message);
            process::exit(1);
        }
        // No ignore delimiters given, so just return None to both
        // variables.
        (None, None) => (None, None),
        // A after_delimiter given, but not a before_delimiter
        (Some(after_delimiter), None) => {
            if this_tidy_request.to_lowercase
                || this_tidy_request.should_straighten_quotes
                || this_tidy_request.should_remove_prefix_words
                || this_tidy_request.should_remove_suffix_words
                || this_tidy_request.should_schlinkert_prune
                || this_tidy_request.should_delete_nonalphanumeric
                || this_tidy_request.should_delete_integers
                || this_tidy_request
                    .should_delete_before_first_delimiter
                    .is_some()
                || this_tidy_request
                    .should_delete_after_first_delimiter
                    .is_some()
                || this_tidy_request.minimum_edit_distance.is_some()
                || this_tidy_request.maximum_shared_prefix_length.is_some()
                || this_tidy_request.homophones_list.is_some()
                || dice_sides.is_some()
                || print_dice_sides_as_their_base
            {
                let err_message = "--ignore-after option does not work with one of the other options you selected. Please change options. Exiting";
                eprintln!("Error: {}", err_message);
                process::exit(1);
            } else {
                (Some(after_delimiter), None)
            }
        }
        // No after_delimiter given, but a before_delimiter has been given
        (None, Some(before_delimiter)) => {
            if this_tidy_request.to_lowercase
                || this_tidy_request.should_straighten_quotes
                || this_tidy_request.should_remove_prefix_words
                || this_tidy_request.should_remove_suffix_words
                || this_tidy_request.should_schlinkert_prune
                || this_tidy_request.should_delete_nonalphanumeric
                || this_tidy_request.should_delete_integers
                || this_tidy_request
                    .should_delete_before_first_delimiter
                    .is_some()
                || this_tidy_request
                    .should_delete_after_first_delimiter
                    .is_some()
                || this_tidy_request.minimum_edit_distance.is_some()
                || this_tidy_request.maximum_shared_prefix_length.is_some()
                || this_tidy_request.homophones_list.is_some()
                || dice_sides.is_some()
                || print_dice_sides_as_their_base
            {
                let err_message = "--ignore-before option does not work with one of the other options you selected. Please change options. Exiting";
                eprintln!("Error: {}", err_message);
                process::exit(1);
            } else {
                (None, Some(before_delimiter))
            }
        }
    }
}
