use rayon::prelude::*;

type ID = u64;

fn is_invalid(id: ID) -> bool {
    let id = id.to_string();
    let len = id.len();

    if len % 2 != 0 {
        return false;
    }

    let half_1 = &id[..len/2  ];
    let half_2 = &id[  len/2..];
    if half_1 == half_2 {
        return true;
    }

    false
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
