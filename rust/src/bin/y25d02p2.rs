use rayon::prelude::*;

type ID = u64;

fn is_invalid(id: ID) -> bool {
    let id = id.to_string();
    let len = id.len();

    // TODO: early-detect cases like 1111?

    for chunk_size in 1 ..= len/2 {
        if len % chunk_size != 0 {
            continue;
        }

        let mut chunks = id.as_bytes().chunks(chunk_size);
        let first = chunks.next().unwrap();
        if chunks.all(|c| c == first) {
            return true;
        }
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
