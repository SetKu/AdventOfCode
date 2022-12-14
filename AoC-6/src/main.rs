#![allow(non_snake_case)]

use std::cmp::Ordering;

fn main() {
    let input = include_str!("input.txt");
    let text = input.lines().next().expect("Unexpected puzzle input.");

    let packet_marker = find_unique_set(text, 4) + 1;
    let message_marker = find_unique_set(text, 14) + 1;

    println!("Packet Marker Characters: {packet_marker}");
    println!("Message Marker Characters: {message_marker}");
}

fn find_unique_set(text: &str, len: usize) -> usize {
    let mut char_buffer = vec![];
    let mut marker_end_index = 0;

    for (index, character) in text.chars().enumerate() {
        char_buffer.push(character);

        let compare = len.cmp(&char_buffer.len());

        if compare == Ordering::Less {
            char_buffer.remove(0);
        } else if compare == Ordering::Greater {
            continue;
        }

        debug_assert_eq!(char_buffer.len(), len);

        let mut buffer_copy = char_buffer.clone();
        buffer_copy.sort();
        buffer_copy.dedup();

        if buffer_copy.len() == len {
            marker_end_index = index;
            break;
        }
    }

    marker_end_index
}
