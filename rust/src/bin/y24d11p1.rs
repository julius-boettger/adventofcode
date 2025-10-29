type Stone = u64;

fn blink_at_stone(stone: Stone) -> Vec<Stone> {
    if stone == 0 {
        return vec![1];
    }

    let stone_string = stone.to_string();
    let stone_string_len = stone_string.len();
    if stone_string.len() % 2 == 0 {
        let stones = stone_string.split_at(stone_string_len / 2);
        return vec![
            stones.0.parse().unwrap(),
            stones.1.parse().unwrap()
        ];
    }

    vec![stone * 2024]
}

fn main() {
    let input = include_str!("../../input/24/11.txt").replace('\n', "");

    // this approach uses 3.5x less memory compared to collecting the iterator into a Vec every time

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
