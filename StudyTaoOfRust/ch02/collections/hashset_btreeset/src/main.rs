use std::collections::BTreeSet;
use std::collections::HashSet;

fn main() {
    let mut hbooks = HashSet::new();
    let mut bbooks = BTreeSet::new();

    hbooks.insert("A Song of Ice and Fire");
    hbooks.insert("The Emerald City");
    hbooks.insert("The Odyssey");

    bbooks.insert("A Song of Ice and Fire");
    bbooks.insert("The Emerald City");
    bbooks.insert("The Odyssey");

    println!("{:?}", hbooks);
    println!("{:?}", bbooks);

    if hbooks.contains("The Emerald City") {
        println!("we have {} books, one is: The Emerald City", hbooks.len());
    }
}
