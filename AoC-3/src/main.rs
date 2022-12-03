#![allow(non_snake_case)]

use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let mut type_priorities = HashMap::new();

    for (i, code) in (b'a'..=b'z').chain(b'A'..=b'Z').enumerate() {
        type_priorities.insert(code as char, i + 1);
    }

    let mut dup_compartment_item_types = vec![];
    let mut dup_group_item_types = vec![];
    let mut current_group_item_types: Vec<char> = vec![];

    for (rucksack_index, rucksack) in input.lines().enumerate() {
        let parsed = rucksack.trim();
        let middle_index = parsed.len() / 2;
        let compartments = parsed.split_at(middle_index);
        assert_eq!(compartments.0.len(), compartments.1.len());

        let mut compartment_item = None;

        'outer: for character in compartments.0.chars() {
            for acquaintance in compartments.1.chars() {
                if character == acquaintance {
                    compartment_item = Some(character);
                    break 'outer;
                }
            }
        }

        if let Some(item) = compartment_item {
            dup_compartment_item_types.push(item);
        }

        let group = rucksack_index % 3;

        if group == 0 {
            current_group_item_types = rucksack.chars().collect();
            continue;
        }

        current_group_item_types = current_group_item_types
            .into_iter()
            .filter(|c| rucksack.chars().any(|r| r == *c))
            .collect();

        if group == 2 {
            if let Some(item_type) = current_group_item_types.first() {
                dup_group_item_types.push(*item_type);
            }
        }
    }

    let compartments_total: usize = dup_compartment_item_types
        .into_iter()
        .map(|c| type_priorities[&c])
        .sum();
    let groups_total: usize = dup_group_item_types
        .into_iter()
        .map(|c| type_priorities[&c])
        .sum();

    println!(
        "🎒 Compartment Priorities Sum: {}",
        compartments_total
    );
    println!(
        "🎅 Group Priorities Sum: {}",
        groups_total
    );
}
