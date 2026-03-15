#![feature(portable_simd)]
#![feature(iter_array_chunks)]

use std::simd::prelude::*;

type Num = u64;

// had the best runtime in my testing
const LANES: usize = 64;
type SimdNum = Simd<Num, LANES>;

#[allow(clippy::large_types_passed_by_value)]
fn next_nth_secret_number(nums: [Num; LANES]) -> SimdNum {
    // 24 bit, AND with this is same as modulo 2^24 = 16777215
    const AND_MASK: SimdNum = SimdNum::from_array([0x00FF_FFFF; LANES]);

    let mut result = SimdNum::from_array(nums);
    for _ in 0 .. 2000 {
        result ^= result << 6;
        result &= AND_MASK;
        result ^= result >> 5;
        result &= AND_MASK;
        result ^= result << 11;
        result &= AND_MASK;
    }
    result
}

#[advent_of_code::main("24/22")]
fn main() {
    let base_result = INPUT.lines()
        .array_chunks::<LANES>()
        .map(|chunk| {
            let parsed = chunk.map(|line| line.parse::<Num>().unwrap());
            next_nth_secret_number(parsed).reduce_sum()
        })
        .sum::<Num>();

    let remaining_count = INPUT.lines().count() % LANES;
    let remainder_result = if remaining_count == 0 { 0 } else {
        let mut remaining = INPUT.lines()
            .rev()
            .take(remaining_count)
            .map(|line| line.parse::<Num>().unwrap())
            .collect::<Vec<_>>();
        remaining.resize(LANES, 0); // fill with default value that doesn't impact sum
        next_nth_secret_number(remaining.try_into().unwrap()).reduce_sum()
    };

    println!("{}", base_result + remainder_result);
}
