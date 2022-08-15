pub fn valid_dice_sides(dice_sides: Option<u8>) -> bool {
    if let Some(dice_sides) = dice_sides {
        if !(2 <= dice_sides && dice_sides <= 36) {
            return false;
        }
    }
    true
}

pub fn valid_list_truncation_options(
    widdle_to: &Option<String>,
    cut_to: Option<usize>,
    take_first: Option<usize>,
    take_rand: Option<usize>,
) -> bool {
    // Check for invalid widdle_to requests
    if widdle_to.is_some() && cut_to.is_some() {
        eprintln!("Error: Can not specify BOTH a 'cut to' and 'widdle to' option. Please only use one of these two.");
        false
    } else if widdle_to.is_some() && (take_first.is_some() || take_rand.is_some()) {
        eprintln!("Error: Can not specify BOTH a 'widdle to' amount and a 'take first' or 'take rand' amount. Please only specify a widdle-to amount or a take amount.");
        false
    } else {
        true
    }
}
