// doesnt work yet, runs into an infinite loop on the first design?

use std::cmp::Ordering::{Equal, Greater, Less};

macro_rules! top {
    ($stack: expr) => {
        $stack.last().unwrap()
    };
}
macro_rules! top_mut {
    ($stack: expr) => {
        $stack.last_mut().unwrap()
    };
}
macro_rules! top_patterns {
    ($stack: expr) => {
        top!($stack).matching_patterns.as_ref().unwrap()
    };
}
macro_rules! top_patterns_mut {
    ($stack: expr) => {
        top_mut!($stack).matching_patterns.as_mut().unwrap()
    };
}

struct StackItem {
    /// pointing to char in the design string
    matched_design_chars: u8,
    /// `None` means "not checked yet", last element of `Some` is used
    matching_patterns: Option<Vec<&'static str>>
}
impl StackItem {
    const fn new(matched_design_chars: usize) -> Self {
        Self {
            #[allow(clippy::cast_possible_truncation)]
            matched_design_chars: matched_design_chars as u8,
            matching_patterns: None
        }
    }
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
        stack.push(StackItem::new(0));

        'inner: loop {
            if top!(stack).matching_patterns.is_none() {
                let mut matching_patterns = patterns.iter()
                    .filter(|p| design[top!(stack).matched_design_chars.into()..].starts_with(*p))
                    .copied()
                    .collect::<Vec<_>>();
                // sort by ascending pattern length
                matching_patterns.sort_unstable_by_key(|a| a.len());
                top_mut!(stack).matching_patterns = Some(matching_patterns);
            }

            if top_patterns!(stack).is_empty() {
                stack.pop();
                if stack.is_empty() {
                    // this design is impossible 
                    continue 'outer;
                }
                // try next pattern for step before that
                top_patterns_mut!(stack).pop();
                continue 'inner;
            }

            #[allow(clippy::cast_possible_truncation)]
            let matched_design_chars = top!(stack).matched_design_chars as usize + top!(top_patterns!(stack)).len();

            match matched_design_chars.cmp(&design.len()) {
                // we have not reached the end yet, keep going
                Less => stack.push(StackItem::new(matched_design_chars)),
                Equal => {
                    // design is possible!
                    possible_designs += 1;
                    continue 'outer;
                },
                // current chosen pattern is too long, try next fitting match
                Greater => {
                    // top is too long => pop
                    top_patterns_mut!(stack).pop();
                    // remove all other patterns that are too long
                    let remaining_chars = design.len() - top!(stack).matched_design_chars as usize;
                    *top_patterns_mut!(stack) = top_patterns!(stack).iter()
                        .filter(|p| p.len() < remaining_chars)
                        .copied()
                        .collect();
                },
            }
        }
    }

    println!("{possible_designs}");
}
