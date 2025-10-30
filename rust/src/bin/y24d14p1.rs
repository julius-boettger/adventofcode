type CoordNum = i16;
const ROWS: CoordNum = 103;
const COLS: CoordNum = 101;
const MAX_ROW: CoordNum = ROWS - 1;
const MAX_COL: CoordNum = COLS - 1;
const CENTER_ROW: CoordNum = MAX_ROW / 2;
const CENTER_COL: CoordNum = MAX_COL / 2;

#[derive(Clone, Copy, Debug)]
struct Coord {
    row: CoordNum,
    col: CoordNum
}
impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

#[derive(Debug)]
struct Robot {
    position: Coord,
    velocity: Coord
}
impl Robot {
    fn advance(&mut self) {
        self.position += self.velocity;
        for (value, max) in [
            (&mut self.position.row, ROWS),
            (&mut self.position.col, COLS)
        ] {
            if *value >= max {
                *value -= max;
            }
            if *value < 0 {
                *value += max;
            }
        }
    }
}

#[derive(Debug)]
struct Quadrant {
    // top left
    min: Coord,
    // bottom right
    max: Coord
}

const QUADRANTS: [Quadrant; 4] = [
    // top left
    Quadrant {
        min: Coord {
            row: 0,
            col: 0,
        },
        max: Coord {
            row: CENTER_ROW - 1,
            col: CENTER_COL - 1,
        }
    },
    // top right
    Quadrant {
        min: Coord {
            row: 0,
            col: CENTER_COL + 1,
        },
        max: Coord {
            row: CENTER_ROW - 1,
            col: MAX_COL,
        }
    },
    // bottom left
    Quadrant {
        min: Coord {
            row: CENTER_ROW + 1,
            col: 0,
        },
        max: Coord {
            row: MAX_ROW,
            col: CENTER_COL - 1,
        }
    },
    // bottom right
    Quadrant {
        min: Coord {
            row: CENTER_ROW + 1,
            col: CENTER_COL + 1,
        },
        max: Coord {
            row: MAX_ROW,
            col: MAX_COL,
        }
    },
];

#[advent_of_code::main]
fn main() {
    // construct vec of robots from input
    let mut robots: Vec<Robot> = vec![];
    let pattern = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in include_str!("../../input/24/14.txt").lines() {
        let Some(capture_groups) = pattern.captures(line) else {
            panic!("pattern doesn't match line: {line}");
        };
        robots.push(Robot {
            position: Coord {
                col: capture_groups[1].parse().unwrap(),
                row: capture_groups[2].parse().unwrap()
            },
            velocity: Coord {
                col: capture_groups[3].parse().unwrap(),
                row: capture_groups[4].parse().unwrap()
            }
        });
    }

    for _ in 0 .. 100 {
        for robot in &mut robots {
            robot.advance();
        }
    }

    // start with 1 for multiplication
    let mut safety_factor = 1;
    for quad in QUADRANTS {
        safety_factor *= robots
            .iter()
            .filter(|r|
                (quad.min.row ..= quad.max.row).contains(&r.position.row) &&
                (quad.min.col ..= quad.max.col).contains(&r.position.col))
            .count();
    }

    println!("{safety_factor}");
}
