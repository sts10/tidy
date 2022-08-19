use crate::split_and_vectorize;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
/// Takes a slice of `PathBuf`s representing the word list(s)
/// that the user has inputted to the program. Then iterates
/// through each file and addes each line to Vec<String>. (Blank
/// lines and duplicate links will be handled elsewhere.)
pub fn make_vec_from_filenames(
    filenames: &[PathBuf],
    skip_rows_start: Option<usize>,
    skip_rows_end: Option<usize>,
) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    for filename in filenames {
        let f = match File::open(filename) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file {:?}: {}", filename, e),
        };
        let file = BufReader::new(&f);
        let mut raw_lines = vec![];
        for line in file.lines() {
            let l = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!(
                        "Error reading a line from file {:?}: {}\nWill continue reading file.",
                        filename, e
                    );
                    continue;
                }
            };
            raw_lines.push(l);
        }
        let number_of_lines_in_file = raw_lines.len();

        let mut line_number = 0;
        for line in raw_lines {
            match (skip_rows_start, skip_rows_end) {
                (Some(skip_rows_start), Some(skip_rows_end)) => {
                    if line_number >= skip_rows_start
                        && line_number < number_of_lines_in_file - skip_rows_end
                    {
                        word_list.push(line);
                    }
                }
                (Some(skip_rows_start), None) => {
                    if line_number >= skip_rows_start {
                        word_list.push(line);
                    }
                }
                (None, Some(skip_rows_end)) => {
                    if line_number < number_of_lines_in_file - skip_rows_end {
                        word_list.push(line);
                    }
                }
                (None, None) => word_list.push(line),
            }
            line_number += 1;
        }
    }
    word_list
}

/// Like `make_vec_from_filenames`, this function takes a slice of `PathBuf`s of
/// files. But in this case these files represent lists of homophones that the
/// user wants to make sure aren't both on the resulting list.
///
/// These homophone files are expected to be formatted such that each line of the file
/// is `homophone1,homophone2`.
///
/// This function produces a Vector of tuples of strings, representing the
/// homophone pairs.
pub fn read_homophones_list_from_filenames(filenames: &[PathBuf]) -> Vec<(String, String)> {
    let mut homophones_list: Vec<(String, String)> = vec![];
    for filename in filenames {
        let f = match File::open(filename) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file {:?}: {}", filename, e),
        };
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!(
                        "Error reading a line from file {:?}: {}\nWill continue reading file.",
                        filename, e
                    );
                    continue;
                }
            };
            let pair: (String, String) = (
                split_and_vectorize(&l, ",")[0].trim().to_string(),
                split_and_vectorize(&l, ",")[1].trim().to_string(),
            );
            homophones_list.push(pair);
        }
    }
    homophones_list
}
