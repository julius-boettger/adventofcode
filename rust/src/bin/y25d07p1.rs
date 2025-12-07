type SplitterSet = std::collections::HashSet<(usize, usize)>;

fn get_char(lines: &[&'static str], row: usize, col: usize) -> Option<char> {
   Some(char::from(lines.get(row)?.as_bytes()[col]))
}

fn simulate_beam(lines: &[&'static str], start_row: usize, col: usize, splitter_set: &mut SplitterSet) {
    let mut current_row = start_row;
    loop {
        let Some(c) = get_char(lines, current_row, col) else {
            return;
        };
        if c != '.' {
            break;
        }
        current_row += 1;
    }

    if !splitter_set.insert((current_row, col)) {
        // the path of this splitter has already been checked
        return;
    }

    simulate_beam(lines, current_row, col - 1, splitter_set);
    simulate_beam(lines, current_row, col + 1, splitter_set);
}

#[advent_of_code::main("25/07")]
fn main() {
    let input_lines = INPUT.lines().collect::<Vec<_>>();
    let start_col = input_lines[0].chars().position(|c| c == 'S').unwrap();
    let mut splitter_set = SplitterSet::new();
    simulate_beam(&input_lines, 0, start_col, &mut splitter_set);
    println!("{}", splitter_set.len());
}
