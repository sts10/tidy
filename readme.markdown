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
- remove all words below a set character length (`-m 3`)
- remove all integers from words (`-i`)
- remove characters through first space from lines (`-s`)
- remove characters through first tab from lines (`-t`)
- remove an inputted list of words to reject (`-r`)
- only retain words from an approved list (`-a`)
- remove homophones from a provided list of pairs of homophones (`-h`)
- remove prefix words (see below) (`-p`)
- calculate and display entropy-per-word of new list (`-e`)

## Usage

```txt
tidy: Combine and clean word lists

USAGE:
    tidy [FLAGS] [OPTIONS] [Inputted Word Lists]...

FLAGS:
    -e, --entropy                 Display information about newly created list when done, including entropy-per-word
        --help                    Prints help information
    -i, --remove_integers         Remove all integers from words
    -p, --remove_prefix           Remove prefix words from list
    -s, --remove_through_space    Remove characters through first space
    -t, --remove_through_tab      Remove characters through first tab
    -l, --lowercase               Lowercase all words
    -V, --version                 Prints version information
    -v, --verbose                 Prints verbose output, including parameters as received

OPTIONS:
    -a, --approve <approved-list>         Path for optional list of approved words
    -h, --homophones <homophones-list>    Path for optional list of homophone pairs, separated by a comma
    -m, --minimum <minimum-length>        Minimum word length
    -o, --output <output>                 Path for outputted list file
    -r, --reject <reject-list>            Path for optional list of words to reject

ARGS:
    <Inputted Word Lists>...    Word list input files
```

## Usage examples

- `tidy --output new_list.txt word_list1.txt word_list2.txt` Combines the word lists in `word_list1.txt` and `word_list2.txt`, removing whitespace, empty lines, and duplicate words into one list. It sorts this list alphabetically, and then prints this new, combined list to the specified output location, in this case: `new_list.txt`.

- `tidy -l -o new_list.txt inputted_word_list.txt` Removes whitespace, empty lines, and duplicate words from `inputted_word_list.txt`. Due to the `-l` flag, it makes all the words lowercase. It sorts this list alphabetically and removes duplicates once again. It then prints this new list to the specified output location, in this case: `new_list.txt`.

- `tidy -l inputted_word_list.txt > new_list.txt` Alternatively, you can use `>` to print tidy's output to a file.

- `tidy -lp -o new_list.txt inputted_word_list.txt` Same as above, but the added `-p` flag removes prefix words from the list. See below for more on prefix words.

- `tidy -lpi -o new_list.txt inputted_word_list.txt` Same as above, but the added `-i` flag removes any integers in words. Words with integers in them are not removed, only the integers within them. For example, "11326	agency" becomes "agency". 

- `tidy -l -o new_list.txt -r bad_words.txt inputted_word_list.txt` Similar to above, but ensures that none of the words in the bad_words.txt file make it on to the final list that is printed to new_list.txt. The reject list is case sensitive.

- `tidy -l -o new_list.txt -a approved_words.txt inputted_word_list.txt` Similar to above, but ensures that only words in the approved_words.txt file make it on to the final list that is printed to new_list.txt. The approved list is case sensitive. (On Mac and some Linux distributions, `/usr/share/dict/words` should contain a list of words for spellcheck purposes.)

- `tidy -l -o new_list.txt -h homophone_pairs.txt inputted_word_list.txt` Similar to above, but expects `homophones_pairs.txt` to be a list of homophones pairs separated by a comma ("right,write" then next line: "epic,epoch"). If both words in the pair are on the inputted_word_list, Tidy will remove the second one. If only one of the words in the pair are on the list, Tidy won't remove it. Must be only two words per line.

- `tidy -le -m 3 -o new-list.txt inputted_word_list.txt` Similar to above, but the `-m 3` means new list won't have any words under 3 characters in length. Also, `-e` flag will cause program to display [entropy](https://en.wikipedia.org/wiki/Entropy_(information_theory)) information when it's done making the new list.

