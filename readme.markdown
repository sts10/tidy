# Tidy

A command-line tool for combining and cleaning large word list files.

> A throw of the dice will never abolish chance. — Stéphane Mallarmé

## What this tool aims to help users do

Tidy aims to help users create "_better_" word lists -- generally word lists that will be used to create passphrases.

Tidy performs basic list-cleaning operations like removing duplicates words and blank lines by default. It additionally provides various optional standardizations and filters, like lowercasing all words (`-l`), or removing words in with integers in them (`-I`), as well as protections against rare-but-possible passphrase pitfalls, such as prefix codes (`-P`) and low minimum word lengths (see below for explanations).

Tidy also can make word lists more "typo-resistant" by enforcing a minimum edit distance (`-d`), removing homophones and/or enforcing a unique prefix length (`-x`), which can allow users to auto-complete words after a specified number of characters.

Tidy can be used to create new word lists (for example, if given more than one list, it will combine and de-duplicate them) with desirable qualities, but it can also assist in audits of existing lists by removing duplicates and optionally printing notable information about a given word list, and/or a handful of pseudorandomly generated sample passphrases (`--samples`).

## Features

Given a text file with one word per line, this tool will create a new word list in which...

-   duplicate lines (words) are removed
-   empty lines have been removed
-   whitespace from beginning and end of words is deleted
-   words are sorted alphabetically (though this can be optionally prevented -- see below)

and print that new word list to the terminal or to a new text file.

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
-   print corresponding dice rolls before words, separated by a tab. Dice can have 2 to 36 sides. (`-dice`)
-   print information about the new list, such as entropy per word, to the terminal (`-A`, `-AA`, `-AAA`, or `-AAAA` depending on how much information you want to printed)

and more! 

If you do NOT want Tidy to sort list alphabetically, you can use the `--no-sort` option.

## Usage

