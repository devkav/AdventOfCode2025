use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const MIN_NUM_ROLLS: usize = 4;


pub fn run() -> usize {
    let file = File::open("./src/input.txt").expect("File was unable to be opened");
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut count = 0;

    for line in reader.lines() {
        let line = line.expect("Should be able to read line");
        let line = line.trim();
        let current_row: Vec<bool> = line.chars().map(|x| x == '@').collect();

        grid.push(current_row);
    }

    let num_rows = grid.len();

    for y in 0..num_rows {
        let num_cols = grid[y].len();

        for x in 0..grid[y].len() {
            if !grid[y][x] {continue}
            let mut surrounding_rolls = 0;

            for y_add in -1..=1 as isize {
                if surrounding_rolls >= MIN_NUM_ROLLS {break}

                for x_add in -1..=1 as isize {
                    if surrounding_rolls >= MIN_NUM_ROLLS {break}
                    if y_add == 0 && x_add == 0 {continue}
                    
                    let current_x: isize = (x as isize) + x_add;
                    let current_y: isize = (y as isize) + y_add;

                    if current_y < 0 || current_y > (num_rows as isize) - 1 {continue}
                    if current_x < 0 || current_x > (num_cols as isize) - 1 {continue}

                    let current_x = current_x as usize;
                    let current_y = current_y as usize;

                    if grid[current_y][current_x] {surrounding_rolls += 1;}
                }
            }

            if surrounding_rolls < MIN_NUM_ROLLS {count += 1;}
        }
    }

    return count;
}
