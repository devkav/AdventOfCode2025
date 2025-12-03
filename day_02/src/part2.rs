use std::fs;

fn is_repeated(num_str: String, divisions: usize) -> bool {
    let length = num_str.chars().count();

    if divisions > length {return false;}

    if length % divisions != 0 {
        return is_repeated(num_str, divisions + 1);
    }

    let steps = length / divisions;
    let first_segment = &num_str[0..steps];
    let mut all_match = true;

    for i in 1..divisions {
        let current_segment = &num_str[steps * i..steps * (i + 1)];

        if current_segment != first_segment {
            all_match = false;
            break;
        }
    }

    if !all_match {
        return is_repeated(num_str, divisions + 1)
    } else {
        return all_match;
    }
}

pub fn run() {
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
            if is_repeated(current_num_str, 2) {sum += i}
        }
    }

    println!("{}", sum);
}
