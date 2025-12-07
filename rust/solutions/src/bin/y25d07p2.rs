use util::Cache;

type Index = u8;
type Timelines = u64;
/// from the coords of one splitter, how many possible timeline are there?
type SplitterTimelinesCache = Cache<(Index, Index), Timelines>;

fn get_char(lines: &[&'static str], row: Index, col: Index) -> Option<char> {
   Some(char::from(lines.get(row as usize)?.as_bytes()[col as usize]))
}

fn timelines(lines: &[&'static str], start_row: Index, col: Index, cache: &mut SplitterTimelinesCache) -> Timelines {
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

    cache.get((current_row, col), |input, cache| {
        1 + timelines(lines, input.0 + 2, input.1 - 1, cache)
          + timelines(lines, input.0 + 2, input.1 + 1, cache)
    })
}

#[advent_of_code::main("25/07")]
fn main() {
    let input_lines = INPUT.lines().collect::<Vec<_>>();
    let start_col = Index::try_from(input_lines[0].chars().position(|c| c == 'S').unwrap()).unwrap();
    let mut cache = SplitterTimelinesCache::new();
    let timelines = 1 + timelines(&input_lines, 2, start_col, &mut cache);
    println!("{timelines}");
}
