#![allow(non_snake_case)]

use core::cmp::Ordering;

fn main() {
    let input = include_str!("input.txt");
    let part_one = visited_tail_positions(input, 2);
    let part_two = visited_tail_positions(input, 10);
    println!("Unique Positions Visited by Tail:");
    println!("  Part One (2 knots): {part_one}");
    println!("  Part Two (10 knots): {part_two}");
}

fn visited_tail_positions(input: &str, knots: usize) -> usize {
    if knots == 0 {
        return 0;
    }

    let mut knot_positions = vec![];

    for _ in 0..knots {
        knot_positions.push((0, 0));
    }

    let mut visited_tail_positions: Vec<(isize, isize)> = vec![(0, 0)];
    let knot_offsets: Vec<(isize, isize)> = (-1..=1)
        .flat_map(|i| (-1..=1).map(move |j| (i, j)))
        .collect();

    for line in input.lines() {
        let trimmed = line.trim();
        let direction_char = trimmed.chars().next().unwrap();
        let count_str = &trimmed[2..];
        let count = count_str.parse::<usize>().unwrap();

        'outer: for _ in 0..count {
            match direction_char {
                'U' => knot_positions[0].1 += 1,
                'D' => knot_positions[0].1 -= 1,
                'L' => knot_positions[0].0 -= 1,
                'R' => knot_positions[0].0 += 1,
                _ => panic!("Invalid direction."),
            };

            for knot_index in 1..knots {
                // On the first loop, this is the position of the head.
                let previous_knot = knot_positions[knot_index - 1];
                let current_knot = &mut knot_positions[knot_index];

                for knot_offset in &knot_offsets {
                    let position = (
                        previous_knot.0 + knot_offset.0,
                        previous_knot.1 + knot_offset.1,
                    );

                    if position.0 == current_knot.0 && position.1 == current_knot.1 {
                        // Knot doesn't need to be moved.
                        // Further, all following knots don't need to be moved.
                        continue 'outer;
                    }
                }

                let xy_matches = (
                    current_knot.0 == previous_knot.0, // Row
                    current_knot.1 == previous_knot.1, // Column
                );

                match xy_matches {
                    (false, true) => match current_knot.0.cmp(&previous_knot.0) {
                        Ordering::Greater => current_knot.0 -= 1,
                        Ordering::Less => current_knot.0 += 1,
                        _ => panic!(),
                    },
                    (true, false) => match current_knot.1.cmp(&previous_knot.1) {
                        Ordering::Greater => current_knot.1 -= 1,
                        Ordering::Less => current_knot.1 += 1,
                        _ => panic!(),
                    },
                    (false, false) => {
                        let delta = (
                            previous_knot.0 - current_knot.0,
                            previous_knot.1 - current_knot.1,
                        );
                        
                        current_knot.0 += delta.0.clamp(-1, 1);
                        current_knot.1 += delta.1.clamp(-1, 1);
                    }
                    _ => panic!(
                        "The head and tails' positions cannot be equal. This was already checked."
                    ),
                };
            }

            let tail_position = knot_positions.last().unwrap();
            if !visited_tail_positions.contains(tail_position) {
                visited_tail_positions.push(*tail_position);
            }
        }

        // println!("{}:\n{}", trimmed, visualize_positions(knot_positions.clone()));
    }

    visited_tail_positions.len()
}

// fn visualize_positions(positions: Vec<(isize, isize)>) -> String {
    // let mut string = String::new();

    // if positions.is_empty() {
        // return string;
    // }

    // let min: (isize, isize) = (-10, -10);
    // let max: (isize, isize) = (30, 10);

    // for y in (min.1..=max.1).rev() {
        // string.push('\n');

        // for x in min.0..=max.0 {
            // if let Some((index, _)) = positions.iter().enumerate().find(|p| p.1.0 == x && p.1.1 == y) {
                // let regular = &format!("{}", 10 - index);
                // string.push_str(if index == 0 { "H" } else { regular });
            // } else if x == 0 && y == 0 {
                // string.push('s');
            // } else if x == 0 && y == -2 {
                // string.push('b');
            // } else if x == 0 {
                // string.push('|');
            // } else if y == 0 {
                // string.push('-');
            // } else {
                // string.push('.');
            // }
        // }
    // }

    // string
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "R 10";
        let result = visited_tail_positions(input, 10);
        let expected = 2;
        assert_eq!(result, expected);
    }


    #[test]
    fn test_2() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let result = visited_tail_positions(input, 2);
        let expected = 13;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let result = visited_tail_positions(input, 10);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_4() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        let result = visited_tail_positions(input, 10);
        let expected = 36;
        assert_eq!(result, expected);
    }
}
