use std::hash::Hash;
use std::collections::HashMap;

/// general-purpose cache for memoization
pub struct Cache<Input, Output> {
    map: HashMap<Input, Output>,
    #[cfg(debug_assertions)]
    accesses: u32,
    #[cfg(debug_assertions)]
    hits: u32,
}

impl<Input: Eq + Clone + Hash, Output: Clone> Cache<Input, Output> {
    #[must_use]
    pub fn new() -> Self {
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
    pub fn get<F: Fn(Input, &mut Self) -> Output>(&mut self, input: Input, function: F) -> Output {
        #[cfg(debug_assertions)] {
            self.accesses += 1;
        }
        if let Some(result) = self.map.get(&input) {
            #[cfg(debug_assertions)] { self.hits += 1; }
            result.clone()
        } else {
            let result = function(input.clone(), self);
            self.map.insert(input, result.clone());
            result
        }
    }
}

/// print stats like cache hit rate on drop, but only when running a debug build
#[cfg(debug_assertions)]
impl<I, O> Drop for Cache<I, O> {
    #[allow(clippy::cast_precision_loss)]
    fn drop(&mut self) {
        println!("cache stats:");
        println!("- contains {} elements", self.map.len());
        println!("- {} accesses", self.accesses);
        println!("- {} hits", self.hits);
        println!("- {:.1}% hit rate ", ((self.hits as f32) / (self.accesses as f32)) * 100.0);
    }
}

// because clippy wanted it
impl<Input: Eq + Clone + Hash, Output: Clone> Default for Cache<Input, Output> {
    fn default() -> Self {
        Self::new()
    }
}
