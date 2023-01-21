use crate::dice::print_as_dice;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn print_list_to_file(
    tidied_list: &[String],
    output: PathBuf,
    dice_sides: Option<u8>,
    print_dice_sides_as_their_base: bool,
) {
    let mut f = File::create(output).expect("Unable to create file");
    for (i, word) in tidied_list.iter().enumerate() {
        // If user set a number of dice_sides, we'll add the appropriate
        // dice roll information, then a tab, then the word.
        if let Some(dice_sides) = dice_sides {
            write!(
                f,
                "{}\t",
                print_as_dice(
                    i,
                    dice_sides,
                    tidied_list.len(),
                    print_dice_sides_as_their_base
                ),
            )
            .expect("Unable to write dice roll to file");
        }
        // Else, just print the word
        writeln!(f, "{}", word).expect("Unable to write word to file");
    }
}
