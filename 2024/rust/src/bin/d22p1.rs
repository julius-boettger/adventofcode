type Num = u64;

fn next_secret_number(secret_number: Num) -> Num {
    let mut result = secret_number;
    result ^= result * 64;
    result %= 16777216;
    result ^= result / 32;
    result %= 16777216;
    result ^= result * 2048;
    result %= 16777216;
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
    let sum = advent_of_code::input!()
        .lines()
        .map(|l| l.parse::<Num>().unwrap())
        .map(|x| next_nth_secret_number(x))
        .sum::<u64>();

    println!("{}", sum);
}