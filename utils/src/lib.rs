use std::fs;

pub fn read_input(day: u32) -> String {
    let path = format!("{}/../inputs/day{day:02}.txt", env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("Could not read {path}: {e}"))
}
