use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const DIAL_SIZE: i16 = 100;

pub fn run() -> i32 {
    let file = File::open("./src/input.txt").expect("File was unable to be opened");
    let reader = BufReader::new(file);
    let mut dial: i16 = 50;
    let mut count: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("Should be able to read line");
        let line = line.trim();

        let direction = line.chars().nth(0).unwrap();
        let num: i16 = (&line[1..]).parse().expect("Failed to parse string to i16");
        let sign: i16;

        if direction == 'L' {sign = -1;} else {sign = 1;}
        let num = sign * num;

        dial = (dial + num) % DIAL_SIZE;

        if dial < 0 {
            dial += DIAL_SIZE;
        } else if dial == 0 {
            count += 1;
        }
    }

    return count;
}