```txt
USAGE:
    tidy [OPTIONS] <Inputted Word Lists>...

ARGS:
    <Inputted Word Lists>...
            Word list input files. Can be more than one, in which case they'll be combined and de-
            duplicated. Requires at least one file

OPTIONS:
    -a, --approve <APPROVED_LIST>
            Path(s) for optional list of approved words. Can accept multiple files

    -A, --attributes
            Print attributes about new list to terminal. Can be used more than once to print more
            attributes. Some attributes may take a nontrivial amount of time to calculate

    -c, --cut-to <CUT_TO>
            Just before printing generated list, cut list down to a set number of words. Can accept
            expressions in the form of base**exponent (helpful for generating diceware lists). Cuts
            are done randomly

    -C, --remove-nonascii
            Remove all words that have any non-ASCII characters from new list

    -d, --delete-after <DELETE_AFTER_DELIMITER>
            Delete all characters after the first instance of the specified delimiter until the end
            of line (including the delimiter). Delimiter must be a single character (e.g., ','). Use
            't' for tab and 's' for space. May not be used together with -g or -G options

    -D, --delete-before <DELETE_BEFORE_DELIMITER>
            Delete all characters before and including the first instance of the specified
            delimiter. Delimiter must be a single character (e.g., ','). Use 't' for tab and 's' for
            space. May not be used together with -g or -G options

        --debug
            Debug mode

        --dice <DICE_SIDES>
            Print dice roll before word in output. Set number of sides of dice. Must be between 2
            and 36. Use 6 for normal dice

        --dry-run
            Dry run. Don't write new list to file or terminal

    -e, --minimum-edit-distance <MINIMUM_EDIT_DISTANCE>
            Set minimum edit distance between words, which can reduce the cost of typos when
            entering words

    -f, --force
            Force overwrite of output file if it exists

    -g, --ignore-after <IGNORE_AFTER_DELIMITER>
            Ignore characters after the first instance of the specified delimiter until the end of
            line, treating anything before the delimiter as a word. Delimiter must be a single
            character (e.g., ','). Use 't' for tab and 's' for space. Helpful for ignoring metadata
            like word frequencies. Works with attribute analysis and most word removal options, but
            not with word modifications (like to lowercase). May not be used together with -d, -D or
            -G options

    -G, --ignore-before <IGNORE_BEFORE_DELIMITER>
            Ignore characters before and including the first instance of the specified delimiter,
            treating anything after the delimiter as a word. Delimiter must be a single character
            (e.g., ','). Use 't' for tab and 's' for space. Helpful for ignoring metadata like word
            frequencies. Works with attribute analysis and most word removal options, but not with
            word modifications (like to lowercase). May not be used together with -d, -D or -g
            options

    -h, --homophones <HOMOPHONES_LIST>
            Path(s) to file(s) containing homophone pairs. There must be one pair of homophones per
            line, separated by a comma (sun,son)

        --help
            Print help information

    -i, --delete-integers
            Delete all integers from all words on new list

    -I, --remove-integers
            Remove all words with integers in them from list

    -K, --schlinkert-prune
            Use Sardinas-Patterson algorithm to remove words to make list uniquely decodable.
            Experimental!

    -l, --lowercase
            Lowercase all words on new list

    -L, --remove-non-latin-alphabetic
            Remove all words with any characters not in the Latin alphabet (A through Z and a
            through z). All words with accented or diacritic characters will be removed, as well as
            any words with puncuation and internal whitespace

    -m, --minimum-word-length <MINIMUM_LENGTH>
            Set minimum word length

    -M, --maximum-word-length <MAXIMUM_LENGTH>
            Set maximum word length

    -n, --delete-nonalphanumeric
            Delete all non-alphanumeric characters from all words on new list. Characters with
            diacritics will remain

    -N, --remove-nonalphanumeric
            Remove all words with non-alphanumeric characters from new list. Words with diacritics
            will remain

    -o, --output <OUTPUT>
            Path for outputted list file. If none given, generated word list will be printed to
            terminal

    -O, --no-sort
            Do NOT sort outputted list alphabetically. Preserves original list order. Note that
            duplicates lines and blank lines will still be removed

    -P, --remove-prefix
            Remove prefix words from new list

    -q, --straighten
            Replace “smart” quotation marks, both “double” and ‘single’, with their "straight"
            versions

        --quiet
            Do not print any extra information

    -r, --reject <REJECT_LIST>
            Path(s) for optional list of words to reject. Can accept multiple files

        --remove-nonalphabetic
            Remove all words with non-alphabetic characters from new list. Words with diacritcis and
            other non-Latin characters will remain

    -s, --samples
            Print a handful of pseudorandomly selected words from the created list to the terminal.
            Should NOT be used as secure passphrases

    -S, --remove-suffix
            Remove suffix words from new list

        --sides-as-base
            When printing dice roll before word in output, print dice values according to the base
            selected through --dice option. Effectively this means that letters will be used to
            represent numbers higher than 9. Note that this option also 0-indexes the dice values.
            This setting defaults to `false`, which will 1-indexed dice values, and use double-digit
            numbers when necessary (e.g. 18-03-08)

        --take-first <TAKE_FIRST>
            Only take first N words from inputted word list. If two or more word lists are inputted,
            it will combine arbitrarily and then take first N words

        --take-rand <TAKE_RAND>
            Only take a random N number of words from inputted word list. If two or more word lists
            are inputted, it will combine arbitrarily and then take a random N words

    -V, --version
            Print version information

    -W, --whittle-to <WHITTLE_TO>
            Whittle list exactly to a specified length, only taking minimum number of words from the
            beginning of inputted list(s). If the outputted list is not exactly the specified
            length, it will try again by taking a different amount of words form input list(s). As a
            result, this using this option may take a moment. Useful for working with lists that are
            sorted by word frequency or some other metadata. Can accept expressions in the form of
            base**exponent (helpful for generating diceware lists).
            
            Optionally can also take and a rough "starting point", after a comma. For example,
            --whittle-to 7776,15000 would start by taking the first 15,000 words from the inputted
            list(s), then keep iterating from there.

    -x, --shared-prefix-length <MAXIMUM_SHARED_PREFIX_LENGTH>
            Set number of leading characters to get to a unique prefix, which can aid auto-complete
            functionality. Setting this value to say, 4, means that knowing the first 4 characters
            of any word on the generated list is enough to know which word it is
```

