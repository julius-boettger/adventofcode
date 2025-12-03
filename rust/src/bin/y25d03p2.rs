// doesnt work yet, wrong on example input.
// joltage calculation is correct, battery_bytes selection loop is wrong.

type JoltageSum = u64;

/// batteries per pank to turn on
const BATS: usize = 12;

#[advent_of_code::main("25/03")]
fn main() {
    println!("{}", INPUT.lines()
        .map(|bank| {
            let mut battery_bytes = [0; BATS];
            battery_bytes[BATS-1] = bank[..bank.len() - (BATS-1)].bytes().max().unwrap();
            for i in (0..(BATS-1)).rev() {
                let last_pos = bank.bytes().position(|b| b == *battery_bytes.last().unwrap()).unwrap();
                battery_bytes[i] = bank[last_pos+1 .. bank.len()-i].bytes().max().unwrap();
            }
            
            let joltage = battery_bytes.into_iter()
                .enumerate()
                .map(|(i, byte)|
                    JoltageSum::from(byte - b'0') * JoltageSum::from(10u8).pow(i.try_into().unwrap()))
                .sum::<JoltageSum>();
            println!("{bank} => {joltage}");
            joltage
        })
        .sum::<JoltageSum>()
    );
}
