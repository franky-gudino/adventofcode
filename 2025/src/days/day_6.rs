pub fn solve(contents: String) {
    let grid = parse_contents(contents);
    let grand_total_1 = get_grand_total_p1(&grid);

    println!("Day 6:\n-----------------");
    println!("[Part 1] Grand total: {grand_total_1}");
}

fn get_grand_total_p1(grid: &Vec<Vec<String>>) -> u64 {
    let mut grand_total = 0;

    for c in 0..grid[0].len() {
        let mut nums: Vec<u64> = Vec::new();

        for r in 0..grid.len() - 1 {
            nums.push(grid[r][c].parse().unwrap());
        }

        let oper = &grid[grid.len() - 1][c];

        grand_total += nums
            .iter()
            .fold(if oper == "+" { 0 } else { 1 }, |acc, num| {
                if oper == "+" { acc + num } else { acc * num }
            });
    }

    grand_total
}

fn parse_contents(contents: String) -> Vec<Vec<String>> {
    contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|col| String::from(col))
                .collect()
        })
        .collect()
}
