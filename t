[1mdiff --git a/readme.markdown b/readme.markdown[m
[1mindex 0085776..774e9bf 100755[m
[1m--- a/readme.markdown[m
[1m+++ b/readme.markdown[m
[36m@@ -354,7 +354,6 @@[m [mEntropy per word          : 12.925 bits[m
 Efficiency per character  : 1.849 bits[m
 Assumed entropy per char  : 4.308 bits[m
 Above brute force line?   : true[m
[31m-Above Shannon line?       : false[m
 Shortest edit distance    : 1[m
 Mean edit distance        : 6.858[m
 Longest shared prefix     : 8[m
[36m@@ -505,11 +504,9 @@[m [mAs you can see, the Shannon line is quite a bit more "strict" than the brute for[m
 [m
 ## A separate tool to help you set dice rolls to correspond with your list[m
 [m
[31m-A word list of 7,776 words "fits" nicely into 5 6-sided dice rolls. If you'd like some help figuring out how to fit your list to a number of dice rolls, another tool I wrote called [Dice Tailor](https://github.com/sts10/dice-tailor) might help.[m
[32m+[m[32mA word list of 7,776 words "fits" nicely into 5 6-sided dice rolls. But not all word lists are 7,776 words long.[m
 [m
[31m-## Language limitations[m
[31m-[m
[31m-As a native English speaker, I wrote this program with lists of English (US) words in mind. Unfortunately, I haven't tested it with other languages. If you have ideas for how to make it more usable for other languages, please open an Issue or submit a Pull Request.[m
[32m+[m[32mIf you'd like some help figuring out how to fit your list to a number of dice rolls, another tool I wrote called [Dice Tailor](https://github.com/sts10/dice-tailor) might help.[m
 [m
 ## What's up with the memchr dependency?[m
 [m
