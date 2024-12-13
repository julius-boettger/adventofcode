// doesn't work yet, because perimeter is
// not counted correctly, it should be less

type Num = u32;
#[derive(Debug)]
struct Region {
    area: Num,
    perimeter: Num
}

type CoordNum = i16;
#[derive(Clone, Copy)]
struct Coord {
    row: CoordNum,
    col: CoordNum
}
impl std::ops::Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord {
            row: self.row + other.row,
            col: self.col + other.col
        }
    }
}
impl Coord {
    fn get_letter<'a>(&self, map: &'a Vec<Vec<char>>) -> &'a char {
        &map[self.row as usize][self.col as usize]
    }

    fn set_letter(&self, map: &mut Vec<Vec<char>>, letter: char) {
        map[self.row as usize][self.col as usize] = letter;
    }

    fn is_in_bounds(&self, map: &Vec<Vec<char>>) -> bool {
        self.row >= 0 &&
        self.col >= 0 &&
        (self.row as usize) < map.len() &&
        (self.col as usize) < map[0].len()
    }
}

const DIRECTIONS: [Coord; 4] = [
    Coord { row: -1, col:  0 }, // up
    Coord { row:  0, col:  1 }, // right
    Coord { row:  1, col:  0 }, // down
    Coord { row:  0, col: -1 }, // left
];

const EXPLORED: char = '.';

fn get_unexplored_region_start(map: &Vec<Vec<char>>) -> Option<Coord> {
    for row in 0 .. map.len() {
        for col in 0 .. map[0].len() {
            if map[row][col] != EXPLORED {
                return Some(Coord {
                    row: row as CoordNum,
                    col: col as CoordNum
                })
            }
        }
    }
    None
}

fn explore_region(map: &mut Vec<Vec<char>>, start: Coord) -> Region {
    let letter = &start.get_letter(map).clone();
    let mut region = Region { area: 0, perimeter: 0 };
    explore_recursive(map, start, letter, &mut region);
    region
}

fn explore_recursive(map: &mut Vec<Vec<char>>, coord: Coord, letter: &char, region: &mut Region) {
    if !coord.is_in_bounds(map) || coord.get_letter(map) != letter {
        region.perimeter += 1;
        return;
    }

    // mark this as explored
    coord.set_letter(map, EXPLORED);
    region.area += 1;

    for direction in DIRECTIONS {
        explore_recursive(map, coord + direction, letter, region);
    }
}

fn main() {
    let mut map: Vec<Vec<char>> = advent_of_code::input!()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut regions: Vec<Region> = vec![];
    while let Some(start) = get_unexplored_region_start(&map) {
        regions.push(explore_region(&mut map, start));
    }

    for region in &regions {
        println!("{:?}", region);
    }

    let price = regions
        .into_iter()
        .map(|r| r.area * r.perimeter)
        .sum::<Num>();

    println!("{:?}", price);
}
