// WIP

#[advent_of_code::main("24/23")]
fn main() {
    INPUT.lines()
        // connections with a computer that starts with 't'
        .filter(|l| l.starts_with('t') || l.chars().nth(3) == Some('t'))
        .for_each(|l| {
            let computer_1 = l.get(0..2).unwrap();
            let computer_2 = l.get(3..5).unwrap();
            println!("{computer_1}, {computer_2}");
        });
}
