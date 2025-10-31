use std::collections::HashMap;

type Stone = u64;
type StoneQuantity = u64;

type StoneMap = HashMap<Stone, StoneQuantity>;
type MovementCache = Cache<Stone, Vec<Stone>>;

/// general-purpose cache for memoization
struct Cache<Input, Output> {
    map: HashMap<Input, Output>,
    #[cfg(debug_assertions)]
    accesses: u32,
    #[cfg(debug_assertions)]
    hits: u32,
}
impl<Input: Eq + Clone + std::hash::Hash, Output: Clone> Cache<Input, Output> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            #[cfg(debug_assertions)]
            accesses: 0,
            #[cfg(debug_assertions)]
            hits: 0
        }
    }
    /// if possible, returns the result from the cache,
    /// otherwise computes it and saves it to the cache
    fn get<F: Fn(Input) -> Output>(&mut self, input: Input, function: F) -> Output {
        #[cfg(debug_assertions)] {
            self.accesses += 1;
        }
        if let Some(result) = self.map.get(&input) {
            #[cfg(debug_assertions)] { self.hits += 1; }
            result.clone()
        } else {
            let result = function(input.clone());
            self.map.insert(input, result.clone());
            result
        }
    }
    /// print stats like cache hit rate, but only when running a debug build
    #[allow(clippy::cast_precision_loss)]
    fn debug_print_stats(&self) {
        #[cfg(debug_assertions)] {
            println!("cache stats:");
            println!("- contains {} elements", self.map.len());
            println!("- {} accesses", self.accesses);
            println!("- {} hits", self.hits);
            println!("- {:.1}% hit rate ", ((self.hits as f32) / (self.accesses as f32)) * 100.0);
        }
    }
}

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

fn blink_at_stones(stone_map: StoneMap, movemement_cache: &mut MovementCache) -> StoneMap {
    let mut new_stone_map: StoneMap = HashMap::new();
    // for every given stone and its quantity
    for stone_pair in stone_map {
        // for every new stone after blinking at it
        for new_stone in movemement_cache.get(stone_pair.0, blink_at_stone) {
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

    let mut stone_movement_cache = MovementCache::new();
    for _ in 0..75 {
        stone_map = blink_at_stones(stone_map, &mut stone_movement_cache);
    }

    println!("{}", stone_map.values().sum::<StoneQuantity>());
    stone_movement_cache.debug_print_stats();
}
