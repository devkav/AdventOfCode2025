use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn run() -> usize {
    let file = File::open("./src/input.txt").expect("File was unable to be opened");
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut planning = true;

    for line in reader.lines() {
        let line = line.expect("Should be able to read line");
        let line = line.trim();

        if line == "" {
            planning = false;
            continue;
        }

        if !planning {continue}

        if planning {
            let tokens: Vec<&str> = line.split("-").collect();
            if tokens.len() != 2 {panic!("Expected 2 tokens, got: {}", tokens.len())}

            let start: usize = tokens[0].parse().expect("Unable to parse token");
            let end: usize = tokens[1].parse().expect("Unable to parse token");
            let mut skip = false;

            for range in &ranges {
                let (current_start, current_end) = range;

                if start >= *current_start && end <= *current_end {
                    // No ranges left over
                    skip = true;
                    break;
                } else if start < *current_start && end > *current_start {
                    // Creates two ranges
                } else if start < *current_start && end <= *current_end {
                    // First half of range left over
                } else if start >= *current_start && end > *current_end {
                    // Second half of range is left over
                }
            }

            if skip {continue}
            //ranges.push((start, end));

        }
    }

   return count;
}
