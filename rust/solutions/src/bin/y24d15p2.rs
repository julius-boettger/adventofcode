// doesn't work yet, don't know why,
// example input works, real input doesn't

type CoordNum = i8;
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[allow(clippy::cast_sign_loss)]
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
        grid[self.row as usize][self.col as usize] = c;
    }
    fn get_char(self, grid: &[Vec<char>]) -> &char {
        &grid[self.row as usize][self.col as usize]
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
                    row: CoordNum::try_from(row).unwrap(),
                    col: CoordNum::try_from(col).unwrap()
                }
            }
        }
    }
    panic!()
}

/// returns new position of robot
fn move_robot(grid: &mut Vec<Vec<char>>, robot: Coord, direction: Coord) -> Coord {
    // check right next to starting position
    let next_coord = robot + direction;
    match next_coord.get_char(grid) {
        // wall, can't move
        '#' => return robot,
        // free space! move there
        '.' => return next_coord,
        // continue
        _ => ()
    }

    // there is at least one obstacle in front of the robot

    let mut obstacles = Some(vec![]);
    get_obstacles(grid, next_coord, direction, &mut obstacles);
    
    // obstacles can't be moved, so we also can't move
    if obstacles.is_none() {
        return robot;
    }

    // obstacles can be moved! move them
    move_obstacles(grid, &obstacles.unwrap(), direction);

    // move robot to the new free space
    robot + direction
}

fn get_obstacles(grid: &Vec<Vec<char>>, coord: Coord, direction: Coord, obstacles: &mut Option<Vec<Coord>>) {
    // result has already been determined, obstacles cant be moved
    if obstacles.is_none() {
        return;
    }

    let obstacle= match coord.get_char(grid) {
        '[' => coord,
        ']' => coord + LEFT,
         _  => panic!()
    };

    match obstacle_can_be_moved(grid, obstacle, direction) {
        // can be moved! free space after it!
        0 => {
            obstacles.as_mut().unwrap().push(obstacle);
            return;
        },
        // there are more obstacles in the way, continue
        1 => (),
        // can't be moved, wall after it
        2 => {
            *obstacles = None;
            return;
        },
        // shouldn't happen
        _ => panic!()
    }

    // there is at least one more obstacle after the current one
    // push the current one for now
    obstacles.as_mut().unwrap().push(obstacle);

    // recursively check all elements in the direction of the current obstacle
    for element in next_to_obstacle(obstacle, direction) {
        if matches!(element.get_char(grid), '[' | ']') {
            get_obstacles(grid, element, direction, obstacles);
        }
    }
}

fn next_to_obstacle(obstacle: Coord, direction: Coord) -> Vec<Coord> {
    match direction {
        LEFT => vec![obstacle + LEFT],
        RIGHT => vec![obstacle + RIGHT + RIGHT],
        UP | DOWN => vec![obstacle + direction, obstacle + RIGHT + direction],
        _ => panic!()
    }
}

/// 0 => can definitely be moved (free space after)
/// 1 => can maybe be moved (obstacle in the way)
/// 2 => can definitely not be moved (wall in the way)
fn obstacle_can_be_moved(grid: &[Vec<char>], obstacle: Coord, direction: Coord) -> u8 {
    let chars: Vec<char> = next_to_obstacle(obstacle, direction)
        .iter()
        .map(|c| *c.get_char(grid))
        .collect();

    if chars.contains(&'#') {
        return 2;
    }

    if chars.iter().any(|c| matches!(*c, '[' | ']')) {
        return 1;
    }
    
    0
}

fn move_obstacles(grid: &mut [Vec<char>], obstacles: &[Coord], direction: Coord) {
    // move obstacle furthest away from the robot first
    // to make space for the one before it and so on
    for obstacle in obstacles.iter().rev() {
        // left and right coord of obstacle
        let mut elements= vec![*obstacle, *obstacle + RIGHT];
        // free space
        for e in &elements { e.set_char(grid, '.'); }
        // move obstacle
        for e in &mut elements { *e += direction; }
        // occupy new space
        elements[0].set_char(grid, '[');
        elements[1].set_char(grid, ']');
    }
}

