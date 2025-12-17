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
            println!("Planning done");
            continue;
        }

        if planning {
            let tokens: Vec<&str> = line.split("-").collect();
            if tokens.len() != 2 {panic!("Expected 2 tokens, got: {}", tokens.len())}

            let start: usize = tokens[0].parse().expect("Unable to parse token");
            let end: usize = tokens[1].parse().expect("Unable to parse token");
            ranges.push((start, end));
        } else {
            let num: usize = line.parse().expect("Unable to parse line to number");

            for range in &ranges {
                let (start, end) = range;

                if num >= *start && num <= *end {
                    count += 1;
                    break;
                }
            }
        }
    }

   return count;
}
