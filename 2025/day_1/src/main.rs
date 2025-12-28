fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut zeroes = 0;
    let mut current_position = 50;

    let input = include_str!("./part_1_input.txt");
    let turns = input.split("\n").collect::<Vec<&str>>();

    for turn in turns {
        if turn.len() == 0 {
            continue;
        }

        let direction = match turn.chars().nth(0).unwrap() {
            'L' => -1,
            'R' => 1,
            _ => 0,
        };

        let clicks = turn
            .replace("R", "")
            .replace("L", "")
            .parse::<i32>()
            .unwrap();

        current_position = get_current_position(current_position + direction * clicks);

        if current_position == 0 {
            zeroes += 1;
        }
    }

    println!("Part 1: {}", zeroes);
}

fn part_2() {
    let mut zeroes = 0;
    let mut current_position = 50;

    let input = include_str!("./part_1_input.txt");
    let turns = input.split("\n").collect::<Vec<&str>>();

    for turn in turns {
        if turn.len() == 0 {
            continue;
        }

        let direction = match turn.chars().nth(0).unwrap() {
            'L' => -1,
            'R' => 1,
            _ => 0,
        };

        let clicks = turn
            .replace("R", "")
            .replace("L", "")
            .parse::<i32>()
            .unwrap();

        let current_position_with_rollover = get_position_with_rollover_v2(
            current_position,
            direction * clicks
        );

        current_position = current_position_with_rollover.0;
        zeroes += current_position_with_rollover.1;
    }

    println!("Part 2: {}", zeroes);
}

fn get_current_position(future_position: i32) -> i32 {
    let mut overflow_position = future_position;

    while overflow_position > 99 {
        overflow_position -= 100;
    }

    while overflow_position < 0 {
        overflow_position += 100;
    }

    overflow_position
}

fn get_position_with_rollover_v2(original_position: i32, clicks: i32) -> (i32, i32) {
    let mut rollover_count = 0;

    let actual_moves = clicks % 100;
    let full_turns = ((clicks - actual_moves) / 100).abs();
    let mut new_position = original_position + actual_moves;

    rollover_count += full_turns;

    if new_position < 0 && original_position != 0 {
        rollover_count += 1;

        new_position = new_position + 100;
    } else if new_position > 99 {
        rollover_count += 1;
        new_position = new_position - 100;
    } else if new_position == 0 {
        rollover_count += 1;
    }

    if new_position < 0 && original_position == 0 {
        new_position += 100;
    }

    (new_position, rollover_count)
}
