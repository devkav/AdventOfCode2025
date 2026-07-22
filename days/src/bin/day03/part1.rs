fn find_largest_number_after(string: &str, start: u16, end: u16) -> u16 {
    let mut max_num: u8 = 0;
    let mut max_num_index: u16 = 0;

    for index in start..=end {
        let char: char = match string.chars().nth(index as usize) {
            Some(i) => i as char,
            None => '0',
        };
        let num: u8 = match char.to_digit(10) {
            Some(i) => i as u8,
            None => 0,
        };

        if num > max_num {
            max_num = num;
            max_num_index = index;
        }
    }

    return max_num_index;
}

pub fn run(input: &str, joltage_char_length: u16) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let line: String = line.trim().to_string();
        let u_len: u16 = line.len() as u16;
        let mut chars: Vec<char> = Vec::new();
        let mut start: u16 = 0;

        for i in 1..=joltage_char_length {
            let num_chars_remaining: u16 = joltage_char_length - i;
            if i > 1 {start += 1}
            let end: u16 = (u_len - 1) - num_chars_remaining;

            let char_index: u16 = find_largest_number_after(&line, start, end);
            let current_char = match line.chars().nth(char_index as usize) {
                Some(i) => i,
                None => panic!("Index in line out of range, index: {}", char_index)
            };

            chars.push(current_char);
            start = char_index;
        }

        let num_str: String = chars.iter().collect();
        let num: usize = num_str.parse::<usize>().expect("Failed to parse string");
        sum += num;
    }

    return sum;
}
