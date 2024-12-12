type Stone = u64;
type StoneQuantity = u64;

#[derive(Debug)]
struct StoneWrap {
    quantity: StoneQuantity,
    stone: Stone,
}

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

fn blink_at_stones(stones: Vec<StoneWrap>) -> Vec<StoneWrap> {
    let mut moved_wraps: Vec<StoneWrap> = vec![];
    // for every given stone wrap
    for start_wrap in stones {
        // for every moved stone after blinking at it
        for moved_stone in blink_at_stone(&start_wrap.stone) {
            // check if it's already in the vec
            match moved_wraps.iter_mut().find(|wrap| wrap.stone == moved_stone) {
                // add it to the vec
                None => {
                    moved_wraps.push(StoneWrap {
                        quantity: start_wrap.quantity,
                        stone: moved_stone
                    });
                }
                // add it's quantity to the existing element in the vec
                Some(moved_wrap) => {
                    moved_wrap.quantity += start_wrap.quantity;
                }
            }
        }
    }
    moved_wraps
}

fn main() {
    let mut input = advent_of_code::input!();
    input = input.replace("\n", "");
    let mut stones: Vec<StoneWrap> = input
        .split(" ")
        .map(|s| StoneWrap { quantity: 1, stone: s.parse().unwrap() })
        .collect();

    for _ in 0..75 {
        stones = blink_at_stones(stones);
    }

    println!("{}", stones.into_iter().map(|s| s.quantity).sum::<StoneQuantity>());
}
