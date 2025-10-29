use rayon::prelude::*;

type Num = u64;

const fn next_secret_number(secret_number: Num) -> Num {
    let mut result = secret_number;
    result ^= result * 64;
    result %= 16_777_216;
    result ^= result / 32;
    result %= 16_777_216;
    result ^= result * 2048;
    result %= 16_777_216;
    result
}

fn next_nth_secret_number(secret_number: Num) -> Num {
    let mut result = secret_number;
    for _ in 0 .. 2000 {
        result = next_secret_number(result);
    }
    result
}

fn main() {
    println!("{}", include_str!("../../input/24/22.txt").lines()
        // collect to let rayon do its magic
        .collect::<Vec<_>>().par_iter()
        .map(|line|
            next_nth_secret_number(
                line.parse::<Num>().unwrap()))
        .sum::<Num>()
    );
}