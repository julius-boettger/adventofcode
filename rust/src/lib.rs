/// return content of input file for this puzzle
#[macro_export]
macro_rules! input {
    () => {
        advent_of_code::input(file!())
    };
}

/// return content of input file determined by source file path
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn input(source_file_path: &'static str) -> String {
    // determine day to select input file from source file path
    let pattern = regex::Regex::new(r"src/bin/y(\d{2})d(\d{2})p\d\.rs").unwrap();
    let Some(capture_groups) = pattern.captures(source_file_path) else {
        panic!("called from {source_file_path}, from which the year and/or day (to select the input file) cannot be determined")
    };
    let year = &capture_groups[1].to_string();
    let day = &capture_groups[2].to_string();

    // return input file content
    let input_file_path = format!("input/{year}/{day}.txt");
    std::fs::read_to_string(&input_file_path)
        .unwrap_or_else(|_| panic!("{input_file_path} could not be read"))
}
