pub fn solve(contents: String) {
    let battery_banks = parse_contents(contents);
    let mut total_output_p1: u32 = 0;
    let mut total_output_p2: u64 = 0;

    for bank in battery_banks {
        total_output_p1 += get_largest_joltage_p1(&bank);
        total_output_p2 += get_largest_joltage_p2(&bank);
    }

    println!("Day 3:\n-----------------");
    println!("[Part 1] Total Output: {total_output_p1}");
    println!("[Part 2] Total Output: {total_output_p2}");
}

fn get_largest_joltage_p2(bank: &String) -> u64 {
    let batteries: Vec<char> = bank.chars().collect();
    let mut twelve_digits = [('0', 0 as usize); 12];

    for i in 0..12 {
        let previous_highest_index = if i == 0 {
            0
        } else {
            twelve_digits[i - 1].1 + 1
        };

        for j in previous_highest_index..batteries.len() - (12 - i - 1) {
            if twelve_digits[i].0 < batteries[j] {
                twelve_digits[i].0 = batteries[j];
                twelve_digits[i].1 = j;
            }
        }
    }

    let mut largest_joltage: u64 = 0;

    for i in 0..12 {
        let digit = twelve_digits[i].0.to_digit(10).map(u64::from).unwrap();
        largest_joltage += digit * (10 as u64).pow(12 - 1 - (i as u32));
    }

    largest_joltage
}

fn get_largest_joltage_p1(bank: &String) -> u32 {
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
