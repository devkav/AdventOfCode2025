const ADD: char = '+';
const MULT: char = '*';

pub fn run(input: &str) -> usize {
    let mut operators: Vec<char> = Vec::new();
    let mut column_widths: Vec<(usize, usize)> = Vec::new();

    for line in input.lines() {
        let first_char = line.chars().nth(0).unwrap();

        if first_char != ADD && first_char != MULT {
            continue;
        }

        let mut current_width = 0;
        let mut last_start = 0;
        let mut last_operator = line.chars().nth(0).unwrap();

        for i in 1..line.len() {
            let current = line.chars().nth(i).unwrap();

            if current == ADD || current == MULT {
                operators.push(last_operator);
                column_widths.push((last_start, current_width));
                last_start = i;
                last_operator = current;
                current_width = 0;
                continue;
            }

            current_width += 1
        }

        operators.push(last_operator);
        column_widths.push((last_start, current_width + 1));
    }

    let num_columns = operators.len();
    let mut problems: Vec<Vec<String>> = vec![Vec::new(); num_columns];

    for line in input.lines() {
        let first_char = line.chars().nth(0).unwrap();

        if first_char == ADD || first_char == MULT {
            continue;
        }

        for column_index in 0..column_widths.len() {
            let (column_start, column_width) = column_widths[column_index];

            if problems[column_index].len() == 0 {
                problems[column_index] = vec![String::new(); column_width];
            }

            for i in 0..column_width {
                let current = line.chars().nth(column_start + i).unwrap();
                problems[column_index][i].push(current);

            }
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