#[advent_of_code::main("24/15")]
fn main() {
    let mut grid: Vec<Vec<char>> = INPUT
        .lines()
        .filter(|l| l.contains('#'))
        .map(|l| l
            .chars()
            .flat_map(|c|
                match c {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                     _  => panic!()
                })
            .collect())
        .collect();

    let directions: Vec<&Coord> = INPUT
        .lines()
        .filter(|l| l.contains('v'))
        .flat_map(|l| l
            .chars()
            .map(Coord::from_char)
            .collect::<Vec<&Coord>>())
        .collect();

    let mut robot = get_robot(&grid);
    robot.set_char(&mut grid, '.');

    for direction in directions {
        //robot.set_char(&mut grid, '.');
        robot = move_robot(&mut grid, robot, *direction);
        //robot.set_char(&mut grid, '@');
        //for row in &grid {
        //    for c in row {
        //        print!("{}", c);
        //    }
        //    println!();
        //}
        //println!();
    }

    let mut box_gps_sum = 0;
    for row in 0 .. grid.len() {
        for col in 0 .. grid[0].len() {
            if grid[row][col] == '[' {
                box_gps_sum += (100 * row) + col;
            }
        }
    }

    println!("{box_gps_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid1() -> Vec<Vec<char>> {
        vec![
            "##########",
            "##....#.##",
            "##.[][].##",
            "##.#[]..##",
            "##......##",
            "##########",
        ].into_iter().map(|r| r.chars().collect()).collect()
    }

    mod next_to_obstacle {
        use super::*;
        fn test(direction: Coord, expected: &[Coord]) {
            let obstacle = Coord { row: 0, col: 0 };
            let elements = next_to_obstacle(obstacle, direction);
            assert_eq!(elements, expected);
        }

        #[test] fn up() {
            test(UP,
                &[Coord { row: -1, col: 0 }, Coord { row: -1, col: 1 }]);
        }
        #[test] fn down() {
            test(DOWN,
                &[Coord { row: 1, col: 0 }, Coord { row: 1, col: 1 }]);
        }
        #[test] fn left() {
            test(LEFT, &[Coord { row: 0, col: -1 }]);
        }
        #[test] fn right() {
            test(RIGHT, &[Coord { row: 0, col: 2 }]);
        }
    }

    mod obstacle_can_be_moved {
        use super::*;
        fn test(direction: Coord, expected: u8) {
            let grid = grid1();
            let obstacle = Coord { row: 2, col: 3 };
            assert_eq!(obstacle_can_be_moved(&grid, obstacle, direction), expected);
        }

        #[test] fn up() { test(UP, 0); }
        #[test] fn right() { test(RIGHT, 1); }
        #[test] fn down() { test(DOWN, 2); }
        #[test] fn left() { test(LEFT, 0); }
    }

    mod get_obstacles {
        use super::*;
        #[allow(clippy::needless_pass_by_value)]
        fn test(direction: Coord, expected: Option<Vec<Coord>>) {
            let grid = grid1();
            let obstacle = Coord { row: 2, col: 3 };
            let mut obstacles = Some(vec![]);
            get_obstacles(&grid, obstacle, direction, &mut obstacles);
            assert_eq!(obstacles, expected);
        }

        #[test] fn up() {
            test(UP, Some(vec![Coord { row: 2, col: 3 }]));
        }
        #[test] fn down() {
            test(DOWN, None);
        }
        #[test] fn left() {
            test(LEFT, Some(vec![Coord { row: 2, col: 3 }]));
        }
        #[test] fn right() {
            test(RIGHT,
                Some(vec![Coord { row: 2, col: 3 }, Coord { row: 2, col: 5 }]));
        }
    }
}
