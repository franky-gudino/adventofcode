pub fn solve(contents: String) {
    let id_ranges = parse_contents(contents);
    let mut p1_sum_of_invalid_ids: u64 = 0;
    let mut p2_sum_of_invalid_ids: u64 = 0;

    for id_range in id_ranges {
        let (first_id, last_id) = parse_id_range(id_range);

        for id in first_id..last_id + 1 {
            if is_invalid_id_p1(id) {
                p1_sum_of_invalid_ids += id;
            }

            if is_invalid_id_p2(id) {
                p2_sum_of_invalid_ids += id;
            }
        }
    }

    println!("Day 2:\n-----------------");
    println!("[Part 1] Sum of invalid ids: {p1_sum_of_invalid_ids}");
    println!("[Part 2] Sum of invalid ids: {p2_sum_of_invalid_ids}");
}

fn is_invalid_id_p2(id: u64) -> bool {
    let id_string = id.to_string();
    let id_doubled = id_string.clone() + &id_string;
    let id_doubled_trimmed = &id_doubled[1..&id_doubled.len() - 1];

    id_doubled_trimmed.contains(&id_string)
}

fn is_invalid_id_p1(id: u64) -> bool {
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
