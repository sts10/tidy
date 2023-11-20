use std::collections::HashSet;

/// Return a Vector of words that "caused" the Sardinas- Patterson algorithm to
/// determine that this list was not uniquely decodable.
/// These "offending" words can then be removed from the original
/// list to, theoretically, make the list uniquely decodable.
pub fn get_sardinas_patterson_final_intersection<T: AsRef<str>>(c: &[T]) -> Vec<String> {
    // Convert c to a HashSet, I think
    let c = c.iter().map(|f| f.as_ref()).collect();

    let c_infinity = generate_c_infinity_with_a_halt_break(&c);
    // We want to collect a list of words that "caused" the Sardinas-Patterson algorithm
    // to determine that this list was not uniquely decodable.
    // If the given list is in fact uniquely decodable, this list of words will be empty.
    // If there are words in the list, we'll return those to src/lib to be
    // removed from the final list.
    let final_intersection = c.intersection(&c_infinity);
    Vec::from_iter(final_intersection)
        .iter()
        .map(|w| w.to_string())
        .collect()
}

/// Generate c for any number n
fn generate_cn<'a>(c: &HashSet<&'a str>, cn_minus_1: &HashSet<&'a str>) -> HashSet<&'a str> {
    let mut cn = HashSet::new();

    for w1 in c.iter() {
        for w2 in cn_minus_1.iter() {
            if w1.len() > w2.len() && w1.starts_with(w2) {
                // w2 is a prefix word of w1
                // so, we're going to add the dangling suffix to a new HashSet
                // called cn
                cn.insert(&w1[w2.len()..]);
            }
            if w2.len() > w1.len() && w2.starts_with(w1) {
                // w1 is a prefix word of w2
                // so, we're going to add the dangling suffix to a new HashSet
                // called cn
                cn.insert(&w2[w1.len()..]);
            }
        }
    }
    cn
}

fn generate_c_infinity_with_a_halt_break<'a>(c: &'a HashSet<&str>) -> HashSet<&'a str> {
    let mut cn = generate_cn(c, c);
    let mut cs = cn.clone();

    loop {
        cn = generate_cn(c, &cn);
        let prior = cs.len();
        cs.extend(&cn);
        if cs.len() == prior {
            // if the set size did not increase, cn is a subset
            // Cycle detected. Halting algorithm.
            break;
        }
    }
    cs
}
