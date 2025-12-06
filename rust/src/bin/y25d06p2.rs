type Num = u64;

fn get_char(input: &'static str, row: usize, col: usize) -> char {
    input.lines().nth(row).unwrap().chars().nth(col).unwrap()
}

#[advent_of_code::main("25/06")]
fn main() {
    let ops_row = INPUT.lines().position(|l| l.starts_with('*') || l.starts_with('+')).unwrap();

    let mut current_op = '\0';
    let mut results: Vec<Num> = Vec::new();
    let mut current_result = 0;
    for col in 0..INPUT.lines().next().unwrap().len() {
        let op_char = get_char(INPUT, ops_row, col);
        if op_char == '+' || op_char == '*' {
            current_op = op_char;
            if !results.is_empty() {
                current_result += 1;
            }
        }

        let mut num = String::new();
        for row in 0..ops_row {
            let c = get_char(INPUT, row, col);
            if c.is_ascii_digit() {
                num.push(c);
            }
        }

        if num.is_empty() {
            continue;
        }
        let num = num.parse::<Num>().unwrap();

        if results.get(current_result).is_none() {
            results.push(num);
        } else {
            match current_op {
                '+' => results[current_result] += num,
                '*' => results[current_result] *= num,
                _ => unreachable!()
            }
        }
    }

    let sum = results.iter().sum::<Num>();

    println!("{sum}");
}
