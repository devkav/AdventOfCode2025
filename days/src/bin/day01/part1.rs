const DIAL_SIZE: i16 = 100;

pub fn run(input: &str) -> i32 {
    let mut dial: i16 = 50;
    let mut count: i32 = 0;

    for line in input.lines() {
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
