# v0.3.9

* Update all dependencies that have new versions
* Uses version 0.14.1 of cargo-dist to create release binaries and a shell installation script.

# v0.3.8

* Uses version 0.8.0 of cargo-dist to create release binaries and a shell installation script.
# v0.3.8

* Uses version 0.8.0 of cargo-dist to create release binaries and a shell installation script.

# v0.3.7

First release using [cargo-dist](https://opensource.axo.dev/cargo-dist/). Should create binaries for Mac and Windows users. Cool!

# v0.3.0

The big new feature in this release is that users can optionally print attributes and word samples in JSON format. 

## Changes
* d06d1ea - Uses an enum for result of Kraft-McMillan Inequality 
* abe465d - only calculates longest word length once, in order to be more efficient
* a979645 - brings help text up to date with JSON feature  
* fdf4071 - print word samples within JSON output
* dad0cd6 - gives credit back to Kraft!
* f77ec28 - more concise creation of `ListAttributes` object. Also think I made the shared prefix calculation a bit faster 
* 8549df7 - make shared prefix optional, since it takes a while
* 95d72b6 - improves the descriptiveness of a function name
* 4fed268 - fixes spelling of 'unique' in new display attributes code 
* b07f7dc - puts `ListAttributes` into a new enum, adds feature of printing list attributes in JSON

# v0.2.91

Mostly housekeeping in this release.

* 0a6a78b - moves Shannon line boolean attribute behind 5 As rather than 4, since it's a pretty dubious attribute at this point  
* 67ab0ca - adds link to NSA's password generator and its word list
* d3f3549 - fixes mistake in explanation of unique decodability in readme 
* dc4828e - adds some metadata to Cargo.toml for thoroughness
* 80181b0 - adds upgrade and uninstall information to the readme  
* 84bf97a - updates word sample language in readme 

# v0.2.90

The big change in this release is that Tidy now performs Schlinkert pruning both on the list as given, _and_ the list where every word is reversed. 

Performing the Schlinkert prune on the reversed words is equivalent to using prefix words in Sardinas-Patterson algorithm, rather than suffix words. Tidy now tries both, preferring whichever process saves more words on the original list. This is the case on the BIPS39 English word list. See #43 for more information.

## Commits with major changes
* 1de5d1c - adds a test to make sure Tidy runs Schlinkert pruning the reversed list
* be38459 - when reversing words before doing the Schlinkert prune, use graphemes rather than characters to better attempt to handle accented characters and emoji  
* 8ac7782 - executes Schlinkert prune in both directions, then prefer whichever saves the most words
* d681136 - Adds deny.toml to ease compatibility checks
* 24063ce - doesn't print a space after 6th word of each sample


Also various function and variable renaming for clarity and, as usual, other updates to the README. 
