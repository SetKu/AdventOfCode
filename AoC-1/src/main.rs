#![allow(non_snake_case)]

fn main() {
    let input = include_str!("input.txt");
    let mut top_three = [0usize, 0, 0];
    let mut current = Some(0);

    for line in input.lines() {
        let stripped = line.replace('\n', "");

        if stripped.is_empty() {
            current = None;
            continue;
        }

        let value: usize = stripped.parse().unwrap();

        if current.is_some() {
            current = Some(current.unwrap() + value);
        } else {
            current = Some(value)
        }

        let mut shift_index = None;

        for (top_index, top_value) in top_three.iter().enumerate() {
            if top_value < &current.unwrap() {
                shift_index = Some(top_index);
                break;
            }
        }

        if let Some(index) = shift_index {
            for shift_index in index..2 {
                top_three[shift_index + 1] = top_three[shift_index];
            }

            top_three[index] = current.unwrap();
        }
    }

    println!("Part 1: Greatest Number of Calories: {}", top_three[0]);
    println!(
        "Part 2: Total Number of Calories (top three elves): {}",
        top_three.iter().sum::<usize>()
    );
}
