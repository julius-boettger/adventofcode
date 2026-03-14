use rayon::prelude::*;

type Num = u64;

const fn next_secret_number(secret_number: Num) -> Num {
    let mut result = secret_number;
    result ^= result << 6;
    result &= 0x00FF_FFFF;
    result ^= result >> 5;
    result &= 0x00FF_FFFF;
    result ^= result << 11;
    result &= 0x00FF_FFFF;
    result
}

fn next_nth_secret_number(secret_number: Num) -> Num {
    let mut result = secret_number;
    for _ in 0 .. 2000 {
        result = next_secret_number(result);
    }
    result
}

#[advent_of_code::main("24/22")]
fn main() {
    println!("{}", INPUT
        .par_lines()
        .map(|line| next_nth_secret_number(line.parse::<Num>().unwrap()))
        .sum::<Num>()
    );
}
