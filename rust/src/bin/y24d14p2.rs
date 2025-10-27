type CoordNum = i16;
const ROWS: CoordNum = 103;
const COLS: CoordNum = 101;

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

fn write_image(robots: &Vec<Robot>, iteration: u16) {
    let mut img = image::GrayImage::new(
        COLS.try_into().unwrap(),
        ROWS.try_into().unwrap()
    );

    for robot in robots {
        img.put_pixel(
            robot.position.col.try_into().unwrap(),
            robot.position.row.try_into().unwrap(),
            image::Luma([255]));
    }

    img.save(format!("generated/{iteration}.png")).unwrap();
}

fn main() {
    // construct vec of robots from input
    let mut robots: Vec<Robot> = vec![];
    let pattern = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in advent_of_code::input!().lines() {
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

    for i in 1 ..= 10_000u16 {
        for robot in &mut robots {
            robot.advance();
        }
        write_image(&robots, i);
    }

    // correct answer determined manually by looking through generated pictures: 7569
}
