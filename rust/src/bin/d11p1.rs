type Stone = u64;

fn blink_at_stone(stone: &Stone) -> Vec<Stone> {
    if *stone == 0 {
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

fn blink_at_stones(stones: Vec<Stone>) -> Vec<Stone> {
    stones.iter().map(|stone| blink_at_stone(&stone)).flatten().collect()
}

fn main() {
    let mut input = advent_of_code::input!();
    input = input.replace("\n", "");
    let mut stones: Vec<Stone> = input
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..25 {
        println!("blink {} at {} stones", i + 1, stones.len());
        stones = blink_at_stones(stones);
    }
    println!("=> {} stones", stones.len());
}
