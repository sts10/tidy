use std::collections::HashSet;

// Return a Vector of words that "caused" the Sardinas-
// Patterson algorithm to determine that this list was not
// uniquely decodable.
// These "offending" words can then be removed from the original
// list to, theoretically, make the list uniquely decodable.
pub fn get_sardinas_patterson_final_intersection(c: &[String]) -> Vec<String> {
    // Right off the bat, convert inputted Slice to a HashSet
    // Since we always want this list to be unique, and we're
    // going to eventually calculate a disjoint boolean!
    let c = vec_to_hash(c);
    let c_infinity = generate_c_infinity_with_a_halt_break(c.clone());
    // We want to collect a list of words that "caused" the Sardinas-
    // Patterson algorithm to determine that this list was not
    // uniquely decodable.
    // If the given list is in fact uniquely decodable,
    // this list of words will be empty, which is what we want.
    // If there are words in the list, we'll return those to src/lib to be
    // removed from the final list.
    let final_intersection = c.intersection(&c_infinity);
    Vec::from_iter(final_intersection)
        .iter()
        .map(|w| w.to_string())
        .collect()
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
