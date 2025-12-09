pub fn solve(contents: String) {
    let grid_of_rolls = parse_contents(contents);
    let accessible_rolls_p1 = get_accessible_rolls_p1(grid_of_rolls);

    println!("Day 4:\n-----------------");
    println!("[Part 1] Accessible Rolls: {accessible_rolls_p1}");
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

fn get_accessible_rolls_p1(grid_of_rolls: Vec<Vec<char>>) -> u32 {
    let rows = grid_of_rolls.len() as isize;
    let cols = grid_of_rolls[0].len() as isize;
    let mut total_accessible_rows: u32 = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid_of_rolls[r as usize][c as usize] != '@' {
                continue;
            }

            let mut adj_rolls = 0;

            for (x, y) in DIRECTIONS {
                let is_in_grid = (0..rows).contains(&(r + y)) && (0..cols).contains(&(c + x));

                if is_in_grid && grid_of_rolls[(r + y) as usize][(c + x) as usize] == '@' {
                    adj_rolls += 1;
                }
            }

            if adj_rolls < 4 {
                total_accessible_rows += 1;
            }
        }
    }

    total_accessible_rows
}

fn parse_contents(contents: String) -> Vec<Vec<char>> {
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
