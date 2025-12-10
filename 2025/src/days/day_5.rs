pub fn solve(contents: String) {
    let (id_ranges, ids) = parse_contents(contents);
    let parsed_id_ranges: Vec<(u64, u64)> = id_ranges.iter().map(parse_id_range).collect();
    let parsed_ids: Vec<u64> = ids.iter().map(|id| id.parse().unwrap()).collect();

    let fresh_ingredients: u64 = get_fresh_ingredients(&parsed_id_ranges, &parsed_ids);

    println!("Day 5:\n-----------------");
    println!("[Part 1] Fresh ingredients: {fresh_ingredients}");
}

fn get_fresh_ingredients(id_ranges: &Vec<(u64, u64)>, ids: &Vec<u64>) -> u64 {
    let mut fresh_ingredients = 0;

    for id in ids {
        for (l, r) in id_ranges {
            if (l..&(*r + 1)).contains(&id) {
                fresh_ingredients += 1;
                break;
            }
        }
    }

    fresh_ingredients
}

fn parse_id_range(id_range: &String) -> (u64, u64) {
    let mut split_range = id_range.split("-");
    let first_id: u64 = split_range.next().unwrap().parse().unwrap(); // assume valid input
    let last_id: u64 = split_range.next().unwrap().parse().unwrap();

    (first_id, last_id)
}

fn parse_contents(contents: String) -> (Vec<String>, Vec<String>) {
    let lines: Vec<String> = contents.lines().map(|line| String::from(line)).collect();

    let new_line_index = lines.iter().position(|x| x == "").unwrap();

    (
        lines[..new_line_index].to_vec(),
        lines[new_line_index + 1..].to_vec(),
    )
}
