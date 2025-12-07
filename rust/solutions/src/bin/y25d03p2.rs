type JoltageSum = u64;

/// batteries per pank to turn on
const BATS: usize = 12;

fn max_byte_pos(bank: &str) -> usize {
    let mut max = (0, 0);
    for (pos, byte) in bank.bytes().enumerate() {
        if byte > max.0 {
            max = (byte, pos);
            if byte == b'9' {
                return pos;
            }
        }
    }
    max.1
}

#[advent_of_code::main("25/03")]
fn main() {
    println!("{}", INPUT.lines()
        .map(|bank| {
            let mut bat_positions = [0; BATS];
            bat_positions[BATS-1] = max_byte_pos(&bank[..bank.len() - (BATS-1)]);

            for i in (0..(BATS-1)).rev() {
                let last_pos = bat_positions[i + 1];
                let bank_start = last_pos + 1;
                bat_positions[i] = max_byte_pos(&bank[bank_start .. bank.len()-i]) + bank_start;
            }
            
            bat_positions.into_iter()
                .enumerate()
                .map(|(i, bat_pos)|
                    JoltageSum::from(bank.as_bytes()[bat_pos] - b'0') * JoltageSum::from(10u8).pow(i.try_into().unwrap()))
                .sum::<JoltageSum>()
        })
        .sum::<JoltageSum>()
    );
}
