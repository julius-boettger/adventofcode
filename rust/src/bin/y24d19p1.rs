#[advent_of_code::main("24/19")]
fn main() {
    let (patterns_input, designs_input) = INPUT.split_once("\n\n").unwrap();

    let patterns = patterns_input.split(", ").collect::<Vec<_>>();

    let mut possible_designs: u8 = 0;
    for design in designs_input.lines() {
        // advance pointer through chars of design
        // push possible options at each position with a choice onto a stack 
        // if no more options: back-track until stack exhausted
    }
}
