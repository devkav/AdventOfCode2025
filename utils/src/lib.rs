use std::fs;

pub mod coordinate_2d;
pub mod coordinate_3d;
pub mod coordinate_generic;

pub mod network;

pub fn read_input(day: u32, test: bool) -> String {
    let suffix = if test { "_test" } else { "" };
    let path = format!(
        "{}/../inputs/day{day:02}{}.txt",
        env!("CARGO_MANIFEST_DIR"),
        suffix
    );
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("Could not read {path}: {e}"))
}
