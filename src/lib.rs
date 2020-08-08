use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

pub fn make_vec(filenames: &[PathBuf]) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    for filename in filenames {
        let f = File::open(filename).unwrap();
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = line.unwrap();
            word_list.push(l);
        }
    }
    word_list
}

pub fn tidy_list(
    list: Vec<String>,
    to_lowercase: bool,
    should_remove_prefix_words: bool,
) -> Vec<String> {
    let mut tidied_list = trim_whitespace(&list);
    tidied_list = remove_blank_lines(&tidied_list);
    tidied_list = sort_and_dedup(&mut tidied_list);
    tidied_list = if to_lowercase {
        tidied_list.iter().map(|w| w.to_ascii_lowercase()).collect()
    } else {
        tidied_list
    };
    tidied_list = if should_remove_prefix_words {
        remove_prefix_words(sort_and_dedup(&mut tidied_list))
    } else {
        tidied_list
    };
    tidied_list = sort_and_dedup(&mut tidied_list);
    tidied_list
}

fn remove_blank_lines(list: &[String]) -> Vec<String> {
    let mut new_list: Vec<String> = [].to_vec();
    for word in list {
        if word != "" {
            new_list.push(word.to_string());
        }
    }
    new_list
}
fn trim_whitespace(list: &[String]) -> Vec<String> {
    let mut new_list: Vec<String> = [].to_vec();
    for word in list {
        if word != "" {
            new_list.push(word.trim_start().trim_end().to_string());
        }
    }
    new_list
}

fn sort_and_dedup(list: &mut Vec<String>) -> Vec<String> {
    list.sort();
    list.dedup();
    list.to_vec()
}

fn remove_prefix_words(list: Vec<String>) -> Vec<String> {
    remove_by_position(find_prefix_words_to_remove(&list), list)
}

fn find_prefix_words_to_remove(list: &[String]) -> Vec<usize> {
    // wonder if I can make this easier to read and/or more efficient if I use starts_with
    let mut prefixes_to_remove: Vec<usize> = Vec::new();
    for word in list {
        let length = word.len();
        for i in 0..length - 1 {
            let prefix_to_check = word.split_at(i + 1).0;
            let prefix_position: Option<usize> = list.iter().position(|x| x == prefix_to_check);
            if let Some(pos) = prefix_position {
                prefixes_to_remove.push(pos);
            }
        }
    }
    prefixes_to_remove
}

fn remove_by_position(positions_to_remove: Vec<usize>, list: Vec<String>) -> Vec<String> {
    let mut cleaned_list: Vec<String> = Vec::new();
    for (i, word) in list.into_iter().enumerate() {
        if !positions_to_remove.contains(&i) {
            cleaned_list.push(word);
        }
    }
    cleaned_list
}
