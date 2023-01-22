use crate::dice::print_as_dice;
use crate::display_information::display_list_information;
use crate::display_information::generate_samples;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Print to terminal or file
pub fn print_list(
    tidied_list: Vec<String>,
    dry_run: bool,
    quiet: bool,
    output: Option<PathBuf>,
    dice_sides: Option<u8>,
    print_dice_sides_as_their_base: bool,
    attributes: u8,
    samples: bool,
    ignore_before_delimiter: Option<char>,
    ignore_after_delimiter: Option<char>,
) {
    if !dry_run {
        if !quiet {
            eprintln!("Printing new list...");
        }
        match output {
            Some(output) => {
                // Print to file
                print_list_to_file(
                    &tidied_list,
                    output,
                    dice_sides,
                    print_dice_sides_as_their_base,
                );
            }
            // If no output file destination, print resulting list, word by word,
            // to println (which goes to stdout, allowing use of > on command line)
            None => {
                for (i, word) in tidied_list.iter().enumerate() {
                    if let Some(dice_sides) = dice_sides {
                        print!(
                            "{:}\t",
                            print_as_dice(
                                i,
                                dice_sides,
                                tidied_list.len(),
                                print_dice_sides_as_their_base
                            )
                        );
                    }
                    println!("{}", word);
                }
            }
        }
    }
    if !quiet {
        if !dry_run {
            eprintln!("\nDone making list\n");
        } else {
            eprintln!("Dry run complete");
        }
        if attributes > 0 {
            display_list_information(
                &tidied_list,
                attributes,
                ignore_after_delimiter,
                ignore_before_delimiter,
            );
        }
        if samples {
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

fn print_list_to_file(
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
