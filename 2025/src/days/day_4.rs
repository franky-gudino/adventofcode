pub fn solve(contents: String) {
    let grid_of_rolls = parse_contents(contents);
    let accessible_rolls_p1 = get_accessible_rolls_p1(grid_of_rolls);

    println!("Day 4:\n-----------------");
    println!("[Part 1] Accessible Rolls: {accessible_rolls_p1}");
}

const DIRECTIONS: [(i32, i32); 8] = [
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
    let rows = grid_of_rolls.len();
    let cols = grid_of_rolls[0].len();
    let mut total_accessible_rows: u32 = 0;

    for r in 0..grid_of_rolls.len() {
        for c in 0..grid_of_rolls[0].len() {
            if grid_of_rolls[r][c] != '@' {
                continue;
            }

            let mut adj_rolls = 0;

            for (x, y) in DIRECTIONS {
                let is_in_grid = (0..rows as i32).contains(&(r as i32 + y))
                    && (0..cols as i32).contains(&(c as i32 + x));

                if is_in_grid
                    && grid_of_rolls[(r as i32 + y) as usize][(c as i32 + x) as usize] == '@'
                {
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
