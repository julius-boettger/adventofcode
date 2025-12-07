type Index = u8;
type Timelines = u64;
/// from the coords of one splitter, how many possible timeline are there?
type SplitterTimelinesMap = std::collections::HashMap<(Index, Index), Timelines>;

fn get_char(lines: &[&'static str], row: Index, col: Index) -> Option<char> {
   Some(char::from(lines.get(row as usize)?.as_bytes()[col as usize]))
}

fn timelines(lines: &[&'static str], start_row: Index, col: Index, splitter_timelines: &mut SplitterTimelinesMap) -> Timelines {
    let mut current_row = start_row;
    loop {
        let Some(c) = get_char(lines, current_row, col) else {
            return 0;
        };
        if c != '.' {
            break;
        }
        current_row += 2;
    }

    #[allow(clippy::option_if_let_else)]
    if let Some(timelines) = splitter_timelines.get(&(current_row, col)) {
        *timelines
    } else {
        let timelines = 1
            + timelines(lines, current_row + 2, col - 1, splitter_timelines)
            + timelines(lines, current_row + 2, col + 1, splitter_timelines);
        splitter_timelines.insert((current_row, col), timelines);
        timelines
    }
}

#[advent_of_code::main("25/07")]
fn main() {
    let input_lines = INPUT.lines().collect::<Vec<_>>();
    let start_col = Index::try_from(input_lines[0].chars().position(|c| c == 'S').unwrap()).unwrap();
    let mut splitter_timelines = SplitterTimelinesMap::new();
    let timelines = 1 + timelines(&input_lines, 2, start_col, &mut splitter_timelines);
    println!("{timelines}");
}
