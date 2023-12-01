pub fn get_input(day: usize, test: bool) -> String {
    std::fs::read_to_string(format!(
        "inputs/{}{}.txt",
        day,
        if test { "-test" } else { "" }
    ))
    .expect("Unable to read file")
}
