//! Compute the edit distance between two strings

use std::cmp::min;

/// `find_edit_distance(str_a, str_b)` returns the edit distance between the two
/// strings. This edit distance is defined as being 1 point per insertion,
/// substitution, or deletion which must be made to make the strings equal.
///
/// I adapted this function from one I found in the
/// [TheAlgorithms/Rust repo on Github](https://github.com/TheAlgorithms/Rust/blob/master/src/dynamic_programming/edit_distance.rs).
///
/// Instead of storing the `m * n` matrix expicitly, only one row (of length `n`) is stored.
/// It keeps overwriting itself based on its previous values with the help of two scalars,
/// gradually reaching the last row. Then, the score is `matrix[n]`.
pub fn find_edit_distance(str_a: &str, str_b: &str) -> u32 {
    let (str_a, str_b) = (str_a.as_bytes(), str_b.as_bytes());
    let (m, n) = (str_a.len(), str_b.len());
    let mut distances: Vec<u32> = vec![0; n + 1]; // the dynamic programming matrix (only 1 row stored)
    let mut s: u32; // distances[i - 1][j - 1] or distances[i - 1][j]
    let mut c: u32; // distances[i][j - 1] or distances[i][j]
    let mut char_a: u8; // str_a[i - 1] the i-th character in str_a; only needs to be computed once per row
    let mut char_b: u8; // str_b[j - 1] the j-th character in str_b

    // 0th row
    for (j, v) in distances.iter_mut().enumerate().take(n + 1).skip(1) {
        *v = j as u32;
    }
    // rows 1 to m
    for i in 1..=m {
        s = (i - 1) as u32;
        c = i as u32;
        char_a = str_a[i - 1];
        for j in 1..=n {
            // c is distances[i][j-1] and s is distances[i-1][j-1] at the beginning of each round of iteration
            char_b = str_b[j - 1];
            c = min(
                s + if char_a == char_b { 0 } else { 1 },
                min(c + 1, distances[j] + 1),
            );
            // c is updated to distances[i][j], and will thus become distances[i][j-1] for the next cell
            s = distances[j]; // here distances[j] means distances[i-1][j] becuase it has not been overwritten yet
                              // s is updated to distances[i-1][j], and will thus become distances[i-1][j-1] for the next cell
            distances[j] = c; // now distances[j] is updated to distances[i][j], and will thus become distances[i-1][j] for the next ROW
        }
    }

    distances[n]
}
