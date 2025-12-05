pub fn solve(contents: String) {
    let battery_banks = parse_contents(contents);
    let mut total_output: u32 = 0;

    for bank in battery_banks {
        total_output += get_largest_joltage(bank);
    }

    println!("Day 3:\n-----------------");
    println!("[Part 1] Total Output: {total_output}");
}

fn get_largest_joltage(bank: String) -> u32 {
    let batteries: Vec<char> = bank.chars().collect();
    let mut first_digit = (batteries[0], 0);

    // get largest digit and its index (max if not last one)
    for i in 1..batteries.len() - 1 {
        if first_digit.0 < batteries[i] {
            first_digit.0 = batteries[i];
            first_digit.1 = i;
        }
    }

    let mut second_digit = batteries[first_digit.1 + 1];

    // starting from the index after first_digit, find the next largest digit
    for i in first_digit.1 + 1..batteries.len() {
        if second_digit < batteries[i] {
            second_digit = batteries[i];
        }
    }

    let parsed_joltage: u32 =
        first_digit.0.to_digit(10).unwrap() * 10 + second_digit.to_digit(10).unwrap();

    parsed_joltage
}

fn parse_contents(contents: String) -> Vec<String> {
    contents.lines().map(|line| String::from(line)).collect()
}
