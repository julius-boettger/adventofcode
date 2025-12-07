use std::collections::HashMap;

type Index = u8;
type Timelines = u64;
/// from the coords of one splitter, how many possible timeline are there?
type SplitterTimelinesCache = Cache<(Index, Index), Timelines>;

/// general-purpose cache for memoization
struct Cache<Input, Output> {
    map: HashMap<Input, Output>,
    #[cfg(debug_assertions)]
    accesses: u32,
    #[cfg(debug_assertions)]
    hits: u32,
}
impl<Input: Eq + Clone + std::hash::Hash, Output: Clone> Cache<Input, Output> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            #[cfg(debug_assertions)]
            accesses: 0,
            #[cfg(debug_assertions)]
            hits: 0
        }
    }
    /// if possible, returns the result from the cache,
    /// otherwise computes it and saves it to the cache
    fn get<F: Fn(Input, &mut Self) -> Output>(&mut self, input: Input, function: F) -> Output {
        #[cfg(debug_assertions)] {
            self.accesses += 1;
        }
        if let Some(result) = self.map.get(&input) {
            #[cfg(debug_assertions)] { self.hits += 1; }
            result.clone()
        } else {
            let result = function(input.clone(), self);
            self.map.insert(input, result.clone());
            result
        }
    }
}
/// print stats like cache hit rate on drop, but only when running a debug build
#[cfg(debug_assertions)]
impl<I, O> Drop for Cache<I, O> {
    #[allow(clippy::cast_precision_loss)]
    fn drop(&mut self) {
        println!("cache stats:");
        println!("- contains {} elements", self.map.len());
        println!("- {} accesses", self.accesses);
        println!("- {} hits", self.hits);
        println!("- {:.1}% hit rate ", ((self.hits as f32) / (self.accesses as f32)) * 100.0);
    }
}

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
