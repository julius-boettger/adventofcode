use rayon::prelude::*;

type ID = u64;

fn is_invalid(id: ID) -> bool {
    let len = id.ilog10() + 1;

    if !len.is_multiple_of(2) {
        return false;
    }

    let split_indicator = ID::from(10u8).pow(len / 2);
    let half_1 = id / split_indicator;
    let half_2 = id - (half_1 * split_indicator);
    half_1 == half_2
}

#[advent_of_code::main("25/02")]
fn main() {
    println!("{}", INPUT.strip_suffix('\n').unwrap().par_split(',')
        .map(|id_range| {
            let (first, last) = id_range.split_once('-').unwrap();
            (first.parse::<ID>().unwrap() ..= last.parse::<ID>().unwrap())
                .filter(|id| is_invalid(*id))
                .sum::<ID>()
        })
        .sum::<ID>()
    );
}
