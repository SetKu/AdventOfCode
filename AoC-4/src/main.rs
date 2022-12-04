#![allow(non_snake_case)]

// Advent of Code
// Day 4: https://adventofcode.com/2022/day/4

fn main() {
    let input = include_str!("input.txt");
    let mut completely_overlapping_pairs = 0;
    let mut overlapping_pairs = 0;

    for line in input.lines() {
        let pair_string_ints: Vec<(usize, usize)> = line
            .trim()
            .split(',')
            .map(|s| s.split_once('-').unwrap())
            .map(|p| (p.0.parse::<usize>().unwrap(), p.1.parse::<usize>().unwrap()))
            .collect();

        let mut any_satisfied = false;
        let mut all_satisfied = false;

        'outer: for (i, pair) in pair_string_ints.iter().enumerate() {
            for (j, other_pair) in pair_string_ints.iter().enumerate() {
                if i == j {
                    continue;
                }

                let mut pair_iter = pair.0..=pair.1;
                let other_pair_iter = other_pair.0..=other_pair.1;

                if !any_satisfied && pair_iter.clone().any(|i| other_pair_iter.contains(&i)) {
                    overlapping_pairs += 1;
                    any_satisfied = true;
                }

                if !all_satisfied && pair_iter.all(|i| other_pair_iter.contains(&i)) {
                    completely_overlapping_pairs += 1;
                    all_satisfied = true;
                }

                if any_satisfied && all_satisfied {
                    break 'outer;
                }
            }
        }
    }

    println!(
        "Total Pairs: {}",
        input.lines().filter(|s| !s.trim().is_empty()).count()
    );
    println!(
        "Completely Overlapping Pairs: {}",
        completely_overlapping_pairs
    );
    println!("Overlapping Pairs: {}", overlapping_pairs);
}
