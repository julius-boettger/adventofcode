/// return content of input file for this puzzle
#[macro_export]
macro_rules! input {
    () => {
        advent_of_code::input(file!())
    };
}

/// return content of input file determined by source file path
pub fn input(source_file_path: &'static str) -> String {
    // determine day to select input file from source file path
    let pattern = regex::Regex::new(r"src/bin/d(\d{2})p\d\.rs").unwrap();
    let Some(capture_groups) = pattern.captures(source_file_path) else {
        panic!("called from {}, from which the day (to select the input file) cannot be determined", source_file_path)
    };
    let day = &capture_groups[1].to_string();

    // return input file content
    let input_file_path = format!("input/{}.txt", day);
    std::fs::read_to_string(&input_file_path)
        .expect(format!("{} could not be read", input_file_path).as_str())
}
