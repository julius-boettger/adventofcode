type JoltageSum = u64;

/// batteries per pank to turn on
const BATS: usize = 12;

fn max_byte(bank: &str) -> (u8, usize) {
    let mut max_byte = 0;
    let mut max_pos = 0;
    for (pos, byte) in bank.bytes().enumerate() {
        if byte > max_byte {
            max_byte = byte;
            max_pos = pos;
        }
    }
    (max_byte, max_pos)
}

#[advent_of_code::main("25/03")]
fn main() {
    println!("{}", INPUT.lines()
        .map(|bank| {
            let mut battery_bytes = [0; BATS];
            let mut battery_positions = [0; BATS];

            let (first_byte, first_byte_pos) = max_byte(&bank[..bank.len() - (BATS-1)]);
            battery_bytes[BATS-1] = first_byte;
            battery_positions[BATS-1] = first_byte_pos;

            for i in (0..(BATS-1)).rev() {
                let last_pos = battery_positions[i+1];
                let relevant_bank = &bank[last_pos+1 .. bank.len()-i];
                let (byte, pos) = max_byte(relevant_bank);
                battery_bytes[i] = byte;
                battery_positions[i] = pos + (last_pos+1);
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
