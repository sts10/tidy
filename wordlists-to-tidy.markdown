# Appendix A: Where can I find some word lists?

## Diceware lists (generally 7,776 words long)
-   The [Electronic Frontier Foundation](https://www.eff.org/) has published [a few word lists for creating diceware passphrases](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).
    -   I'm pretty sure password manager BitWarden uses [the EFF long list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt).
    -   [KeePassXC](https://keepassxc.org/) uses [the EFF long list with some minor modifications](https://github.com/keepassxreboot/keepassxc/blob/develop/share/wordlists/eff_large.wordlist).
    -   Note: These lists often have a tab between the dice numbers and each word. Tidy can delete the dice numbers easily with something like `tidy -D t -o clean_eff.txt eff_large_wordlist.txt` or using the `-i` flag.
-   [ulif's "diceware"](https://github.com/ulif/diceware) seems to have collected [a few word lists](https://github.com/ulif/diceware/tree/master/diceware/wordlists) in its Github repo, along with [a separate page that explains each of the lists](https://github.com/ulif/diceware/blob/master/docs/wordlists.rst).
-   [dmuth's "diceware" program](https://github.com/dmuth/diceware) has a [collection of lists](https://github.com/dmuth/diceware/tree/master/wordlist) (h/t [atoponce](https://www.reddit.com/r/Passwords/comments/sqrymt/comment/hwnfb94/))
-   [Original "Reinhold" diceware list](https://theworld.com/%7Ereinhold/diceware.wordlist.asc) created by [Arnold Reinhold](https://theworld.com/~reinhold/). Though it has some issues.
    -   Arnold Reinhold hosts [diceware lists in a variety of languages](https://theworld.com/~reinhold/diceware.html#Diceware%20in%20Other%20Languages|outline).

## Short word lists
-   [Bitcoin BIPS-0039](https://github.com/bitcoin/bips/tree/master/bip-0039) (2,048 words) (h/t [atoponce](https://www.reddit.com/r/Passwords/comments/sqrymt/comment/hwnfb94/))
-   [Monero's word list](https://github.com/monero-project/monero/blob/master/src/mnemonics/english.h) (1,626 words) (h/t [atoponce](https://www.reddit.com/r/Passwords/comments/sqrymt/comment/hwnfb94/))
-   [Mnemonicode](https://github.com/schollz/mnemonicode/blob/master/word_list.go) is another word list optimized for pronunciation. I believe [croc](https://github.com/schollz/croc), a file transferring tool, uses it.
-   [Magic Wormhole](https://github.com/magic-wormhole/magic-wormhole/), a tool for transferring files, uses [a version of the PGP Word List](https://github.com/magic-wormhole/magic-wormhole/blob/master/src/wormhole/_wordlist.py), which specifically tries to use pairs of words that are phonetically distinct.
-   [simple1024](https://github.com/pera/simple1024) is a word list with 1024 common English words, an alternative to EFF's short word lists.

## Pretty long word lists
-   If you're using Linux or MacOS, you've likely got some long lists on your computer. Check `/usr/share/dict/words` or `/usr/share/dict/american-english`.
-  [NSA's RandPassGenerator](https://github.com/nsacyber/RandPassGenerator) uses [a massive 117,828-word list](https://github.com/nsacyber/RandPassGenerator/blob/master/RandPassGenerator/data/wordlist.txt).
-   [Niceware list](https://github.com/diracdeltas/niceware/blob/master/lib/wordlist.js) (~65,000 words). [I used Tidy to help create v 4.0.0 of this list](https://github.com/diracdeltas/niceware/pull/52)!<!-- (there's also [a Rust port of niceware](https://github.com/healeycodes/niceware)).-->
-   [Norvig Natural Language Corpus Data](https://norvig.com/ngrams/) has [a list of 333,000 commonly used words](https://norvig.com/ngrams/count_1w.txt) from the Google Web Trillion Word Corpus, as well as an assortment of other word lists.
-   [British National Corpus (BNC) database and word frequency lists](https://www.kilgarriff.co.uk/bnc-readme.html)
-   [Lists used by a program called webpassgen](https://github.com/atoponce/webpassgen/tree/master/lists)
-   [SCOWL (Spell Checker Oriented Word Lists) and Friends](http://wordlist.aspell.net/) is a database of information on English words useful for creating high-quality word lists suitable for use in spell checkers of most dialects of English."
    -   [ENABLE2K](https://web.archive.org/web/20090122025747/http://personal.riverusers.com/~thegrendel/software.html) seems to be an older version of the SCOWL project? 

## Collections of word lists
-   A collection of a few [Public Domain Word Lists](https://github.com/MichaelWehar/Public-Domain-Word-Lists)
-   [**A great list of word lists** by Aaron Toponce](https://gist.github.com/atoponce/95c4f36f2bc12ec13242a3ccc55023af).
-   [A list of word lists](http://www.webplaces.com/passwords/passphrase-word-lists.htm).
-   [Danish wordlists](https://github.com/n0kovo/danish-wordlists) is a "collection of [Danish] wordlists for cracking danish passwords"
-   [r/wordlists subreddit](https://www.reddit.com/r/wordlists/), which seems to have links to a few non-English word lists.
-   You can also scan [GitHub's #wordlists topic](https://github.com/topics/wordlists)

## Various 
-   The EFF also has some [fandom-inspired lists](https://www.eff.org/deeplinks/2018/08/dragon-con-diceware). They use a space between dice numbers and words, so Tidy can clean up with the `-D s` option. I prefer [Aaron Toponce's proposed _new_ fandom word lists](https://github.com/sts10/new-fandom-wordlists).
-   I'm pretty sure this is [1Password](https://1password.com/)'s [word list](https://1password.com/txt/agwordlist.txt) as of 2021.
    -   1Password published a few slightly different word lists ([one](https://github.com/1Password/spg/blob/master/testdata/agwordlist.txt), [two](https://github.com/agilebits/crackme/blob/master/doc/AgileWords.txt)) in 2018.
-   [SecureDrop](https://github.com/freedomofpress/securedrop/) has a few lists, including one of [adjectives](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/adjectives.txt) and one of [nouns](https://github.com/freedomofpress/securedrop/blob/develop/securedrop/dictionaries/nouns.txt).

## Shameless plug

-   I used Tidy to create [Orchard Street Wordlists](https://github.com/sts10/orchard-street-wordlists) ([as well as a few other word lists](https://github.com/sts10/generated-wordlists)).
