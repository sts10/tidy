/// This is a (rather clumsily) implementation of the Sardinas-Patterson algorithm
/// The goal is to check if a word list (`c`) is uniquely decodable.
///
/// I followed
/// https://github.com/danhales/blog-sardinas-patterson/blob/master/index.ipynb
/// very closely.
/// It is not very fast, likely because of the handful of `clone()` calls.
/// We should try to minimize those to increase the speed of the calculations!
use std::collections::HashSet;

pub fn check_decodability(c: &[String]) -> bool {
    // Right off the bat, convert inputted Slice
    // to a HashSet
    let c = vec_to_hash(c);
    sardinas_patterson_theorem(c.clone())
}

fn vec_to_hash(v: &[String]) -> HashSet<String> {
    let mut my_hash = HashSet::new();
    for e in v {
        my_hash.insert(e.to_string());
    }
    my_hash
}

// generate c for any number n
fn generate_cn(c: HashSet<String>, n: usize) -> HashSet<String> {
    if n == 0 {
        return c;
    } else {
        let mut cn = HashSet::new();

        // generate c_(n-1)
        let cn_minus_1 = generate_cn(c.clone(), n - 1);
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
        // Now the other way?
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
    let mut cn = generate_cn(c.clone(), n);

    while cn.len() > 0 {
        if cn.is_subset(&cs) {
            // Cycle detected. Halting algorithm.
            break;
        } else {
            cs = cs.union(&cn).map(|e| e.to_string()).collect();
            c_infinity = c_infinity.union(&cn).map(|e| e.to_string()).collect();
            n += 1;
            cn = generate_cn(c.clone(), n);
        }
    }
    c_infinity
}

/// Returns true if c in uniquely decodable
fn sardinas_patterson_theorem(c: HashSet<String>) -> bool {
    let c_infinity = generate_c_infinity_with_a_halt_break(c.clone());
    c.is_disjoint(&c_infinity)
}
