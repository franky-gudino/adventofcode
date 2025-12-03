enum Direction {
    Left,
    Right,
}

const DIAL_LENGTH: i32 = 100;

pub fn solve(contents: String) {
    let instructions = parse_contents(contents);

    let password1 = find_password_part_1(&instructions);
    let password2 = find_password_part_2(&instructions);

    println!("Day 1:\n-----------------");
    println!("[Part 1] Password: {password1}");
    println!("[Part 2] Password: {password2}");
}

fn find_password_part_1(instructions: &Vec<String>) -> u32 {
    let mut current: i32 = 50;
    let mut password: u32 = 0;

    for instruction in instructions {
        let (direction, distance) = parse_instructions(instruction);

        current = (current
            + match direction {
                Direction::Left => -distance,
                Direction::Right => distance,
            })
        .rem_euclid(DIAL_LENGTH);

        if current == 0 {
            password += 1;
        }
    }

    password
}

fn find_password_part_2(instructions: &Vec<String>) -> u32 {
    let mut current: i32 = 50;
    let mut password: u32 = 0;

    for instruction in instructions {
        let (direction, distance) = parse_instructions(instruction);

        let result = (current
            + match direction {
                Direction::Left => -distance,
                Direction::Right => distance,
            })
        .rem_euclid(DIAL_LENGTH);

        let rotations = match direction {
            Direction::Left => {
                let end = current - distance;
                let start_floor = (current - 1).div_euclid(DIAL_LENGTH);
                let end_floor = (end - 1).div_euclid(DIAL_LENGTH);

                (start_floor - end_floor) as u32
            }
            Direction::Right => {
                let end = current + distance;

                (end / DIAL_LENGTH) as u32
            }
        };

        password += rotations;
        current = result;
    }

    password
}

fn parse_contents(contents: String) -> Vec<String> {
    contents.lines().map(|line| String::from(line)).collect()
}

fn parse_instructions(instructions: &str) -> (Direction, i32) {
    let direction = if &instructions[0..1] == "L" {
        Direction::Left
    } else {
        Direction::Right
    };

    let distance: i32 = instructions[1..].parse().unwrap();

    (direction, distance)
}
