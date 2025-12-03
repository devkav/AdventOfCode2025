use std::fs;

pub fn run() -> u64 {
    let input = fs::read_to_string("./src/input.txt").expect("Could not read file");
    let input = input.trim();
    let tokens = input.split(",");
    let mut sum: u64 = 0;

    for token in tokens {
        let num_strs: Vec<&str> = token.split("-").collect();
        let nums = num_strs.iter().map(|&num| num.parse().expect("Number cannot be parsed")).collect::<Vec<u64>>();
        
        let start: u64 = nums[0];
        let end: u64 = nums[1];

        for i in start..end+1 {
            let current_num_str = i.to_string();
            let length = current_num_str.chars().count();

            if length % 2 == 1 {continue;}

            let first_half = &current_num_str[0..length / 2];
            let second_half = &current_num_str[length / 2..];

            if first_half == second_half {sum += i;}
        }
    }

    return sum;
}
