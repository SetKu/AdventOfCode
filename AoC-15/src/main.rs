#![allow(non_snake_case)]

use std::ops::RangeInclusive;

use nalgebra::{Vector2, vector};
use nom::bytes::complete::take_till;
use nom::character::complete::{i64 as parse_i64, multispace1};
use nom::character::is_digit;
use nom::{multi::separated_list0, sequence::tuple, IResult};

#[derive(Debug)]
struct Pair {
    sensor: Vector2<i64>,
    beacon: Vector2<i64>,
}

fn not_a_digit(input: &str) -> IResult<&str, &str> {
    take_till(|v| is_digit(v as u8) || v == '-' || v == '+')(input)
}

fn parse_line(input: &str) -> IResult<&str, Pair> {
    // Note: None of the parsers will fail if they take nothing. Therefore, failure must be checked for after.
    tuple((
        not_a_digit,
        parse_i64,
        not_a_digit,
        parse_i64,
        not_a_digit,
        parse_i64,
        not_a_digit,
        parse_i64,
    ))(input)
    .map(|result| {
        let sensor = Vector2::new(result.1 .1, result.1 .3);
        let beacon = Vector2::new(result.1 .5, result.1 .7);
        (result.0, Pair { sensor, beacon })
    })
}

fn parse(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list0(multispace1, parse_line)(input)
}

fn main() {
    let input = include_str!("input.txt");
    let data = parse(input).expect("Failed to parse input.");

    assert!(
        data.0.trim().is_empty(),
        "Failed to parse remaining input:\n{:?}",
        data.0
    );

    let one = part_one(&data.1, 2_000_000);
    println!("Part One: {one}");
    let two = part_two(&data.1, 4_000_000);
    println!("Part Two: {two}");
}

// Order of a and b doesn't matter.
fn manhattan(a: &Vector2<i64>, b: &Vector2<i64>) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn domains_by_level(data: &[Pair]) -> Vec<(i64, RangeInclusive<i64>)> {
    data.iter()
        .map(|pair| (pair.sensor, manhattan(&pair.sensor, &pair.beacon)))
        .flat_map(|(sensor, distance)| {
            (-distance..=distance)
                .map(|offset| {
                    (
                        sensor.y + offset,
                        (sensor.x - distance + offset.abs())..=(sensor.x + distance - offset.abs()),
                    )
                })
                .collect::<Vec<(i64, RangeInclusive<i64>)>>()
        })
        .collect::<Vec<_>>()
}

fn part_one(data: &[Pair], row_index: i64) -> u32 {
    let mut col_indexes = domains_by_level(data)
        .into_iter()
        .filter_map(|domain| {
            if domain.0 == row_index {
                Some(domain.1)
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let relevant_beacons = data
        .iter()
        .filter_map(|pair| {
            if pair.beacon.y == row_index {
                Some(pair.beacon.x)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // Unique
    col_indexes.sort();
    col_indexes.dedup();

    col_indexes = col_indexes
        .into_iter()
        .filter(|index| !relevant_beacons.contains(index))
        .collect::<Vec<_>>();

    col_indexes.len() as u32
}

fn part_two(data: &[Pair], limit: i64) -> i64 {
    let distances = data
        .iter()
        .map(|pair| (pair.sensor, manhattan(&pair.sensor, &pair.beacon)))
        .collect::<Vec<_>>();
    
    let answer = domains_by_level(data)
        .into_iter()
        // Instead of iterating over all the trillions of values in the input,
        // we instead just iterate over the couple million which surround the 
        // sensor-beacon ranges, avoiding checking a huge number of frivilous
        // coordinates.
        .flat_map(|(y, range)| {
            if range.end() == range.start() {
                // This technique is inneficient, as it takes the positions above and below
                // for capping points to the diamond shapes. However, it's functional
                // and as of writing, I'm getting the answer in good time. So I'll leave it here
                // for now.
                [
                    (*range.start(), y - 1),
                    (*range.start(), y + 1),
                ]
            } else {
                [
                    (*range.start() - 1, y),
                    (*range.end() + 1, y),
                ]
            }
        })
        .filter(|(x, y)| (0..=limit).contains(x) && (0..=limit).contains(y))
        .find(|(x, y)| {
            !distances.iter().any(|(sensor, distance)| {
                manhattan(sensor, &vector![*x, *y]) <= *distance
            })
        })
        .expect("Failed to find answer for part two.");

    return answer.0 * 4_000_000 + answer.1;
}

#[cfg(test)]
mod tests {
    use crate::{parse, parse_line, part_one, part_two, domains_by_level};

    #[test]
    fn part_one_works() {
        let input = include_str!("test.txt");
        let data = parse(input).unwrap();
        assert!(data.0.is_empty());
        let result = part_one(&data.1, 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn part_two_works() {
        let input = include_str!("test.txt");
        let data = parse(input).unwrap();
        assert!(data.0.is_empty());
        let result = part_two(&data.1, 20);
        assert_eq!(result, 56000011);
    }

    #[test]
    fn parse_line_works() {
        let input = "Sensor at x=-3729579, y=1453415: closest beacon is at x=4078883, y=2522671";
        let result = parse_line(input).expect("Parsing straight-up failed.");
        assert!(result.0.is_empty());
        assert_eq!(result.1.sensor.x, -3729579);
        assert_eq!(result.1.sensor.y, 1453415);
        assert_eq!(result.1.beacon.x, 4078883);
        assert_eq!(result.1.beacon.y, 2522671);
    }

    #[test]
    fn domain_leveling_works() {
        let input_1 = "Sensor at x=15, y=5: closest beacon is at x=20, y=3";
        let result_1 = parse_line(input_1).expect("Parsing straight-up failed for first input.").1;
        let mut domains_1 = domains_by_level(&vec![result_1]);
        domains_1.sort_by_key(|domain| domain.0);

        let input_2 = "Sensor at x=15, y=5: closest beacon is at x=10, y=3";
        let result_2 = parse_line(input_2).expect("Parsing straight-up failed for second input.").1;
        let mut domains_2 = domains_by_level(&vec![result_2]);
        domains_2.sort_by_key(|domain| domain.0);

        let expected = [
            (-2, 15..=15),
            (-1, 14..=16),
            ( 0, 13..=17),
            ( 1, 12..=18),
            ( 2, 11..=19),
            ( 3, 10..=20),
            ( 4,  9..=21),
            ( 5,  8..=22),
            ( 6,  9..=21),
            ( 7, 10..=20),
            ( 8, 11..=19),
            ( 9, 12..=18),
            (10, 13..=17),
            (11, 14..=16),
            (12, 15..=15),
        ];

        assert_eq!(domains_1, expected);
        assert_eq!(domains_2, expected);
    }
}
