// doesnt work yet
// try with example input
// dont only check if the start of a range is contained in another range,
// also the end also needs to be greater than the start, otherwise they dont overlap

type ID = u64;

fn any_contain(ranges: &[std::ops::RangeInclusive<ID>], id: ID) -> Option<usize> {
    for (i, range) in ranges.iter().enumerate() {
        if range.contains(&id) {
            return Some(i);
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

        if let Some(i) = any_contain(&valid_ranges, start) {
            let start = *valid_ranges[i].start();
            let end = *valid_ranges[i].end().max(&end);
            valid_ranges[i] = start ..= end;
        } else {
            valid_ranges.push(start ..= end);
        }
    }

    valid_ranges.sort_unstable_by_key(|range| *range.end());

    println!("{:?}", valid_ranges[0]);
    println!("{:?}", valid_ranges[valid_ranges.len()-1]);

    for i in (0..valid_ranges.len()).rev() {
        let range = &valid_ranges[i];
        if let Some(contained_i) = any_contain(&valid_ranges, *range.start()) {
            let start = *valid_ranges[contained_i].start();
            let end = *valid_ranges[contained_i].end().max(range.end());
            valid_ranges[contained_i] = start ..= end;
            // should be fine because we iterate in reverse,
            // swapped elements have already been checked
            //valid_ranges.swap_remove(i);
            //valid_ranges.remove(i);

            if valid_ranges.len() == 1 {
                break;
            }
        }
    }

    println!("{valid_ranges:?}", );
}
