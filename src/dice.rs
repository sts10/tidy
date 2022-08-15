
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
