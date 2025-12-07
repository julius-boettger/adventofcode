type Num = u64;

#[advent_of_code::main("25/06")]
fn main() {
    let ops_input_line_i = INPUT.lines().position(|l| l.starts_with('*') || l.starts_with('+')).unwrap();
    let ops = INPUT.lines()
        .nth(ops_input_line_i).unwrap()
        .split_ascii_whitespace()
        .map(|op_str| op_str.chars().next().unwrap())
        .collect::<Vec<_>>();

    let sum = INPUT.lines()
        .take(ops_input_line_i) // until operators
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<Num>().unwrap())
        })
        .fold(Vec::new(), |acc, nums| {
            if acc.is_empty() {
                return nums.collect();
            }

            let mut result = acc;
            for (i, num) in nums.enumerate() {
                match ops[i] {
                    '+' => result[i] += num,
                    '*' => result[i] *= num,
                    _ => unreachable!()
                }
            }
            result
        })
        .iter()
        .sum::<Num>();

    println!("{sum:?}");
}
