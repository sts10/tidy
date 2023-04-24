/// This is a (rather clumsily) implementation of the Sardinas-Patterson algorithm
/// The goal is to check if a word list (`c`) is uniquely decodable.
///
/// I followed
/// https://github.com/danhales/blog-sardinas-patterson/blob/master/index.ipynb
/// very closely.
use std::collections::HashSet;

/// Return true if the list is uniquely decodable, false if not
pub fn check_decodability(c: &[String]) -> bool {
    let is_c_uniquely_decodable_forwards = sardinas_patterson_theorem(vec_to_hash(c));
    // We need to check both forward and reverse
    let c_reversed = reverse_all_words(c);
    let is_c_uniquely_decodable_backwards = sardinas_patterson_theorem(vec_to_hash(&c_reversed));
    is_c_uniquely_decodable_forwards || is_c_uniquely_decodable_backwards
}

fn vec_to_hash(v: &[String]) -> HashSet<String> {
    let mut my_hash = HashSet::new();
    for e in v {
        my_hash.insert(e.to_string());
    }
    my_hash
}

// Generate c for any number n
fn generate_cn(c: &HashSet<String>, n: usize) -> HashSet<String> {
    if n == 0 {
        return c.to_owned();
    } else {
        let mut cn = HashSet::new();

        // generate c_(n-1)
        let cn_minus_1 = generate_cn(c, n - 1);
        for w1 in c.iter() {
            for w2 in cn_minus_1.iter() {
                if w1.len() > w2.len() && w1.starts_with(w2) {
                    // w2 is a prefix word of w1
                    // so, we're going to add the dangling suffix to a new HashSet
                    // called cn
                    cn.insert(w1[w2.len()..].to_string());
                }
            }
        }
        // Now the other way? Could we clean this up?
        for w1 in cn_minus_1.iter() {
            for w2 in c.iter() {
                if w1.len() > w2.len() && w1.starts_with(w2) {
                    // w2 is a prefix word of w1
                    // so, we're going to add the dangling suffix to a new HashSet
                    // called cn
                    cn.insert(w1[w2.len()..].to_string());
                }
            }
        }
        cn
    }
}

fn generate_c_infinity_with_a_halt_break(c: HashSet<String>) -> HashSet<String> {
    let mut cs = HashSet::new();
    let mut c_infinity = HashSet::new();
    let mut n = 1;
    let mut cn = generate_cn(&c, n);

    while !cn.is_empty() {
        if cn.is_subset(&cs) {
            // Cycle detected. Halting algorithm.
            break;
        } else {
            cs = cs.union(&cn).map(|e| e.to_string()).collect();
            c_infinity = c_infinity.union(&cn).map(|e| e.to_string()).collect();
            n += 1;
            cn = generate_cn(&c, n);
        }
    }
    c_infinity
}

fn reverse_all_words(list: &[String]) -> Vec<String> {
    let mut reversed_list = vec![];
    for word in list {
        reversed_list.push(word.chars().rev().collect::<String>());
    }
    reversed_list
}

/// Returns true if c is uniquely decodable
fn sardinas_patterson_theorem(c: HashSet<String>) -> bool {
    let c_infinity = generate_c_infinity_with_a_halt_break(c.clone());
    c.is_disjoint(&c_infinity)
}