## Usage examples

-   `tidy --output new_list.txt word_list1.txt word_list2.txt` Combines the word lists in `word_list1.txt` and `word_list2.txt`, removing whitespace, empty lines, and duplicate words into one list. It sorts this list alphabetically, and then prints this new, combined list to the specified output location, in this case: `new_list.txt`.

-   `tidy -l -o new_list.txt inputted_word_list.txt` Deletes whitespace, removes empty lines and duplicate words from `inputted_word_list.txt`. Due to the `-l` flag, it makes all the words lowercase. It sorts this list alphabetically and removes duplicates once again. It then prints this new list to the specified output location, in this case: `new_list.txt`.

-   `tidy -l inputted_word_list.txt > new_list.txt` Alternatively, you can use `>` to print tidy's output to a file.

-   `tidy -lP -o new_list.txt inputted_word_list.txt` Same as above, but the added `-P` flag removes prefix words from the list. See below for more on prefix words.

-   `tidy -lPi -o new_list.txt inputted_word_list.txt` Same as above, but the added `-i` flag deletes any integers in words. Words with integers in them are not removed, only the integers within them. For example, "11326 agency" becomes "agency".

-   `tidy -lPiO -o new_list.txt inputted_word_list.txt` Same as above, but the added `-O` flag preserves the original order of the list, rather than sort it alphabetically. Note that duplicates and blank lines are still removed.

-   `tidy -I -o new_list.txt inputted_word_list.txt` Using the `-I` flag removes any words with integers from the list. For example, "hello1" would be removed from the list.

-   `tidy -AA -I -o new_list.txt inputted_word_list.txt` Adding `-AA` prints some information about the created list to the terminal.

