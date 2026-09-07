use std::collections::BTreeMap;

fn counts(text: &str) -> BTreeMap<char, usize> {
    let mut found = BTreeMap::new();
    for one in text.chars() {
        *found.entry(one).or_insert(0) += 1;
    }
    found
}

fn main() {
    println!("{:?}", counts("abracadabra"));
}
