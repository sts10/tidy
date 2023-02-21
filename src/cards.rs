use radix_fmt::*; // https://stackoverflow.com/a/50278316

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

/// Convert 0-z inputted character to a 1-indexed, padded string ("01" to "36")
fn char_to_card(ch: char) -> String {
    match ch {
        '0' => "B01",
        '1' => "B02",
        '2' => "B03",
        '3' => "B04",
        '4' => "B05",
        '5' => "B06",
        '6' => "B07",
        '7' => "B08",
        '8' => "B09",
        '9' => "B10",
        'a' => "BJa",
        'b' => "BQu",
        'c' => "BKi",
        'd' => "R01",
        'e' => "R02",
        'f' => "R03",
        'g' => "R04",
        'h' => "R05",
        'i' => "R06",
        'j' => "R07",
        'k' => "R08",
        'l' => "R09",
        'm' => "R10",
        'n' => "RJa",
        'o' => "RQu",
        'p' => "RKi",
        _ => panic!("Unable to convert this number from a letter to a card code."),
    }
    .to_string()
}
