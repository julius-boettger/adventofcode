type CoordNum = i8;

#[derive(Clone, Copy, Debug)]
struct Coord {
    row: CoordNum,
    col: CoordNum
}
impl std::ops::Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col
        }
    }
}
impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}
impl Coord {
    const fn from_char(c: char) -> &'static Self {
        match c {
            '<' => &LEFT,
            'v' => &DOWN,
            '>' => &RIGHT,
            '^' => &UP,
            _ => panic!()
        }
    }
    fn set_char(self, grid: &mut [Vec<char>], c: char) {
        grid[usize::try_from(self.row).unwrap()]
            [usize::try_from(self.col).unwrap()] = c;
    }
    fn get_char(self, grid: &[Vec<char>]) -> &char {
        &grid[usize::try_from(self.row).unwrap()]
             [usize::try_from(self.col).unwrap()]
    }
}

const LEFT:  Coord = Coord { row:  0, col: -1 };
const DOWN:  Coord = Coord { row:  1, col:  0 };
const RIGHT: Coord = Coord { row:  0, col:  1 };
const UP:    Coord = Coord { row: -1, col:  0 };

fn get_robot(grid: &[Vec<char>]) -> Coord {
    for row in 0 .. grid.len() {
        for col in 0 .. grid[0].len() {
            if grid[row][col] == '@' {
                return Coord {
                    row: row.try_into().unwrap(),
                    col: col.try_into().unwrap()
                }
            }
        }
    }
    panic!()
}

// returns new position of robot
fn move_robot(grid: &mut [Vec<char>], robot: Coord, direction: Coord) -> Coord {
    let mut next_coord = robot;
    let mut next_char;
    let mut moves = 0;
    loop {
        next_coord += direction;
        next_char = next_coord.get_char(grid);
        moves += 1;
        
        // wall or free space
        if *next_char == '#' || *next_char != 'O' {
            break;
        }
    }

    // there is no space to move in the direction
    if *next_char == '#' {
        return robot;
    }

    // there is free space right next to the starting position
    if moves == 1 {
        return next_coord;
    }

    // there is free space somewhere in the direction,
    // but there are obstacles to move before it

    let next_to_robot = robot + direction;
    // remove obstacle
    next_to_robot.set_char(grid, '.');
    // put obstacle on next free space in direction
    next_coord.set_char(grid, 'O');
    // move robot to the new free space
    next_to_robot
}

fn main() {
    let input= advent_of_code::input!();

    let mut grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| l.contains('#'))
        .map(|l| l
            .chars()
            .collect())
        .collect();

    let directions: Vec<&Coord> = input
        .lines()
        .filter(|l| l.contains('v'))
        .flat_map(|l| l
            .chars()
            .map(Coord::from_char)
            .collect::<Vec<_>>())
        .collect();

    let mut robot = get_robot(&grid);
    robot.set_char(&mut grid, '.');

    for direction in directions {
        robot = move_robot(&mut grid, robot, *direction);
    }

    let mut box_gps_sum = 0;
    for row in 0 .. grid.len() {
        for col in 0 .. grid[0].len() {
            if grid[row][col] == 'O' {
                box_gps_sum += (100 * row) + col;
            }
        }
    }

    println!("{box_gps_sum}");
}
