use itertools::{iproduct, Itertools};

fn main() {
    let resp = (0..4).permutations(2);

    resp.for_each(|xs| print!("{:?}", xs));
    println!("");

    iproduct!(0..10, 0..10).for_each(|xs| print!("{:?}", xs));
    println!("");

    let resc = (0..4).combinations(2);
    resc.for_each(|xs| print!("{:?}", xs));
    println!("");

    let recsr = (0..4).combinations_with_replacement(2);
    recsr.for_each(|xs| print!("{:?}", xs));
    println!("");
}

// permutations
// [0, 1][1, 0]
// combinations
// [0, 1]
// combinations_with_replacement
// [0, 0][0, 1][1, 1]
