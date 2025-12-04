pub fn solve(contents: String) {
    let id_ranges = parse_contents(contents);
    let mut sum_of_invalid_ids: u64 = 0;

    for id_range in id_ranges {
        let (first_id, last_id) = parse_id_range(id_range);

        for id in first_id..last_id + 1 {
            if is_invalid_id(id) {
                sum_of_invalid_ids += id;
            }
        }
    }

    println!("Day 2:\n-----------------");
    println!("[Part 1] Sum of invalid ids: {sum_of_invalid_ids}");
}

fn is_invalid_id(id: u64) -> bool {
    let id_string = id.to_string();
    let id_len = id_string.len();

    id_len % 2 == 0 && &id_string[..id_len / 2] == &id_string[id_len / 2..]
}

fn parse_id_range(id_range: String) -> (u64, u64) {
    let mut split_range = id_range.split("-");
    let first_id: u64 = split_range.next().unwrap().parse().unwrap(); // assume valid input
    let last_id: u64 = split_range.next().unwrap().parse().unwrap();

    (first_id, last_id)
}

fn parse_contents(contents: String) -> Vec<String> {
    contents
        .split(",")
        .map(|id_range| String::from(id_range))
        .collect()
}
