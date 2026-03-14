#![feature(portable_simd)]

use rayon::prelude::*;
use std::simd::prelude::*;

type Num = u64;

const LANES: usize = 16;
type SimdNum = Simd<u64, LANES>;

fn next_secret_number(secret_number: SimdNum) -> SimdNum {
    const AND_MASK: SimdNum = SimdNum::from_array([0x00FF_FFFF; LANES]);

    let mut result = secret_number;
    result ^= result << 6;
    result &= AND_MASK;
    result ^= result >> 5;
    result &= AND_MASK;
    result ^= result << 11;
    result &= AND_MASK;
    result
}

fn next_nth_secret_number(simd_num: SimdNum) -> SimdNum {
    let mut result = simd_num;
    for _ in 0 .. 2000 {
        result = next_secret_number(result);
    }
    result
}

#[advent_of_code::main("24/22")]
fn main() {
    let lines = INPUT.lines().collect::<Vec<_>>();
    println!("{}", lines.par_chunks(LANES)
        .map(|chunk| {
            let mut parsed = chunk.par_iter()
                .map(|line| line.parse::<Num>().unwrap())
                .collect::<Vec<_>>();

            // make sure every chunk has LANES values
            if parsed.len() != LANES {
                parsed.resize(LANES, 0);
            }

            let simd_num = SimdNum::from_slice(&parsed);
            let simd_result = next_nth_secret_number(simd_num);
            simd_result.reduce_sum()
        })
        .sum::<Num>()
    );
}
