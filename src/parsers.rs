/// Parse user's input to a handful of options, either directly as a `usize`,
/// or, if they entered Python exponent notation (base**exponent), which
/// we'll need to evaluate as an exponent. Either way, return a `usize`
/// or `expect`/`panic!`.
///  
/// This is useful when making lists fit to a specific amount of dice and
/// dice sides. (As an example, five rolls of a six-sided dice would be: 6**5).
pub fn eval_list_length(input: &str) -> Result<usize, String> {
    match input.split("**").collect::<Vec<&str>>().as_slice() {
        [] => Err("Please specify a number.".to_string()),
        [num_string] => num_string.parse::<usize>().map_err(|_| {
            format!(
                "Unable to parse input {}. Enter a number or a base**exponent",
                input
            )
        }),
        [base_string, exponent_string] => {
            let base: usize = base_string
                .parse::<usize>()
                .map_err(|_| format!("Unable to parse input {}. Positive integers only.", input))?;
            let exponent: u32 = exponent_string
                .parse::<u32>()
                .map_err(|_| format!("Unable to parse input {}. Positive integers only.", input))?;
            Ok(base.pow(exponent))
        }
        _ => Err("You can only specify one exponent! Use format: base**exponent".to_string()),
    }
}

use crate::split_and_vectorize;
use crate::TidyRequest;
use std::process;
pub fn parse_whittle_options(
    mut this_tidy_request: TidyRequest,
    whittle_to_s: Option<String>,
    inputted_word_list_length: usize,
) -> (TidyRequest, Option<usize>, Option<usize>) {
    match whittle_to_s {
        Some(whittle_to_string) => {
            // Some whittle_to String has been provided, which we need to do a lot of work for
            // First, parse length_to_whittle_to
            let length_to_whittle_to =
                eval_list_length(split_and_vectorize(&whittle_to_string, ",")[0]).unwrap();
            // Determine initial starting point
            let starting_point = if split_and_vectorize(&whittle_to_string, ",").len() == 2 {
                // If user gave us one, use that.
                split_and_vectorize(&whittle_to_string, ",")[1]
                    .parse::<usize>()
                    .unwrap_or((length_to_whittle_to as f64 * 1.4) as usize)
            } else {
                // If not, start with length_to_whittle_to*1.4 as a decent opening guess.
                // Effectively this assumes we'll cut about 40% of words in most
                // Tidy runs.
                (length_to_whittle_to as f64 * 1.4) as usize
            };
            // It's possible that our derive starting_point is higher than the length
            // of our inputted_word_list. If that's the case, reset starting_point
            // to that length.
            let starting_point = if starting_point > inputted_word_list_length {
                inputted_word_list_length as usize
            } else {
                // if not, we're good. Let given starting_point pass through.
                starting_point
            };

            // Another potential issue: User is asking for too many words, given length of
            // the inputted_word_list (which would be a problem!)
            if length_to_whittle_to > inputted_word_list_length {
                eprintln!(
                    "ERROR: Cannot make a list of {} words from the inputted list(s), given the selected options. Please try again, either by changing options or inputting more words.",
                    length_to_whittle_to
                );
                process::exit(1);
            }

            // Give user a heads up that we're working on it.
            eprintln!(
                "Whittling list to {} words. This may take a moment...",
                length_to_whittle_to
            );

            // When whittling, confidentally overwrite a few request parameters
            this_tidy_request.take_first = Some(starting_point);
            this_tidy_request.take_rand = None;
            this_tidy_request.print_rand = None;
            this_tidy_request.print_first = None;

            (
                this_tidy_request,
                Some(length_to_whittle_to),
                Some(starting_point),
            )
        }
        None => (this_tidy_request, None, None),
    }
}
