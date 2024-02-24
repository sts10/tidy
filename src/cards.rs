use radix_fmt::*; // https://stackoverflow.com/a/50278316

/// Convert a number in a "card code" (base 26)
pub fn print_as_cards(n: usize, list_length: usize) -> String {
    let n_as_base_26 = radix(n, 26);

    // Pad card codes with zeros
    let n_width = n_as_base_26.to_string().len();
    let pad_width = radix(list_length - 1, 26).to_string().len();

    let mut padded_n = String::new();
    for _i in n_width..pad_width {
        padded_n.push('0');
    }
    // Now that we have the appropriate number of zeros
    // in `padded_n`, it's time to add our number
    padded_n += &n_as_base_26.to_string();

    padded_n
        .to_string()
        .chars()
        .map(|ch| char_to_card(ch) + "-")
        .collect::<String>()
        .trim_end_matches('-')
        .trim()
        .to_string()
}

/// Convert 0-z inputted character to a 3-character "card code"
fn char_to_card(ch: char) -> String {
    match ch {
        '0' => "B02",
        '1' => "B03",
        '2' => "B04",
        '3' => "B05",
        '4' => "B06",
        '5' => "B07",
        '6' => "B08",
        '7' => "B09",
        '8' => "B10",
        '9' => "BJa",
        'a' => "BQu",
        'b' => "BKi",
        'c' => "BAc",
        'd' => "R02",
        'e' => "R03",
        'f' => "R04",
        'g' => "R05",
        'h' => "R06",
        'i' => "R07",
        'j' => "R08",
        'k' => "R09",
        'l' => "R10",
        'm' => "RJa",
        'n' => "RQu",
        'o' => "RKi",
        'p' => "RAc",
        _ => panic!("Unable to convert this number from a letter to a card code."),
    }
    .to_string()
}
