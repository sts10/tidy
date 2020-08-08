# Tidy

A command-line tool for combining and cleaning large word list files. 

## What this tool can do

Given a text file with a word list, this tool will create a new word list that...
- removes whitespace
- removes empty lines
- removes duplicate lines
- sorts alphabetically

and print that new word list to a new text file.

Optionally, it can...
- combine two or more word lists
- make all characters lowercase (`-l`)
- remove prefix words (see below) (`-p`)

## Usage
```txt
USAGE:
    tidy [FLAGS] --output <output> [Inputted Word Lists]...

FLAGS:
    -h, --help             Prints help information
    -p, --remove-prefix    Remove prefix words from list
    -l, --to_lowercase     Lowercase all words
    -V, --version          Prints version information
    -v, --verbose          Prints verbose output, including parameters as received

OPTIONS:
    -o, --output <output>    Path for outputted list file

ARGS:
    <Inputted Word Lists>...    Word list input files
```

## Usage examples

- `tidy --output new_list.txt word_list1.txt word_list2.txt` Combines the word lists in `word_list1.txt` and `word_list2.txt`, removing whitespace, empty lines, and duplicate words into one list. It sorts this list alphabetically, and then prints this new, combined list to the specified output location, in this case: `new_list.txt`.

- `tidy -l -o new_list.txt inputted_word_list.txt` Removes whitespace, empty lines, and duplicate words from `inputted_word_list.txt`. Due to the `-l` flag, it makes all the words lowercase. It sorts this list alphabetically and removes duplicates once again. It then prints this new list to the specified output location, in this case: `new_list.txt`.

- `tidy -lp -o new_list.txt inputted_word_list.txt` Same as above, but the added `-p` flag removes prefix words from the list. See below for more on prefix words.

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/tidy`

## What are prefix words (aka prefix codes)? 

A word list that doesn't have any prefix words (also known as "[prefix codes](https://en.wikipedia.org/wiki/Prefix_code)") can better guarantee more consistent entropy when combining words from the list randomly and without punctuation between the words. You can read more about this issue [here](https://github.com/ulif/diceware#id3).

## Where can I find some large word lists?

- The [Electronic Frontier Foundation](https://www.eff.org/) has published [a few word lists for creating diceware passphrases](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).

- [SecureDrop](https://github.com/freedomofpress/securedrop/) has separate lists of [adjectives](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/adjectives.txt) and [nouns](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/nouns.txt).

- AgileBits, the company that makes password manager [1Password](https://1password.com/), [published a word list](https://github.com/agilebits/crackme/blob/master/doc/AgileWords.txt) in 2018.

- This diceware password generating program called ["diceware"](https://github.com/ulif/diceware) has [a few word lists](https://github.com/ulif/diceware/tree/master/diceware/wordlists) in its Github repo, along with [a separate page that explains each of the lists](https://github.com/ulif/diceware/blob/master/docs/wordlists.rst).

## To do

[ ] Add ability to filter out an inputted list of words to reject
[ ] Add a flag for removing digits from words (this would help parse diceware word lists that still have the dice numbers in them)
