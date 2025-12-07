type Index = u8;
type CoordSet = std::collections::HashSet<(Index, Index)>;

fn get_char(lines: &[&'static str], row: Index, col: Index) -> Option<char> {
   Some(char::from(lines.get(row as usize)?.as_bytes()[col as usize]))
}

fn simulate_beam(lines: &[&'static str], start_row: Index, col: Index, activated_splitters: &mut CoordSet) {
    let mut current_row = start_row;
    loop {
        let Some(c) = get_char(lines, current_row, col) else {
            return;
        };
        if c != '.' {
            break;
        }
        current_row += 2;
    }

    if !activated_splitters.insert((current_row, col)) {
        // the path of this splitter has already been checked
        return;
    }

    simulate_beam(lines, current_row + 2, col - 1, activated_splitters);
    simulate_beam(lines, current_row + 2, col + 1, activated_splitters);
}

#[advent_of_code::main("25/07")]
fn main() {
    let input_lines = INPUT.lines().collect::<Vec<_>>();
    let start_col = Index::try_from(input_lines[0].chars().position(|c| c == 'S').unwrap()).unwrap();
    let mut activated_splitters = CoordSet::new();
    simulate_beam(&input_lines, 2, start_col, &mut activated_splitters);
    println!("{}", activated_splitters.len());
}
