use std::collections::HashMap;

type Stone = u64;
type StoneQuantity = u64;

type StoneMap = HashMap<Stone, StoneQuantity>;
type MovementCache = HashMap<Stone, Vec<Stone>>;

const fn decimal_digits(mut stone: Stone) -> u8 {
    if stone == 0 {
        return 1;
    }

    let mut count = 0;
    while stone > 0 {
        stone /= 10;
        count += 1;
    }
    count
}

fn blink_at_stone(stone: Stone) -> Vec<Stone> {
    if stone == 0 {
        return vec![1];
    }

    let stone_digits = decimal_digits(stone);
    if stone_digits % 2 == 0 {
        // "1234" => ["12", "34"]
        // power of 10 that devides the two halves of digits
        let split_indicator: Stone = Stone::from(10u8).pow((stone_digits / 2).into());
        let stone_1 = stone / split_indicator;
        let stone_2 = stone - (stone_1 * split_indicator);
        return vec![stone_1, stone_2];
    }

    vec![stone * 2024]
}

fn blink_at_stone_cached(stone: Stone, movemement_cache: &mut MovementCache) -> Vec<Stone> {
    // check if stone to blink at is already in cache
    #[allow(clippy::option_if_let_else)]
    if let Some(result) = movemement_cache.get(&stone) {
        result.clone()
    } else {
        let result = blink_at_stone(stone);
        movemement_cache.insert(stone, result.clone());
        result
    }
}

fn blink_at_stones(stone_map: StoneMap, movemement_cache: &mut MovementCache) -> StoneMap {
    let mut new_stone_map: StoneMap = HashMap::new();
    // for every given stone and its quantity
    for stone_pair in stone_map {
        // for every new stone after blinking at it
        for new_stone in blink_at_stone_cached(stone_pair.0, movemement_cache) {
            // check if it's already in the map
            match new_stone_map.get_mut(&new_stone) {
                // add it's quantity to the existing pair in the map
                Some(quantity) => { *quantity += stone_pair.1; }
                // add it to the map
                #[allow(non_snake_case)]
                None => { new_stone_map.insert(new_stone, stone_pair.1); }
            }
        }
    }
    new_stone_map
}

#[advent_of_code::main("24/11")]
fn main() {
    // key: number engraved on stone
    // value: quantity
    let mut stone_map: StoneMap = (INPUT as &str)
        .replace('\n', "")
        .split(' ')
        .map(|s| (s.parse().unwrap(), 1))
        .collect();

    let mut stone_movement_cache: MovementCache = HashMap::new();
    for _ in 0..75 {
        stone_map = blink_at_stones(stone_map, &mut stone_movement_cache);
    }

    println!("{}", stone_map.values().sum::<StoneQuantity>());
}
