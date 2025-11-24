// doesnt work yet, runs into an infinite loop on the first design?

use std::cmp::Ordering::{Equal, Greater, Less};

struct StackItem {
    /// pointing to char in the design string
    matched_design_chars: u8,
    matching_pattern_index: u8
}

#[advent_of_code::main("24/19")]
fn main() {
    let (patterns_input, designs_input) = INPUT.split_once("\n\n").unwrap();

    let patterns = patterns_input.split(", ").collect::<Vec<_>>();

    let mut possible_designs: u8 = 0;
    let mut stack = Vec::new();

    'outer: for (i, design) in designs_input.lines().enumerate() {
        println!("checking design {i}");

        stack.clear();
        stack.push(StackItem {
            matched_design_chars: 0,
            matching_pattern_index: 0
        });

        'inner: loop {
            let stack_item = stack.last_mut().unwrap();

            // TODO: calculate matching patterns once, sort them by length (longest first) and store in stack item
            let Some(matching_pattern) = patterns.iter()
                .filter(|p| design[stack_item.matched_design_chars.into()..].starts_with(*p))
                .nth(stack_item.matching_pattern_index.into())
            else {
                stack.pop();
                if stack.is_empty() {
                    // this design is impossible 
                    continue 'outer;
                }
                // try next pattern for step before that
                stack.last_mut().unwrap().matching_pattern_index += 1;
                continue 'inner;
            };

            #[allow(clippy::cast_possible_truncation)]
            let design_length = design.len() as u8;
            #[allow(clippy::cast_possible_truncation)]
            let matched_design_chars = stack_item.matched_design_chars + matching_pattern.len() as u8;

            match matched_design_chars.cmp(&design_length) {
                Equal => {
                    // design is possible!
                    possible_designs += 1;
                    continue 'outer;
                },
                Greater => {
                    // current chosen pattern is too long, try the next match
                    stack_item.matching_pattern_index += 1;
                },
                Less => {
                    // we have not reached the end yet, keep going
                    stack.push(StackItem {
                        matched_design_chars,
                        matching_pattern_index: 0
                    });
                },
            }
        }
    }

    println!("{possible_designs}");
}
