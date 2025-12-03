// doesnt work yet, example input works, real input doesnt

#[advent_of_code::main("25/01")]
fn main() {
    let mut password: u16 = 0;
    let mut position = 50;

    for line in INPUT.lines() {
        let direction_factor = if line.starts_with('L') { -1 } else { 1 };
        let distance: i16 = line[1..].parse().unwrap();

        position += distance * direction_factor;

        while position >= 100 {
            position -= 100;
            password += 1;
        }
        while position < 0 {
            position += 100;
            password += 1;
        }
    }

    println!("{password}");
}
