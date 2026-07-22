const ADD: &str = "+";
const MULT: &str = "*";

pub fn run(input: &str) -> usize {
    let mut operators: Vec<&str> = Vec::new();

    for line in input.lines() {
        let first_char = line.chars().nth(0).unwrap().to_string();

        if first_char != ADD && first_char != MULT {
            continue;
        }

        operators = line.trim()
            .split(" ")
            .filter(|operator| operator.len() > 0)
            .collect();
    }

    let mut results: Vec<usize> = operators.iter().map(|operator| if operator == &ADD {0} else {1}).collect();

    for line in input.lines() {
        let first_char = line.chars().nth(0).unwrap().to_string();

        if first_char == ADD || first_char == MULT {
            continue;
        }

        let nums: Vec<usize> = line.trim()
            .split(" ")
            .filter(|num| num.len() > 0)
            .map(|num| num.parse().unwrap())
            .collect();

        for (i, num) in nums.iter().enumerate() {
            let operator = operators[i];

            if operator == ADD {
                results[i] += num;
            } else if operator == MULT {
                results[i] *= num;
            } else {
                panic!("Unknown operator: {operator}");
            }
        }
    }

    return results.iter().sum();
}
