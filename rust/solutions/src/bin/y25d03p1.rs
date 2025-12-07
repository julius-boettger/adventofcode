type JoltageSum = u16;

#[advent_of_code::main("25/03")]
fn main() {
    println!("{}", INPUT.lines()
        .map(|bank| {
            let bat_1_byte = bank[..bank.len()-1].bytes().max().unwrap();
            let bat_1_pos = bank.bytes().position(|b| b == bat_1_byte).unwrap();
            let bat_2_byte = bank[bat_1_pos+1..].bytes().max().unwrap();
            let joltage = ((bat_1_byte - b'0') * 10) + (bat_2_byte - b'0');
            JoltageSum::from(joltage)
        })
        .sum::<JoltageSum>()
    );
}
