type Num = u64;

fn get_char(lines: &[&'static str], row: usize, col: usize) -> char {
    char::from(lines[row].as_bytes()[col])
}

#[advent_of_code::main("25/06")]
fn main() {
    let input_lines = INPUT.lines().collect::<Vec<_>>();
    let ops_row = input_lines.iter().position(|l| l.starts_with('*') || l.starts_with('+')).unwrap();

    let mut current_op = '\0';
    let mut results: Vec<Num> = Vec::new();
    let mut current_result = 0;
    for col in 0..input_lines[0].len() {
        let op_char = get_char(&input_lines, ops_row, col);
        if op_char != ' ' {
            current_op = op_char;
            if !results.is_empty() {
                current_result += 1;
            }
        }

        let mut num = String::new();
        for row in 0..ops_row {
            let c = get_char(&input_lines, row, col);
            if c == ' ' {
                continue;
            }
            num.push(c);
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
