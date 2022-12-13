#![allow(non_snake_case)]

const WIDTH: usize = 40;

fn main() {
    let input = include_str!("input.txt");
    let (signal_strength_sum, crt_output) = process_input(input, vec![20, 60, 100, 140, 180, 220]);
    println!("Signal Strength Sum: {signal_strength_sum}");
    println!();
    println!("Output:");
    println!("{}", crt_output);
}

fn process_input(input: &str, signal_strength_checkpoints: Vec<usize>) -> (isize, String) {
    let mut cycle: usize = 1;
    let mut x_register: isize = 1;
    let mut signal_strength_sum = 0;
    let mut crt_output = String::new();

    let mut update = |cycle: usize, x_register: isize| {
        if signal_strength_checkpoints.contains(&cycle) {
            signal_strength_sum += cycle as isize * x_register;
        }

        let current_line_index = ((cycle - 1) % WIDTH) as isize;
        let energized_indexes = (x_register - 1)..=(x_register + 1);

        if energized_indexes.contains(&current_line_index) {
            crt_output.push('#');
        } else {
            crt_output.push('.');
        }

        if current_line_index == (WIDTH - 1) as isize {
            crt_output.push('\n');
        }
    };

    for line in input.lines().filter(|l| !l.is_empty()) {
        update(cycle, x_register);

        let trimmed = line.trim();
        if trimmed == "noop" {
            cycle += 1;
            continue;
        }

        cycle += 1;
        update(cycle, x_register);
        cycle += 1;

        let value_string = trimmed.strip_prefix("addx ").expect("Unexpected input.");
        let parsed: isize = value_string.parse().expect("Unable to parse 'addx' value.");
        x_register += parsed;
    }

    (signal_strength_sum, crt_output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let example = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";  
        let result = process_input(example, vec![20, 60, 100, 140, 180, 220]).0;
        let expected = 13140;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let example = "noop\naddx 2\naddx -3\nnoop\naddx 1";
        let result = process_input(example, vec![4, 6]).0;
        let expected = 12;
        assert_eq!(result, expected);
    }
}