-   `tidy -l -o new_list.txt -r bad_words.txt inputted_word_list.txt` Similar to above, but ensures that none of the words in the bad_words.txt file make it on to the final list that is printed to new_list.txt. The reject list is case sensitive, so you may want to run it through tidy using the -l flag before using it. (You can find [a list of bad words here](https://code.google.com/archive/p/badwordslist/downloads).)

-   `tidy -l -o new_list.txt -a approved_words.txt inputted_word_list.txt` Similar to above, but ensures that only words in the approved_words.txt file make it on to the final list that is printed to new_list.txt. The approved list is case sensitive. (On Mac and some Linux distributions, `/usr/share/dict/words` should contain a list of words for spellcheck purposes.)

-   `tidy -l -o new_list.txt -h homophone_pairs.txt inputted_word_list.txt` Similar to above, but expects `homophones_pairs.txt` to be a list of homophones pairs separated by a comma ("right,write" then next line: "epic,epoch"). If both words in the pair are on the inputted_word_list, Tidy will remove the second one. If only one of the words in the pair are on the list, Tidy won't remove it. Must be only two words per line.

-   `tidy -lA -m 3 -o new-list.txt inputted_word_list.txt` Similar to above, but the `-m 3` means new list won't have any words under 3 characters in length. Have Tidy also print some attributes about the new list to the terminal screen.

-   `tidy -d t -o just_the_words.txt diceware_list.txt` If you've got [a diceware list with numbers and a tab before each word](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt), the `-d t` flag will delete everything up to and including the first tab in each line ("11133 abruptly" becomes "abruptly").

-   `tidy --dice 6 -o diceware_list.txt just_words.txt` Add corresponding dice roll numbers to a list with `--dice`. Can accept dice sides between 2 and 36. Each dice roll and word are separated by a tab.

-   `tidy -P -x 4 --cut-to 7776 --dice 6 --output diceware.txt 1password-2021.txt` Make a 7,776-word list from a [1Password (~18k) word list](https://1password.com/txt/agwordlist.txt), removing prefix words and guaranteeing 4 characters can auto-complete any word. Lastly, add corresponding 6-sided dice role for each word.

-   `tidy -o d-and-d.txt --dice 20 --cut-to 20**3 wordlist.txt` Create an 8,000-word list where each word corresponds to 3 rolls of a 20-sided die (`06-07-07	dragon`). `--cut-to` randomly truncates the resulting list to the specified amount -- can accept integers (`8000`) or informal exponent notation (`20**3`). See [EFF's fandom-inspired wordlists](https://www.eff.org/deeplinks/2018/08/dragon-con-diceware) for more.

-   `tidy -d s --whittle-to 7776 -PlL -m 3 -M 12 --dice 6 -o wiki-diceware.txt ~/Downloads/enwiki-20190320-words-frequency-sorted.txt` Carefully make a 7,776-word list by only taking the words needed from the top of `~/Downloads/enwiki-20190320-words-frequency-sorted.txt` [file](https://github.com/IlyaSemenov/wikipedia-word-frequency/blob/master/results/enwiki-20190320-words-frequency.txt). Assumes this file is sorted by word frequencies, with a frequency count after the word, separated by a space (example line: `located 1039008`). Since we only want to use the most common words, we'll use Tidy's `--whittle-to` option to only take exactly how many words we need to construct a list of 7,776 words. Note that this may take longer that usual Tidy executions, since Tidy will very likely need to make multiple attempts to make a list that's exactly the requested length. [More info on whittle](https://github.com/sts10/tidy/issues/15#issuecomment-1215907335).

## List attributes

Tidy can also calculate different attributes about a created list. `tidy -AAAA -G t --dry-run eff_long_list.txt` prints:

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
Above Shannon line?       : false
Shortest edit distance    : 1
Mean edit distance        : 6.858
Longest shared prefix     : 8
Unique character prefix   : 9
```

Using the `--samples` flag will print 5 sample passphrases to the terminal. (Note that these sample passphrases should not be used for security purposes, as Tidy has not been audited.)

```txt
Pseudorandomly generated sample words
-------------------------------------
departure traitor augmented supremacy device annoying 
steep frigidity wreath barterer tibia reprimand 
creamlike strum snowfall tannery clean protrude 
favorable unlivable vanquish crate sarcastic exclude 
fastness september boasting unbroken battalion sweep
```

## On verbs used

In both Tidy's code and documentation, "remove" means that a word will be removed (e.g. words with integers will be removed from the list), while "delete" means that a word will only be modified (e.g. integers removed from words). Uppercase flags remove words, while lowercase flags delete specified characters. All delete calls and word modifications (like "to lowercase") occur _before_ any remove call.

## Installation

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/tidy --branch main`

You should then be able to run `tidy --help`.

## For Tidy developers

Run all code tests: `cargo test`

Generate docs: `cargo doc --document-private-items --no-deps`. Add `--open` flag to open docs after generation. They're printed to `./target/doc/tidy/index.html`.

## Blog posts related to this project

* [Read more about the 0.2 version of this project](https://sts10.github.io/2021/12/09/tidy-0-2-0.html)
* [Read about uniquely decodable codes and "Schlinkert pruning"](https://sts10.github.io/2022/08/12/efficiently-pruning-until-uniquely-decodable.html)
* [Read about initial inspiration for the project](https://sts10.github.io/2020/09/30/making-a-word-list.html)

## Working with homophones

If you're looking for a large-ish list of English homophones, I'd humbly point you to [this other project of mine](https://github.com/sts10/homophones).

### Using Tidy to remove homophones

If passphrases from your list will ever be spoken out loud, you may want to consider removing homophones -- words that sound alike -- from your list.

I'd say that Tidy offers two ways of dealing with homophones.

Given a pair of homophones, like "sun" and "son":

1. To ensure you don't have BOTH homophones in your generated list, you'd run `tidy` with a flag like `-h ../homophones/homophone-lists/homophones-large-as-pairs.txt` ([link](https://github.com/sts10/homophones/blob/main/homophone-lists/homophones-large-as-pairs.txt)). This will let either "sun" or "son" on your list but NOT both.
2. To ensure you have NEITHER of the words in the homophone pair on your generated word list, you'd use the reject words flags: `-r ../homophones/homophone-lists/cleaned-as-singles.txt` ([link](https://github.com/sts10/homophones/blob/main/homophone-lists/cleaned-as-singles.txt)). This will remove _both_ "sun" and "son" from your generated list before its outputted.

## What are prefix words (a.k.a. prefix codes)?

A word list that doesn't has any prefix words (also known as "[prefix codes](https://en.wikipedia.org/wiki/Prefix_code)") can better guarantee more consistent entropy when combining words from the list randomly and without punctuation between the words.

As a brief example, if a list has "boy", "hood", and "boyhood" on it, users who specified they wanted two words worth of randomness (entropy) might end up with "boyhood", which an attacker guessing single words would try. Removing prefix words -- in this case "boy" -- prevents this possibility from occurring. Mandating that words have a punctuation mark, like a hyphen, between them (`boy-hood`) also solves this potential issue.

You can read more about this issue [here](https://github.com/ulif/diceware#prefix-code).

## On maximum shared prefix length

Tidy allows users to set a maximum shared prefix length.

Setting this value to say, 4, means that knowing the first 4 characters of any word on the generated list is sufficient to know which word it is. As an example, we'd know that if a word starts with "radi", we know it must be the word "radius" (if "radical" had been on the list, it Tidy would have removed it).

This is useful if you intend the list to be used by software that uses auto-complete. For example, a user will only have to type the first 4 characters of any word before a program could successfully auto-complete the entire word.

(Note that this setting is distinct from the operation of eliminating prefix words, though can be used in conjunction with that feature.)

Use the attributes flag twice (`-AA`) to get information about shared prefix length for a generated list. Tidy will print both "Longest shared prefix" and "Unique character prefix" (which is longest shared prefix + 1).

## What is "Efficiency per character" and "Assumed entropy per char" and what's the difference?

If we take the entropy per word from a list (log<sub>2</sub>(list_length)) and divide it by the length of the **average**-length word on the list, we get a value we might call "efficiency per character". This just means that, on average, you get _E_ bits per character typed. 

If we take the entropy per word from a list (log<sub>2</sub>(list_length)) and divide it by the length of the **shortest** word on the list, we get a value we might call "assumed entropy per char" (or character).

For example, if we're looking at the EFF long list, we see that its' 7,776-words long, so we'd assume an entropy of 12.925 bits per word. The average word length is 7.0, so the efficiency is 1.8 bits per character. (I got this definition of "efficiency" from [an EFF blog post about their list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).) The shortest word on the list is three letters long, so we'd divide 12.925 by 3 and get an "assumed entropy per character" of about 4.31 bits per character.

I contend that this second value in particular may be useful when we ask what the shortest word on a good word list should be. There may be an established method for determining what this minimum word length should be, but if there is I don't know about it yet. Here's the math I've worked out on my own.

<!-- Consider the story of a user who gets a passphrase compromised of only the shortest words on the list. Does this passphrase genuinely have the entropy of `log2(list_length)` per word? -->

### The "brute force line"

Assuming the list is comprised of 26 unique characters, if the shortest word on a word list is shorter than log<sub>26</sub>(list_length), there's a possibility that a user generates a passphrase such that the formula of entropy_per_word = log<sub>2</sub>(list_length) will _overestimate_ the entropy per word. This is because a brute-force character attack would have fewer guesses to run through than the number of guesses we'd assume given the word list we used to create the passphrase.

As an example, let's say we had a 10,000-word list that contained the one-character word "a" on it. Given that it's 10,000 words, we'd expect each word to add an additional ~13.28 bits of entropy. That would mean a three-word passphrase would give users 39.86 bits of entropy. However! If a user happened to get "a-a-a" as their passphrase, a brute force method shows that entropy to be only 14.10 bits (4.7 \* 3 words). Thus we can say that it falls below the "brute force line", a phrase I made up.

To see if a given generated list falls above or below this line, use the `-A`/`--attributes` flag.

#### Maximum word list lengths to clear the Brute Force Line

Formula: 

Where _S_ is the length of the shortest word on the list, 26 is the number of letters in the English alphabet, and _M_ is max list length: 2<sup>_S_ * log<sub>2</sub>(26)</sup> = _M_

(or in Python: `max_word_list_length = 2**(S*4.7)`)

| shortest word length | max list length |
|----------------------|-----------------|
| 2                    | 675             |
| 3                    | 17559           |
| 4                    | 456419          |
| 5                    | 11863283        |

### An even stricter "line"

If we go by [a 1951 Claude Shannon paper](https://www.princeton.edu/~wbialek/rome/refs/shannon_51.pdf), each letter in English actually only gives 2.62 bits of entropy. Users can see if their generated word list falls above this (stricter) line -- which I've dubbed the "Shannon line" -- by using the `-A`/`--attributes` flag.

#### Maximum word list lengths to clear the Shannon Line

Formula: 

Where _S_ is the length of the shortest word on the list and _M_ is max list length: 2<sup>_S_ * 2.62</sup> = _M_

(or in Python: `max_word_list_length = 2**(S*2.62)`)

| shortest word length | max list length |
|----------------------|-----------------|
| 2                    | 37              |
| 3                    | 232             |
| 4                    | 1428            |
| 5                    | 8779            |
| 6                    | 53974           |

As you can see, the Shannon line is quite a bit more "strict" than the brute force line.

## Language limitations

As a native English speaker, I wrote this program with lists of English (US) words in mind. Unfortunately, I haven't tested it with other languages. If you have ideas for how to make it more usable for other languages, please open an Issue or submit a Pull Request.

## What's up with the memchr dependency?

Tidy's function for removing characters on either side of a given delimiter uses a library called [memchr](https://docs.rs/memchr/2.3.4/memchr/), which "provides heavily optimized routines for searching bytes." The optimization gained from using this crate is far from noticeable or necessary for most uses of Tidy -- using Rust's built-in `find` is not much slower -- but I figured the extra speed was worth the dependency in this case.

See [this repo](https://github.com/sts10/splitter) for more information.

## To do

-   [X] Add option to remove suffix words 
-   [X] Add option to remove words that have characters from certain character sets, for example non-ASCII characters
-   [ ] Figure out how to have Tidy read from stdin
-   [ ] Improve performance of Sardinas-Patterson functions
<!-- -   [ ] Investigate making the list variable as [AHashSet](https://github.com/tkaitchuck/ahash)<String> rather than a `Vec<String>` to potentially boost performance. Will probably need to write some benchmarks with Criterion to measure potential performance gains. -->

## Appendix: Where can I find some word lists?

-   The [Electronic Frontier Foundation](https://www.eff.org/) has published [a few word lists for creating diceware passphrases](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases). 
    -   I'm pretty sure password manager BitWarden uses [the EFF long list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt). 
    -   [KeePassXC](https://keepassxc.org/) uses [the EFF long list with some minor modifications](https://github.com/keepassxreboot/keepassxc/blob/develop/share/wordlists/eff_large.wordlist). Since there's a tab between the dice numbers and each word, Tidy can delete the dice numbers easily with something like `tidy -D t -o clean_eff.txt eff_large_wordlist.txt` or using the `-i` flag.
    -   The EFF also has some [fandom-inspired lists](https://www.eff.org/deeplinks/2018/08/dragon-con-diceware). They use a space between dice numbers and words, so Tidy can clean up with the `-D s` option.
-   I'm [pretty sure](https://twitter.com/1Password/status/1462885816569577480) this is [1Password](https://1password.com/)'s [word list](https://1password.com/txt/agwordlist.txt) as of 2021.
    -   [1Password](https://1password.com/) published a slightly different [word list](https://github.com/agilebits/crackme/blob/master/doc/AgileWords.txt) in 2018.
-   If you're using Linux or MacOS, you've likely got some long lists on your computer. Check `/usr/share/dict/words` or `/usr/share/dict/american-english`.
-   [SecureDrop](https://github.com/freedomofpress/securedrop/) has a few lists, including one of [adjectives](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/adjectives.txt) and one of [nouns](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/nouns.txt).
-   [Bitcoin BIPS-0039](https://github.com/bitcoin/bips/tree/master/bip-0039) (2,048 words) (h/t [atoponce](https://www.reddit.com/r/Passwords/comments/sqrymt/comment/hwnfb94/))
-   [Monero's word list](https://github.com/monero-project/monero/blob/master/src/mnemonics/english.h) (1,626 words) (h/t [atoponce](https://www.reddit.com/r/Passwords/comments/sqrymt/comment/hwnfb94/))
-   [Mnemonicode](https://github.com/schollz/mnemonicode/blob/master/word_list.go) is another word list optimized for pronunciation. I believe [croc](https://github.com/schollz/croc), a file transferring tool, uses it.
-   [Magic Wormhole](https://github.com/magic-wormhole/magic-wormhole/), a tool for transferring files, uses [a version of the PGP Word List](https://github.com/magic-wormhole/magic-wormhole/blob/master/src/wormhole/_wordlist.py), which specifically tries to use pairs of words that are phonetically distinct.
-   A collection of a few [Public Domain Word Lists](https://github.com/MichaelWehar/Public-Domain-Word-Lists)
-  [ulif's "diceware"](https://github.com/ulif/diceware) seems to have collected [a few word lists](https://github.com/ulif/diceware/tree/master/diceware/wordlists) in its Github repo, along with [a separate page that explains each of the lists](https://github.com/ulif/diceware/blob/master/docs/wordlists.rst).
-   [dmuth's "diceware" program](https://github.com/dmuth/diceware) has a [collection of lists](https://github.com/dmuth/diceware/tree/master/wordlist) (h/t [atoponce](https://www.reddit.com/r/Passwords/comments/sqrymt/comment/hwnfb94/))
-   [Niceware list](https://github.com/diracdeltas/niceware/blob/master/lib/wordlist.js) (~65,000 words)<!-- (there's also [a Rust port of niceware](https://github.com/healeycodes/niceware)).-->
-   [Lists used by a program called webpassgen](https://github.com/atoponce/webpassgen/tree/master/lists)
-   [Original "Reinhold" diceware list](https://theworld.com/%7Ereinhold/diceware.wordlist.asc) created by [Arnold Reinhold](https://theworld.com/~reinhold/). Though it has some issues.
    -   Arnold Reinhold hosts [diceware lists in a variety of languages](https://theworld.com/~reinhold/diceware.html#Diceware%20in%20Other%20Languages|outline).
-   [r/wordlists subreddit](https://www.reddit.com/r/wordlists/), which seems to have links to a few non-English word lists.
-   [simple1024](https://github.com/pera/simple1024) is a word list with 1024 common English words, an alternative to EFF's short word lists.

### Shameless plug

-   I've created [a few word lists](https://github.com/sts10/generated-wordlists) using Tidy.
