type Stone = u64;

fn blink_at_stone(stone: Stone) -> Vec<Stone> {
    if stone == 0 {
        return vec![1];
    }

    let stone_digits = stone.ilog10() + 1;
    if stone_digits % 2 == 0 {
        // "1234" => ["12", "34"]
        // power of 10 that devides the two halves of digits
        let split_indicator: Stone = Stone::from(10u8).pow(stone_digits / 2);
        let stone_1 = stone / split_indicator;
        let stone_2 = stone - (stone_1 * split_indicator);
        return vec![stone_1, stone_2];
    }

    vec![stone * 2024]
}

#[advent_of_code::main("24/11")]
fn main() {
    // this approach uses 3.5x less memory compared to collecting the iterator into a Vec every time

    let input = INPUT.replace('\n', "");
    let mut stones: Box<dyn Iterator<Item = Stone>> = Box::new(
        input
            .split(' ')
            .map(|s| s.parse().unwrap())
    );

    for _ in 0..25 {
        stones = Box::new(stones.flat_map(blink_at_stone));
    }

    println!("{} stones", stones.count());
}
