type ID = u64;

#[advent_of_code::main("25/05")]
fn main() {
    let (input_ranges, input_ids) = INPUT.split_once("\n\n").unwrap();

    let valid_ranges = input_ranges.lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse::<ID>().unwrap() ..= end.parse::<ID>().unwrap()
        })
        .collect::<Vec<_>>();

    let mut valid_ids: u16 = 0;
    for id in input_ids.lines() {
        let id = id.parse::<ID>().unwrap();
        for valid_range in &valid_ranges {
            if valid_range.contains(&id) {
                valid_ids += 1;
                break;
            }
        }
    }

    println!("{valid_ids}");
}