- `tidy -t -o just_the_words.txt diceware_list.txt` If you've got [a diceware list with numbers and a tab before each word](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt), the `-t` flag will remove everything up to and including the first tab in each line ("11133	abruptly" becomes "abruptly").

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/tidy --branch main`

## What are prefix words (aka prefix codes)? 

A word list that doesn't have any prefix words (also known as "[prefix codes](https://en.wikipedia.org/wiki/Prefix_code)") can better guarantee more consistent entropy when combining words from the list randomly and without punctuation between the words. 

As a brief example, if a list have "boy", "hood", and "boyhood" users who specified they wanted two words worth of randomness (entropy) might end up with "boyhood", which an attacker guessing single words would try. Removing prefix words -- in this case "boy" -- prevents this possibility from occurring.

You can read more about this issue [here](https://github.com/ulif/diceware#prefix-code).

## Language limitations 

As a native English speaker, I wrote this program with lists of English (US) words in mind. Unfortunately, I haven't tested it with other languages. That said, if you have ideas for how to make it more usable for other languages, please open an Issue or submit a Pull Request. 

## What's up with the memchr dependency? 

Tidy's function for removing characters through the first space or tab uses a library called [memchr](https://docs.rs/memchr/2.3.4/memchr/), which "provides heavily optimized routines for searching bytes." The optimization gained from using this crate is far from noticeable or necessary for most uses of Tidy -- using Rust's built-in `find` is not much slower -- but I figured the extra speed was worth the dependency in this case.

See [this repo](https://github.com/sts10/splitter) for more information.

## To do

- [ ] Add option to remove words that have characters from certain character sets, for example non-ASCII characters
<!-- - [ ] Investigate making the list variable as [FxHashSet](https://docs.rs/fxhash/0.2.1/fxhash/type.FxHashSet.html)<String> rather than a `Vec<String>` to potentially boost performance, as is done in [csafe](https://github.com/sts10/csafe/blob/main/src/lib.rs). Will probably need to write some benchmarks with Criterion to measure potential performance gains. -->

## Appendix: Where can I find some large word lists?

- The [Electronic Frontier Foundation](https://www.eff.org/) has published [a few word lists for creating diceware passphrases](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases). Since there's a tab between the dice numbers and each word, Tidy can remove the dice numbers easily with something like `tidy -t -o clean_eff.txt eff_large_wordlist.txt` or using the `-i` flag. The EFF also has some [fandom-inspired lists](https://www.eff.org/deeplinks/2018/08/dragon-con-diceware) that Tidy can clean up with the `-s` flag.

- [SecureDrop](https://github.com/freedomofpress/securedrop/) has separate lists of [adjectives](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/adjectives.txt) and [nouns](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/nouns.txt).

- AgileBits, the company that makes password manager [1Password](https://1password.com/), [published a word list](https://github.com/agilebits/crackme/blob/master/doc/AgileWords.txt) in 2018.

- This diceware password generating program called ["diceware"](https://github.com/ulif/diceware) seems to have collected [a few word lists](https://github.com/ulif/diceware/tree/master/diceware/wordlists) in its Github repo, along with [a separate page that explains each of the lists](https://github.com/ulif/diceware/blob/master/docs/wordlists.rst).

- [Niceware list](https://github.com/diracdeltas/niceware/blob/master/lib/wordlist.js) (~65,000 words)

- [Lists used by a program called webpassgen](https://github.com/atoponce/webpassgen/tree/master/lists)

- [Magic Wormhole](https://github.com/magic-wormhole/magic-wormhole/), a tool for transferring files, uses [a version of the PGP Word List](https://github.com/magic-wormhole/magic-wormhole/blob/master/src/wormhole/_wordlist.py), which specifically tries to use pairs of words that are phonetically distinct.

- [Mnemonicode](https://github.com/schollz/mnemonicode/blob/master/word_list.go) is another word list optimized for pronunciation. I believe [croc](https://github.com/schollz/croc), another file transferring tool, uses it.

- A collection of a few [Public Domain Word Lists](https://github.com/MichaelWehar/Public-Domain-Word-Lists)

- [r/wordlists subreddit](https://www.reddit.com/r/wordlists/), which seems to have links to a few non-English word lists.

- Shameless plug: I created [a ~16k word list](https://github.com/sts10/common_word_list_maker/blob/main/example_word_list.txt) using [Google Books Ngram data](https://github.com/sts10/common_word_list_maker).
