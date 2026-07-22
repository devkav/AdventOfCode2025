const ADD: char = '+';
const MULT: char = '*';

pub fn run(input: &str) -> usize {
    let mut operators: Vec<char> = Vec::new();
    let mut column_width = 0;

    for line in input.lines() {
        let first_char = line.chars().nth(0).unwrap();

        if first_char != ADD && first_char != MULT {
            continue;
        }

        let mut next_column_found = false;
        let mut i = 1;

        while !next_column_found {
            let current = line.chars().nth(i).unwrap();

            if current == ADD || current == MULT {
                next_column_found = true;
                column_width = i;
                continue;
            }

            i += 1;
        }

        operators = line.trim()
            .split(" ")
            .filter(|operator| operator.len() > 0)
            .map(|char_str| char_str.chars().next().unwrap())
            .collect();
    }

    let num_columns = operators.len();
    let mut problems: Vec<Vec<String>> = vec![vec![String::new(); column_width]; num_columns];

    for line in input.lines() {
        let first_char = line.chars().nth(0).unwrap();

        if first_char == ADD || first_char == MULT {
            continue;
        }

        for (i, num_char) in line.chars().enumerate() {
            if i == column_width -1 || num_char == ' ' {
                continue;
            }

            let column_num = i / column_width;
            let num_pos = i % column_width;

            problems[column_num][num_pos].push(num_char);
        }
    }

    let mut results: Vec<Option<usize>> = vec![None; num_columns];

    for (problem_index, problem) in problems.iter().enumerate() {
        for num_str in problem {
            if num_str.trim().len() == 0 {
                continue
            }

            let operator = operators[problem_index];
            let num: usize = num_str.trim().parse().unwrap();

            if operator == ADD {
                results[problem_index] = match results[problem_index] {
                    None => Some(num),
                    Some(value) => Some(value + num)
                };
            } else if operator == MULT {
                results[problem_index] = match results[problem_index] {
                    None => Some(num),
                    Some(value) => Some(value * num)
                };
            } else {
                panic!("Unknown operator: {operator}");
            }
        }
    }

    return results.iter().fold(0, |acc, result| acc + if result.is_some() {result.unwrap()} else {0});
}
