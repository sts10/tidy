use crate::dice::print_as_dice;
use crate::display_information::display_list_information;
use crate::display_information::generate_samples;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct PrintRequest {
    pub tidied_list: Vec<String>,
    pub dry_run: bool,
    pub quiet: bool,
    pub output: Option<PathBuf>,
    pub dice_sides: Option<u8>,
    pub print_dice_sides_as_their_base: bool,
    pub attributes: u8,
    pub samples: bool,
    pub ignore_before_delimiter: Option<char>,
    pub ignore_after_delimiter: Option<char>,
}

/// Print to terminal or file
pub fn print_list(print_req: PrintRequest) {
    if !print_req.quiet {
        if print_req.tidied_list.is_empty() {
            eprintln!(
                "WARNING: All words removed (tidied list is empty). Check inputted list and given options."
            );
        } else if !print_req.dry_run {
            eprintln!("Printing new list...");
        }
    }
    if !print_req.dry_run {
        match print_req.output {
            Some(output) => {
                // Print to file
                print_list_to_file(
                    &print_req.tidied_list,
                    output,
                    print_req.dice_sides,
                    print_req.print_dice_sides_as_their_base,
                );
            }
            // If no output file destination, print resulting list, word by word,
            // to println (which goes to stdout, allowing use of > on command line)
            None => {
                for (i, word) in print_req.tidied_list.iter().enumerate() {
                    if let Some(dice_sides) = print_req.dice_sides {
                        print!(
                            "{:}\t",
                            print_as_dice(
                                i,
                                dice_sides,
                                print_req.tidied_list.len(),
                                print_req.print_dice_sides_as_their_base
                            )
                        );
                    }
                    println!("{}", word);
                }
            }
        }
    }
    if !print_req.quiet {
        if !print_req.dry_run && !print_req.tidied_list.is_empty() {
            eprintln!("\nDone making list.");
        } else if print_req.dry_run {
            eprintln!("Dry run complete");
        }
        if print_req.attributes > 0 {
            display_list_information(
                &print_req.tidied_list,
                print_req.attributes,
                print_req.ignore_after_delimiter,
                print_req.ignore_before_delimiter,
            );
        }
        if print_req.samples {
            let samples = generate_samples(
                &print_req.tidied_list,
                print_req.ignore_after_delimiter,
                print_req.ignore_before_delimiter,
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
