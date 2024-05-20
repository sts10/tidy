# Tidy

A command-line tool for combining and cleaning large word list files.

> A throw of the dice will never abolish chance. — Stéphane Mallarmé

## What this tool aims to help users do

Tidy aims to help users create "_better_" word lists -- generally word lists that will be used to create passphrases like "block insoluble cardinal discounts".

Tidy performs basic list-cleaning operations like removing duplicate words and blank lines by default. It additionally provides various optional standardizations and filters, like lowercasing all words (`-l`), or removing words in with integers in them (`-I`), as well as protections against rare-but-possible passphrase pitfalls, such as prefix codes (`-P`) and low minimum word lengths (see below for explanations).

Tidy also can make word lists more "typo-resistant" by enforcing a minimum edit distance (`-d`), removing homophones and/or enforcing a unique prefix length (`-x`), which can allow users to auto-complete words after a specified number of characters.

Tidy can be used to **create new word lists** (for example, if given more than one list, Tidy will combine and de-duplicate them) with desirable qualities. You can obviously **edit** existing word lists.

### Other resources
* If you want to _audit_ an existing word list without editing it, Tidy can do that, but I'd suggest using my related [Word List Auditor](https://github.com/sts10/wla).
* If you just want some word lists, you can check out my [Orchard Street Wordlists](https://github.com/sts10/orchard-street-wordlists).

## Tidy's features

Given a text file with one word per line, this tool will create a new word list in which...

-   duplicate lines (words) are removed
-   empty lines have been removed
-   whitespace from beginning and end of words is deleted
-   words are sorted alphabetically (though this can be optionally prevented -- see below)

and print that new word list to the terminal or to a text file.

Optionally, the tool can...

-   combine two or more inputted word lists
-   make all characters lowercase (`-l`)
-   set a minimum and maximum for word lengths
-   handle words with integers and non-alphanumeric characters
-   delete all characters before or after a delimiter (`-d`/`-D`)
-   take lists of words to reject or allow
-   remove homophones from a provided list of comma-separated pairs of homophones
-   enforce a minimum [edit distance](https://en.wikipedia.org/wiki/Edit_distance) between words
-   remove prefix words (see below) (`-P`)
-   remove suffix words (`-S`)
-   remove all words with non-alphabetic characters from new list
-   straighten curly/smart quotes, i.e. replacing them with their "straight" equivalents (`-q`)
-   guarantee a maximum shared prefix length (see below) (`-x`)
-   normalize Unicode of all characters of all words on list to a specified [normalization form](https://www.unicode.org/faq/normalization.html) (NFC, NFKD, etc.) (`-z`)
-   print corresponding dice rolls before words, separated by a tab. Dice can have 2 to 36 sides. (`--dice`)
-   print information about the new list, such as entropy per word, to the terminal (`-A`, `-AA`, `-AAA`, or `-AAAA` depending on how much information you want to printed)

and more!

NOTE: If you do NOT want Tidy to sort list alphabetically, you can use the `--no-sort` option.

## Usage

```txt
Usage: tidy [OPTIONS] <Inputted Word Lists>...

Arguments:
  <Inputted Word Lists>...
          Word list input files. Can be more than one, in which case they'll be
          combined and de-duplicated. Requires at least one file

Options:
  -a, --approve <APPROVED_LIST>
          Path(s) for optional list of approved words. Can accept multiple files

  -A, --attributes...
          Print attributes about new list to terminal. Can be used more than once to
          print more attributes. Some attributes may take a nontrivial amount of time
          to calculate

  -j, --json
          Print attributes and word samples in JSON format

      --cards
          Print playing card abbreviation next to each word. Strongly recommend only
          using on lists with lengths that are powers of 26 (26^1, 26^2, 26^3, etc.)

      --debug
          Debug mode

  -d, --delete-after <DELETE_AFTER_DELIMITER>
          Delete all characters after the first instance of the specified delimiter
          until the end of line (including the delimiter). Delimiter must be a single
          character (e.g., ','). Use 't' for tab and 's' for space. May not be used
          together with -g or -G options

  -D, --delete-before <DELETE_BEFORE_DELIMITER>
          Delete all characters before and including the first instance of the specified
          delimiter. Delimiter must be a single character (e.g., ','). Use 't' for tab
          and 's' for space. May not be used together with -g or -G options

  -i, --delete-integers
          Delete all integers from all words on new list

  -n, --delete-nonalphanumeric
          Delete all non-alphanumeric characters from all words on new list. Characters
          with diacritics will remain

      --dice <DICE_SIDES>
          Print dice roll before word in output. Set number of sides of dice. Must be
          between 2 and 36. Use 6 for normal dice

      --dry-run
          Dry run. Don't write new list to file or terminal

  -f, --force
          Force overwrite of output file if it exists

      --homophones <HOMOPHONES_LIST>
          Path(s) to file(s) containing homophone pairs. There must be one pair of
          homophones per line, separated by a comma (sun,son). If BOTH words are found
          on a list, the SECOND word is removed. File(s) can be a CSV (with no column
          headers) or TXT file(s)

  -g, --ignore-after <IGNORE_AFTER_DELIMITER>
          Ignore characters after the first instance of the specified delimiter until the
          end of line, treating anything before the delimiter as a word. Delimiter must be
          a single character (e.g., ','). Use 't' for tab and 's' for space. Helpful for
          ignoring metadata like word frequencies. Works with attribute analysis and most
          word removal options, but not with word modifications (like to lowercase).
          May not be used together with -d, -D or -G options

  -G, --ignore-before <IGNORE_BEFORE_DELIMITER>
          Ignore characters before and including the first instance of the specified
          delimiter, treating anything after the delimiter as a word. Delimiter must
          be a single character (e.g., ','). Use 't' for tab and 's' for space. Helpful
          for ignoring metadata like word frequencies. Works with attribute analysis
          and most word removal options, but not with word modifications (like to lowercase).
          May not be used together with -d, -D or -g options

      --locale <LOCALE>
          Specify a locale for words on the list. Aids with sorting. Examples: en-US,
          es-ES. Defaults to system LANG. If LANG environmental variable is not set,
          uses en-US

  -l, --lowercase
          Lowercase all words on new list

  -M, --maximum-word-length <MAXIMUM_LENGTH>
          Set maximum word length

  -x, --shared-prefix-length <MAXIMUM_SHARED_PREFIX_LENGTH>
          Set number of leading characters to get to a unique prefix, which can aid
          auto-complete functionality. Setting this value to say, 4, means that knowing
          the first 4 characters of any word on the generated list is enough to know
          which word it is

  -e, --minimum-edit-distance <MINIMUM_EDIT_DISTANCE>
          Set minimum edit distance between words, which can reduce the cost of typos
          when entering words

  -m, --minimum-word-length <MINIMUM_LENGTH>
          Set minimum word length

  -O, --no-sort
          Do NOT sort outputted list alphabetically. Preserves original list order. Note
          that duplicate lines and blank lines will still be removed

  -z, --normalization-form <NORMALIZATION_FORM>
          Normalize Unicode of all characters of all words. Accepts nfc, nfd, nfkc,
          or nfkd (case insensitive)

  -o, --output <OUTPUT>
          Path for outputted list file. If none given, generated word list will be printed
          to terminal

      --sides-as-base
          When printing dice roll before word in output, print dice values according to
          the base selected through --dice option. Effectively this means that letters will
          be used to represent numbers higher than 9. Note that this option also 0-indexes
          the dice values. This setting defaults to `false`, which will 1-indexed
          dice values, and use double-digit numbers when necessary (e.g. 18-03-08)

      --print-first <PRINT_FIRST>
          Just before printing generated list, cut list down to a set number of
          words. Can accept expressions in the form of base**exponent (helpful
          for generating diceware lists). Words are selected from the beginning
          of processed list, and before it is sorted alphabetically

      --print-rand <PRINT_RAND>
          Just before printing generated list, cut list down to a set number of words.
          Can accept expressions in the form of base**exponent (helpful for generating
          diceware lists). Cuts are done randomly

      --quiet
          Do not print any extra information

  -I, --remove-integers
          Remove all words with integers in them from list

  -N, --remove-nonalphanumeric
          Remove all words with non-alphanumeric characters from new list. Words
          with diacritics will remain

      --remove-nonalphabetic
          Remove all words with non-alphabetic characters from new list. Words with
          diacritcis and other non-Latin characters will remain

  -L, --remove-non-latin-alphabetic
          Remove all words with any characters not in the Latin alphabet (A through
          Z and a through z). All words with accented or diacritic characters
          will be removed, as well as any words with puncuation and internal whitespace

  -C, --remove-nonascii
          Remove all words that have any non-ASCII characters from new list

  -P, --remove-prefix
          Remove prefix words from new list

  -S, --remove-suffix
          Remove suffix words from new list

  -r, --reject <REJECT_LIST>
          Path(s) for optional list of words to reject. Can accept multiple files

  -s, --samples
          Print a handful of pseudorandomly selected words from the created list
          to the terminal. Should NOT be used as secure passphrases

  -K, --schlinkert-prune
          Use Sardinas-Patterson algorithm to remove words to make list
          uniquely decodable. Experimental!

      --skip-rows-start <SKIP_ROWS_START>
          Skip first number of lines from inputted files. Useful for dealing
          with headers like from PGP signatures

      --skip-rows-end <SKIP_ROWS_END>
          Skip last number of lines from inputted files. Useful for dealing
          with footers like from PGP signatures

  -q, --straighten
          Replace “smart” quotation marks, both “double” and ‘single’, with
          their "straight" versions

      --take-first <TAKE_FIRST>
          Only take first N words from inputted word list. If two or more word
          list files are inputted, it will combine all given lists by alternating words
          from the given word list files until it has N words

      --take-rand <TAKE_RAND>
          Only take a random N number of words from inputted word list. If two or more
          word lists are inputted, it will combine arbitrarily and then take a random
          N words. If you're looking to cut a list exactly to a specified size,
          consider print-rand or whittle-to options

  -W, --whittle-to <WHITTLE_TO>
          Whittle list exactly to a specified length, only taking minimum number
          of words from the beginning of inputted list(s). If the outputted list
          is not exactly the specified length, it will try again by taking a
          different amount of words form input list(s). As a result, this using this
          option may cause Tidy to take a moment to produce the finished list. Can
          accept expressions in the form of base**exponent (helpful for generating
          diceware lists).

          This option should generally only be used if all of the following conditions
          are met: (a) the inputted word list is sorted by desirability (e.g. ordered
          by word frequency); (b) the user is either removing prefix words, removing
          suffix words, or doing a Schlinkert prune; (c) the user needs the resulting
          list to be a specified length.

          Optionally can also take a "starting point" after a comma. For example,
          --whittle-to 7776,15000 would start by taking the first 15,000 words from
          the inputted list(s) as a first attempt at making a list of 7,776 words,
          iterating if necessary.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Usage examples

-   `tidy --output new_list.txt word_list1.txt word_list2.txt` Combines the word lists in `word_list1.txt` and `word_list2.txt`, removing whitespace, empty lines, and duplicate words into one list. It sorts this list alphabetically, and then prints this new, combined list to the specified output location, in this case: `new_list.txt`.

-   `tidy -l -o new_list.txt inputted_word_list.txt` Deletes whitespace, removes empty lines and duplicate words from `inputted_word_list.txt`. Due to the `-l` flag, it makes all the words lowercase. It sorts this list alphabetically and removes duplicates once again. It then prints this new list to the specified output location, in this case: `new_list.txt`.

-   `tidy -l inputted_word_list.txt > new_list.txt` Alternatively, you can use `>` to print tidy's output to a file.

-   `tidy -lP -o new_list.txt inputted_word_list.txt` Same as above, but the added `-P` flag removes prefix words from the list. See below for more on prefix words.

-   `tidy -lPi -o new_list.txt inputted_word_list.txt` Same as above, but the added `-i` flag deletes any integers in words. Words with integers in them are not removed, only the integers within them. For example, "11326 agency" becomes "agency".

-   `tidy -lPiO -o new_list.txt inputted_word_list.txt` Same as above, but the added `-O` flag preserves the original order of the list, rather than sort it alphabetically. Note that duplicates and blank lines are still removed.

-   `tidy -I -o new_list.txt inputted_word_list.txt` Using the `-I` flag removes any words with integers from the list. For example, "hello1" would be completely removed from the list, since it has an integer in it. Note that this is distinct from the lowercase `-i` flag, which would leave the word "hello" on the resulting list (removing the "1").

-   `tidy -AA -I -o new_list.txt inputted_word_list.txt` Adding `-AA` prints some information about the created list to the terminal. You can add up to 4 `A` flags to get the maximum amount of information that Tidy can print about a list. See below for more information.

-   `tidy -l -o new_list.txt -r profane_words.txt inputted_word_list.txt` Similar to above, but ensures that none of the words in the profane_words.txt file make it on to the final list that is printed to new_list.txt. The reject list is case sensitive, so you may want to run it through tidy using the -l flag before using it. (You can find lists of profane words [here](https://github.com/LDNOOBW/List-of-Dirty-Naughty-Obscene-and-Otherwise-Bad-Words) and [here](https://code.google.com/archive/p/badwordslist/downloads).)

-   `tidy -l -o new_list.txt -a approved_words.txt inputted_word_list.txt` Similar to above, but ensures that only words in the approved_words.txt file make it on to the final list that is printed to new_list.txt. The approved list is case sensitive. (On Mac and some Linux distributions, `/usr/share/dict/words` should contain a list of words for spellcheck purposes.)

-   `tidy -l -o new_list.txt --homophones homophone_pairs.txt inputted_word_list.txt` Similar to above, but expects `homophones_pairs.txt` to be a list of homophones pairs separated by a comma ("right,write" then next line: "epic,epoch"). If both words in the pair are on the inputted_word_list, Tidy will remove the second one. If only one of the words in the pair are on the list, Tidy won't remove it. Must be only two words per line.

-   `tidy -lA -m 3 -o new-list.txt inputted_word_list.txt` Similar to above, but the `-m 3` means new list won't have any words under 3 characters in length. Have Tidy also print some attributes about the new list to the terminal screen.

-   `tidy -z nfkd --locale fr -o bip-0039/french.txt --force bip-0039/french.txt` Verify that [the BIP-0039 French list](https://github.com/bitcoin/bips/blob/master/bip-0039/french.txt) is (a) normalized to [Unicode Normalization Form](https://www.unicode.org/reports/tr15/) Compatibility Decomposition (abbreviated as NFKD) (as per [the BIP-0039 specification](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki#wordlist)) and (b) sorted appropriately for the French language (thanks to specifying `--locale fr`). Locales can also be specified like "en-US" or "es-ES". If a `locale` is not specified, locale uses system LANG. If no LANG is found, uses "en-US". This locale setting only really affects how the words on the outputted list are **sorted**, so it's not _crucial_ for most use-cases to specify one.

-   `tidy -d t -o just_the_words.txt diceware_list.txt` If you've got [a diceware list with numbers and a tab before each word](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt), the `-d t` flag will delete everything up to and including the first tab in each line ("11133 abruptly" becomes "abruptly").

-   `tidy --dice 6 -o diceware_list.txt just_words.txt` Add corresponding dice roll numbers to a list with `--dice`. Can accept dice sides between 2 and 36. Each dice roll and word are separated by a tab.

-   `tidy -P -x 4 --print-rand 7776 --dice 6 --output diceware.txt 1password-2021.txt` Make a 7,776-word list from a [1Password (~18k) word list](https://1password.com/txt/agwordlist.txt), removing prefix words and guaranteeing 4 characters can auto-complete any word. Lastly, add corresponding 6-sided dice role for each word.

-   `tidy -o d-and-d.txt --dice 20 --print-rand 20**3 wordlist.txt` Create an 8,000-word list where each word corresponds to 3 rolls of a 20-sided die (`06-07-07	dragon`). `--print-rand` randomly truncates the resulting list to the specified amount -- can accept integers (`8000`) or informal exponent notation (`20**3`).

-   `tidy -d s --whittle-to 7776 -PlL -m 3 -M 12 --dice 6 -o wiki-diceware.txt ~/Downloads/enwiki-20190320-words-frequency-sorted.txt` Carefully make a 7,776-word list by only taking the words needed from the top of `~/Downloads/enwiki-20190320-words-frequency-sorted.txt` [file](https://github.com/IlyaSemenov/wikipedia-word-frequency/blob/master/results/enwiki-20190320-words-frequency.txt). Assumes this file is sorted by word frequencies, with a frequency count after the word, separated by a space (example line: `located 1039008`). Since we only want to use the most common words, we'll use Tidy's `--whittle-to` option to only take exactly how many words we need to construct a list of 7,776 words. Note that this may take longer that usual Tidy executions, since Tidy will very likely need to make multiple attempts to make a list that's exactly the requested length. [More info on whittle](https://github.com/sts10/tidy/issues/15#issuecomment-1215907335).

## Installation

### Using Rust and cargo
1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/tidy --locked --branch main` (Run this same command to upgrade Tidy.)

You should then be able to run `tidy --help` for help text.

Uninstall Tidy by running `cargo uninstall tidy`.

### Releases
Check the [GitHub Releases page](https://github.com/sts10/tidy/releases) for binaries suitable for Mac, Windows, and Linux users.

To install the executable on a Linux/macOS machine, download the `tidy` executable and move it to somewhere in your `$PATH`, like `$HOME/.local/bin` (you can do this on the command line with something like `mv ~/Downloads/tidy ~/.local/bin/`).

## For Tidy developers

* Run all code tests: `cargo test`
* Generate docs: `cargo doc --document-private-items --no-deps`. Add `--open` flag to open docs after generation. Locally, docs are printed to `./target/doc/tidy/index.html`.
* Check license compatibility of Tidy's dependencies: `cargo deny check licenses` (requires that you [have cargo-deny installed locally](https://github.com/EmbarkStudios/cargo-deny#install-cargo-deny))

Pull Requests welcome!

## Tidy can print attributes about a word list

**Note when using Tidy to audit a list**: Tidy will remove blank lines and duplicate lines (words) _before_ calculating these list attributes. For example, if you're 4,000-word list has, say, 5 duplicate words, Tidy will report that the list has 3,995 words. No warning of duplicate words is given.

If you really want to _audit_ a word list, without making changes to it, try [Word List Auditor](https://github.com/sts10/wla).

That said, Tidy can calculate different attributes about a created list. `tidy -AAAA -G t --dry-run eff_long_list.txt` prints:

```text
Attributes of new list
----------------------
List length               : 7776 words
Mean word length          : 6.99 characters
Length of shortest word   : 3 characters (aim)
Length of longest word    : 9 characters (zoologist)
Free of prefix words?     : true
Free of suffix words?     : false
Uniquely decodable?       : true
Entropy per word          : 12.925 bits
Efficiency per character  : 1.849 bits
Assumed entropy per char  : 4.308 bits
Above brute force line?   : true
Shortest edit distance    : 1
Mean edit distance        : 6.858
Longest shared prefix     : 8
Unique character prefix   : 9
```

Using the `--samples` flag will print 5 sample passphrases to the terminal. (Note that these sample passphrases should not be used for security purposes, as Tidy has not been audited.)

```txt
Word samples
------------
destruct subpar dizzy outshine stipend ovary
slapstick hastily tremor visibly gizzard unloaded
salaried unwieldy churn vanity speak vessel
deserve humble pantyhose dayroom reprise unnatural
vascular stencil visible sporty embellish submarine
```

## How Tidy counts the length of a word

When counting the length of a word, Tidy counts the number of [grapheme clusters](https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries) in the word. Generally, less common characters like accented letters and emoji all count as 1 grapheme cluster and thus, to Tidy, one character. I believe this better fits with how us humans intuitively count characters in a string/word.

## What types of files does Tidy work with?
In general, Tidy expects inputted files to have one word per line.

### Line endings
Tidy supports `\n` and `\r\n` line endings.

## On verbs used

In both Tidy's code and documentation, "remove" means that a word will be removed (e.g. words with integers will be removed from the list), while "delete" means that a word will only be modified (e.g. integers removed from words). Uppercase flags remove words, while lowercase flags delete specified characters. All delete calls and word modifications (like "to lowercase") occur _before_ any remove call.

## Blog posts related to this project

* [Read about how Tidy handles Unicode normalization, locales, and alphabetizing words](https://sts10.github.io/2023/01/29/sorting-words-alphabetically-rust.html)
* [Read more about the 0.2 version of this project](https://sts10.github.io/2021/12/09/tidy-0-2-0.html)
* [Read about uniquely decodable codes and "Schlinkert pruning"](https://sts10.github.io/2022/08/12/efficiently-pruning-until-uniquely-decodable.html) (introduced in Tidy version 0.2.60)
* [Read about initial inspiration for the project](https://sts10.github.io/2020/09/30/making-a-word-list.html)

## Using Tidy with non-English words and/or accented characters

Tidy does its best to work well with all languages. That said, I'm an English speaker and have not tested Tidy with other languages all that much.

There are a few steps you can take to help Tidy produce a good word list in all languages.

If you're using Tidy to work a word list with accented characters, it is highly recommended that you:
1. have Tidy normalize the Unicode of all characters on the list (e.g. `-z nfc` or `-z nfkd`). This will better ensure that there are no duplicate-looking words on the list, which could cause Tidy and others to over-estimate the strength of passphrases generated from the outputted list. Note that if you're passing a reject list file or approved list file to Tidy, you should normalize those lists _before_ using them. For example: `tidy -z nfc --locale ES-es -l --force -o profane-spanish-words.txt profane-spanish-words.txt && tidy -z nfc --locale ES-es -r profane-spanish-words.txt -o my-new-spanish-word-list.txt -l a-bunch-of-spanish-words.txt`
2. specify the "locale" of the words on your list (e.g. `--locale fr` or `--locale ES-es`). This will ensure that the outputted list is sorted correctly.
3. if the language you're working with has or may have apostrophes in words, consider using the `-q` or `--straighten` option to standardize these characters across all words on the new list.

See [this blog post](https://sts10.github.io/2023/01/29/sorting-words-alphabetically-rust.html) for more. If you find Tidy not performing as expected with non-English words, please open an Issue on this repository with an example.

## Using Tidy to remove homophones

If passphrases from your list will ever be spoken out loud, you may want to consider removing homophones -- words that sound alike -- from your list.

I'd say that Tidy offers two ways of dealing with homophones.

Given a pair of homophones, like "sun" and "son":

1. To ensure you don't have BOTH homophones in your generated list, you'd run `tidy` with a flag like `--homophones ../homophones/homophone-lists/homophones-large-as-pairs.txt` ([link](https://github.com/sts10/homophones/blob/main/homophone-lists/homophones-large-as-pairs.txt)). This will let either "sun" or "son" on your list but NOT both.
2. To ensure you have NEITHER of the words in the homophone pair on your generated word list, you'd use the reject words flags: `-r ../homophones/homophone-lists/cleaned-as-singles.txt` ([link](https://github.com/sts10/homophones/blob/main/homophone-lists/cleaned-as-singles.txt)). This will remove _both_ "sun" and "son" from your generated list before its outputted.

If you're looking for a relatively long list of English homophones, I'd humbly point you to [this other project of mine](https://github.com/sts10/homophones).

## Prefix codes, suffix codes, and uniquely decodable codes

If a word list is "uniquely decodable" that means that words from the list can be safely combined _without_ a delimiter between each word, e.g. `enticingneurosistriflecubeshiningdupe`.

As a brief example, if a list has "boy", "hood", and "boyhood" on it, users who specified they wanted two words worth of randomness (entropy) might end up with "boyhood", which an attacker guessing single words would try. Removing the word "boy", which makes the remaining list uniquely decodable, prevents this possibility from occurring.

To make a list uniquely decodable, Tidy removes words. Tidy offers three (3) distinct procedures to make cuts until a list is uniquely decodable. Users can (1) remove all [prefix words](https://en.wikipedia.org/wiki/Prefix_code), (2) remove all suffix words, or (3) perform "Schlinkert pruning," a procedure based on [the Sardinas–Patterson algorithm](https://en.wikipedia.org/wiki/Sardinas%E2%80%93Patterson_algorithm) that I developed for Tidy. Note that Schlinkert pruning a long inputted word list may take hours or days; removing prefix or suffix words should be significantly quicker. You can learn more about uniquely decodable codes and Schlinkert pruning by reading [this blog post](https://sts10.github.io/2022/08/12/efficiently-pruning-until-uniquely-decodable.html).

Tidy can also simply _check_ if the inputted list is (already) uniquely decodable. It does this using [the Sardinas–Patterson algorithm](https://en.wikipedia.org/wiki/Sardinas%E2%80%93Patterson_algorithm). You can do this by passing Tidy four `attributes` flag (`-AAAA`).

## Whittling

Tidy offers an option `--whittle-to`. This option should **only** be used in specific situations -- users generally should prefer `--print-rand` or `--print-first` options. The situation where whittling gives an advantage over the `print` options is when the following conditions are met:
(a) the inputted word list is sorted by desirability (e.g. ordered by word frequency) and
(b) the user is either removing prefix words (`-P`), removing suffix words (`-S`), and/or doing a Schlinkert prune (`-K`).

To see why whittling is best for this particular situation, see [this document](https://gist.github.com/sts10/25e75d39acdeeafddad943d4d32684ff).

## On maximum shared prefix length

Tidy allows users to set a maximum shared prefix length.

Setting this value to say, 4, means that knowing the first 4 characters of any word on the generated list is sufficient to know which word it is.

On this example generated list where we told Tidy to make the maximum shared prefix length 4 characters, we'd know that if a word starts with "radi", we know it must be the word "radius" (if "radical" had been on the list, Tidy would have removed it).

This is useful if you intend the list to be used by software that uses auto-complete. For example, a user will only have to type the first 4 characters of any word before a program could successfully auto-complete the entire word.

(Note that this setting is distinct from the operation of eliminating prefix words, though can be used in conjunction with that feature.)

Use the attributes flag twice (`-AA`) to get information about shared prefix length for a generated list. Tidy will print both "Longest shared prefix" and "Unique character prefix" (which is longest shared prefix + 1).

## What is "Efficiency per character" and "Assumed entropy per char" and what's the difference?

If we take the entropy per word from a list (log<sub>2</sub>(list_length)) and divide it by the **average** word length of words on the list, we get a value we might call "efficiency per character". This just means that, on average, you get _E_ bits per character typed.

If we take the entropy per word from a list (log<sub>2</sub>(list_length)) and divide it by the length of the **shortest** word on the list, we get a value we might call "assumed entropy per char" (or character).

For example, if we're looking at the EFF long list, we see that it is 7,776-words long, so we'd assume an entropy of log<sub>2</sub>7776 or 12.925 bits per word. The average word length is 7.0, so the efficiency is 1.8 bits per character. (I got this definition of "efficiency" from [an EFF blog post about their list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).) And lastly, the shortest word on the list is three letters long, so we'd divide 12.925 by 3 and get an "assumed entropy per character" of about 4.31 bits per character.

I contend that this "assumed entropy per character" value in particular may be useful when we ask the more theoretical question of "how short should the shortest word on a good word list should be?" There may be an established method for determining what this minimum word length should be, but if there is I don't know about it yet! Here's the math I've worked out on my own.

<!-- Consider the story of a user who gets a passphrase compromised of only the shortest words on the list. Does this passphrase genuinely have the entropy of `log2(list_length)` per word? -->

### The "brute force line"

Assuming the list is comprised of 26 unique characters, if the shortest word on a word list is shorter than log<sub>26</sub>(list_length), there's a possibility that a user generates a passphrase such that the formula of entropy_per_word = log<sub>2</sub>(list_length) will _overestimate_ the entropy per word. This is because a brute-force character attack would have fewer guesses to run through than the number of guesses we'd assume given the word list we used to create the passphrase.

As an example, let's say we had a 10,000-word list that contained the one-character word "a" on it. Given that it's 10,000 words, we'd expect each word to add an additional ~13.28 bits of entropy. That would mean a three-word passphrase would give users 39.86 bits of entropy. However! If a user happened to get "a-a-a" as their passphrase, a brute force method shows that entropy to be only 14.10 bits (4.7 \* 3 words). Thus we can say that it falls below the "brute force line", a phrase I made up.

To see if a given generated list falls above or below this line, use the `-A`/`--attributes` flag.

#### Maximum word list lengths to clear the Brute Force Line

Formula:

Where _S_ is the length of the shortest word on the list, 26 is the number of letters in the English alphabet, and _M_ is max list length: _M_ = 2<sup>_S_ * log<sub>2</sub>(26)</sup>. Conveniently, [this simplifies rather nicely](https://github.com/sts10/tidy/issues/9#issuecomment-1216003299) to _M_ = 26<sup>_S_</sup>.

(or in Python: `max_word_list_length = 26**shortest_word_length`)

| shortest word length | max list length |
|----------------------|-----------------|
| 2                    | 676             |
| 3                    | 17576           |
| 4                    | 456976          |
| 5                    | 11881376        |

### An even stricter "line"

If we go by [a 1951 Claude Shannon paper](https://www.princeton.edu/~wbialek/rome/refs/shannon_51.pdf), each letter in English actually only gives 2.6 bits of entropy. Users can see if their generated word list falls above this (stricter) line -- which I've dubbed the "Shannon line" -- by using the `-A`/`--attributes` flag.

#### Maximum word list lengths to clear the Shannon Line

Formula:

Where _S_ is the length of the shortest word on the list and _M_ is max list length: 2<sup>_S_ * 2.6</sup> = _M_

(or in Python: `max_word_list_length = 2**(shortest_word_length*2.6)`, which, to preserve correct number of significant digits, should be `max_word_list_length = 6.1**shortest_word_length`)

| shortest word length | max list length |
|----------------------|-----------------|
| 2                    | 37              |
| 3                    | 226             |
| 4                    | 1384            |
| 5                    | 8445            |
| 6                    | 51520           |

As you can see, the Shannon line is quite a bit more "strict" than the brute force line.

## A separate tool to help you set dice rolls to correspond with your list

A word list of 7,776 words "fits" nicely into 5 6-sided dice rolls. But not all word lists are 7,776 words long.

If you'd like some help figuring out how to fit your list to a number of dice rolls, another tool I wrote called [Dice Tailor](https://github.com/sts10/dice-tailor) might help.

## What's up with the memchr dependency?

Tidy's function for removing characters on either side of a given delimiter uses a library called [memchr](https://docs.rs/memchr/2.3.4/memchr/), which "provides heavily optimized routines for searching bytes." The optimization gained from using this crate is far from noticeable or necessary for most uses of Tidy -- using Rust's built-in `find` is not much slower -- but I figured the extra speed was worth the dependency in this case.

See [this repo](https://github.com/sts10/splitter) for more information.

## For developers: How to create a release

This project uses [cargo-dist](https://opensource.axo.dev/cargo-dist/) to create releases.

Some of [my personal docs are here](https://sts10.github.io/docs/cargo-dist-tips.html); but basically, `cargo install cargo-dist`. When you're ready to cut a new release, test the current state of the project with `cargo dist build` and `cargo dist plan`. If that went well, create a new git tag that matches the current project version in `Cargo.toml` with `git tag vX.X.X`. Finally, run `git push --tags` to kick off the release process. GitHub will handle it from here -- check your project's GitHub Releases page in about 5 to 10 minutes.

## Appendix: Tools that seem similar to Tidy
-   [cook](https://github.com/giteshnxtlvl/cook): "An overpower[ed] wordlist generator, splitter, merger, finder, saver, create words permutation and combinations, apply different encoding/decoding and everything you need." Written in Go.
-   [duplict](https://github.com/nil0x42/duplicut): "Remove duplicates from MASSIVE wordlist, without sorting it". Seems to indeed be much faster (approximately 10x) than `tidy --no-sort` for de-duplicating large word lists. Written in C.
-   [wordlist-knife](https://github.com/kazkansouh/wordlist-knife): "Versatile tool for managing wordlists." Written in Python.
