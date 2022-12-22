#![allow(non_snake_case)]

fn main() {
    let input = include_str!("input.txt");
    let total = process(input);
    println!("Part 1 Score: {}", total.0);
    println!("Part 2 Score: {}", total.1);
}

// First result is for part one, and the second is for part two.
fn process(input: &str) -> (usize, usize) {
    let mut first_score = 0;
    let mut second_score = 0;

    let draw_points = 3;
    let win_points = 6;

    let map_choice = |c: char| match c {
        'A' => 0, // Rock
        'B' => 1, // Paper
        'C' => 2, // Scissors
        'X' => 0, // Rock / Lose
        'Y' => 1, // Paper / Draw
        'Z' => 2, // Scissors / Win
        _ => panic!(),
    };

    let choice_points = |c: usize| c + 1;

    for line in input.lines() {
        let mut split = line.split(' ');
        let opponent_char = split.next().unwrap().chars().next().unwrap();
        let my_char = split.next().unwrap().chars().next().unwrap();

        let opponent_choice = map_choice(opponent_char);
        let my_choice = map_choice(my_char);
        let my_points = choice_points(my_choice);

        first_score += my_points;

        if opponent_choice == my_choice {
            // Draw
            first_score += draw_points;
        } else {
            let win_condition = if my_choice == 0 { 2 } else { my_choice - 1 };
            let did_win = win_condition == opponent_choice;
            if did_win {
                first_score += win_points
            }
        }

        // Part 2

        let real_choice = match my_choice {
            // Lose
            0 => {
                if opponent_choice == 0 {
                    2
                } else {
                    opponent_choice - 1
                }
            }
            // Draw
            1 => {
                second_score += draw_points;
                opponent_choice
            }
            // Win
            2 => {
                second_score += win_points;
                if opponent_choice == 2 {
                    0
                } else {
                    opponent_choice + 1
                }
            }
            _ => panic!(),
        };

        let real_points = choice_points(real_choice);
        second_score += real_points;
    }

    (first_score, second_score)
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn works() {
        let expected = 8;
        let input = "A Y";
        let result = process(input).0;
        assert_eq!(result, expected);

        let expected: usize = 3 + 9 + 6;
        let input = concat!(
            "A Z\n", // +3
            "B Z\n", // +9
            "C Z",   // +6
        );

        let result = process(input).0;
        assert_eq!(result, expected);

        let expected: usize = 8;
        let input = "A Z";
        let result = process(input);
        assert_eq!(result.1, expected);

        let expected: usize = 8 + 2;
        let input = concat!(
            "A Z\n", // +8
            "C X",   // +2
        );

        let result = process(input).1;
        assert_eq!(result, expected);
    }
}
