// doesn't work yet, because separate regions with
// the same letter need to be priced independently

use std::collections::HashMap;

type Num = u32;

#[derive(Debug)]
struct Region {
    area: Num,
    perimeter: Num
}

fn process_area(plot: &char, region_map: &mut HashMap<char, Region>) {
    match region_map.get_mut(plot) {
        Some(region) => { region.area += 1; }
        None => {
            region_map.insert(*plot, Region {
                area: 1,
                perimeter: 0,
            });
        }
    }
}

fn process_perimeter(plot: &char, next_plot: &char, region_map: &mut HashMap<char, Region>) {
    if plot == next_plot {
        return;
    }

    for p in [plot, next_plot] {
        region_map.get_mut(&p).unwrap().perimeter += 1;
    }
}

fn main() {
    let map: Vec<Vec<char>> = advent_of_code::input!()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    // key: name of region
    // value: stats about region
    let mut region_map: HashMap<char, Region> = HashMap::new();

    // iterate through rows
    for row in 0 .. map.len() {
        let max_col = map[0].len() - 2;
        for col in 0 ..= max_col {
            let plot = map[row][col];
            let next_plot = map[row][col + 1];

            if col == 0 {
                process_area(&plot, &mut region_map);
                // add extra fence at start
                region_map.get_mut(&plot).unwrap().perimeter += 1;
            }

            process_area(&next_plot, &mut region_map);
            process_perimeter(&plot, &next_plot, &mut region_map);

            // add extra fence at end
            if col == max_col {
                region_map.get_mut(&next_plot).unwrap().perimeter += 1;
            }
        }
    }

    // iterate through columns
    for col in 0 .. map[0].len() {
        let max_row = map.len() - 2;
        for row in 0 ..= max_row {
            let plot = map[row][col];
            let next_plot = map[row + 1][col];

            // add extra fence at start
            if row == 0 {
                region_map.get_mut(&plot).unwrap().perimeter += 1;
            }

            process_perimeter(&plot, &next_plot, &mut region_map);

            // add extra fence at end
            if row == max_row {
                region_map.get_mut(&next_plot).unwrap().perimeter += 1;
            }
        }
    }

    let price = region_map
        .values()
        .map(|r| r.area * r.perimeter)
        .sum::<Num>();

    for pair in region_map {
        println!("{}: {} x {}", pair.0, pair.1.area, pair.1.perimeter);
    }
    
    println!("{:?}", price);
}