#![allow(non_snake_case)]

// Advent of Code 2022
// Day 5: https://adventofcode.com/2022/day/5

const CONTAINER_BLOCK_SIZE: usize = 4;

type Container = char;

fn main() {
    let input = include_str!("input.txt");
    let mut stacks: Vec<Vec<Container>> = vec![];

    let mut instruction_index = 0;

    for (line_index, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            instruction_index = line_index + 1;
            break;
        }

        for (char_index, character) in line.chars().enumerate() {
            // 4 characters constitute each section for a container.
            // e.g. "[Q] "
            // Integer division rounding behaviour (down to zero): https://is.gd/BqmlK1
            let stack_index = char_index / CONTAINER_BLOCK_SIZE;

            if character.is_ascii_alphabetic() {
                if stacks.len() < stack_index + 1 {
                    for _ in stacks.len()..(stack_index + 1) {
                        stacks.push(vec![]);
                    }
                }

                stacks[stack_index].insert(0, character);
            }
        }
    }

    let mut part_one = stacks.clone();
    let mut part_two = stacks;

    for line in input.lines().skip(instruction_index) {
        let mut copy = line.to_string();
        let error_message = "Unexpected input.";

        copy = copy.strip_prefix("move ").expect(error_message).to_string();
        let post_count_index = copy.find(' ').expect(error_message);
        let container_count = copy[0..post_count_index]
            .parse::<usize>()
            .expect(error_message);

        copy = copy[(post_count_index + 1)..]
            .strip_prefix("from ")
            .expect(error_message)
            .to_string();
        let post_origin_index = copy.find(' ').expect(error_message);
        let origin_stack_index = copy[0..post_origin_index]
            .parse::<usize>()
            .expect(error_message)
            - 1;

        copy = copy[(post_origin_index + 1)..]
            .strip_prefix("to ")
            .expect(error_message)
            .to_string();
        let dest_stack_index = copy[0..].parse::<usize>().expect(error_message) - 1;

        for _ in 0..container_count {
            let container = part_one[origin_stack_index].pop().expect(error_message);
            part_one[dest_stack_index].push(container);
        }

        let mut containers: Vec<Container> = vec![];

        for _ in 0..container_count {
            containers.push(part_two[origin_stack_index].pop().expect(error_message));
        }

        while !containers.is_empty() {
            part_two[dest_stack_index].push(containers.pop().unwrap());
        }
    }

    for (i, part) in [part_one, part_two].into_iter().enumerate() {
        print!("Top Row (Part {}): ", i + 1);

        for stack in part {
            print!("{}", stack.last().unwrap_or(&' '));
        }

        println!();
    }
}
