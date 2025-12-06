type ID = u64;
type IDRange = std::ops::RangeInclusive<ID>;

/// check if a `new_range` can be represented by merging it with an existing range.
/// if so, return the index and the new value of the range to modify.
fn mergeable(ranges: &[IDRange], new_range: &IDRange, skip_i: Option<usize>) -> Option<(usize, IDRange)> {
    for (i, iter_range) in ranges.iter().enumerate() {
        if let Some(skip_i) = skip_i && i == skip_i {
            continue;
        }

        if iter_range == new_range {
            return Some((i, iter_range.clone()));
        }

        // check both possible orders
        for (range_1, range_2) in [(iter_range, new_range), (new_range, iter_range)] {
            if !range_1.contains(range_2.start()) {
                continue;
            }

            // range_1: [-----]
            // range_2:  [---]
            if range_1.contains(range_2.end()) {
                return Some((i, range_1.clone()));
            }

            // range_1: [---]
            // range_2:   [---]
            if range_2.contains(range_1.end()) {
                return Some((i, *range_1.start() ..= *range_2.end()));
            }
        }
    }

    None
}

#[advent_of_code::main("25/05")]
fn main() {
    let (input_ranges, _) = INPUT.split_once("\n\n").unwrap();

    let mut valid_ranges = Vec::new();
    for line in input_ranges.lines() {
        let (start, end) = line.split_once('-').unwrap();
        let start = start.parse::<ID>().unwrap();
        let end = end.parse::<ID>().unwrap();
        let range = start ..= end;

        if let Some((i, modified_range)) = mergeable(&valid_ranges, &range, None) {
            valid_ranges[i] = modified_range;
        } else {
            valid_ranges.push(range);
        }
    }

    valid_ranges.sort_unstable_by_key(|range| *range.end());

    'outer: loop {
        let mut any_modified = false;

        for iter_i in (0..valid_ranges.len()).rev() {
            let mergeable = mergeable(&valid_ranges, &valid_ranges[iter_i], Some(iter_i));
            let Some((modify_i, modified_range)) = mergeable else {
                continue;
            };

            valid_ranges[modify_i] = modified_range;
            any_modified = true;
            // should be fine because we iterate in reverse,
            // swapped elements have already been checked
            //valid_ranges.swap_remove(i);
            valid_ranges.remove(iter_i);

            if valid_ranges.len() == 1 {
                break 'outer;
            }
        }

        if !any_modified {
            break;
        }
    }

    let valid_ids = valid_ranges.into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<ID>();

    println!("{valid_ids}");
}
