#![allow(non_snake_case)]

const ZERO: Vector2<i32> = vec2(0, 0);

use cgmath::{vec2, Vector2};
use parser::parse;

mod parser {
    use cgmath::{vec2, Vector2};
    use nom::{
        bytes::complete::tag,
        character::complete::i32 as parse_i32,
        multi::{separated_list0, separated_list1},
        sequence::separated_pair,
        IResult, Parser,
    };

    fn analyze_line(input: &str) -> IResult<&str, Vec<Vector2<i32>>> {
        separated_list1(
            tag(" -> "),
            separated_pair(parse_i32, tag(","), parse_i32).map(|p| vec2(p.0, p.1)),
        )(input)
    }

    pub(crate) fn parse(input: &str) -> IResult<&str, Vec<Vec<Vector2<i32>>>> {
        separated_list0(tag("\n"), analyze_line)(input)
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Block {
    Rock,
    Sand,
}

fn build_map(data: Vec<Vec<Vector2<i32>>>) -> Vec<(Vector2<i32>, Block)> {
    let mut map = vec![];

    for path in data {
        let mut previous = path[0].clone();

        for point in path {
            let mut diff = point - previous;

            loop {
                if diff.x != 0 {
                    // Inch towards zero
                    diff.x -= diff.x.signum();
                } else if diff.y != 0 {
                    diff.y -= diff.y.signum();
                }

                let step = point - diff;
                map.push((step, Block::Rock));

                if diff == ZERO {
                    break;
                }
            }

            previous = point;
        }
    }

    map
}

fn total_sand(map: &Vec<(Vector2<i32>, Block)>) -> usize {
    map.iter().filter(|e| e.1 == Block::Sand).count()
}

fn escapes(pos: &Vector2<i32>) -> [Vector2<i32>; 3] {
    [pos + vec2(0, 1), pos + vec2(-1, 1), pos + vec2(1, 1)]
}

fn first_valid_escape(map: &Vec<(Vector2<i32>, Block)>, pos: Vector2<i32>) -> Option<Vector2<i32>> {
    for escape in escapes(&pos) {
        // Check whether a block invalidates this escape.
        if map.iter().find(|p| p.0 == escape).is_none() {
            return Some(escape);
        }
    }

    None
}

fn part_one(map: &mut Vec<(Vector2<i32>, Block)>, source: &Vector2<i32>) -> u32 {
    let (_, max) = min_max(&map);

    loop {
        let mut sand_pos = source.clone();
        let mut rested = false;

        while !rested {
            if let Some(escape) = first_valid_escape(&map, sand_pos) {
                // Capture end state.
                if escape.y > max.y {
                    return total_sand(&map) as u32;
                }

                sand_pos = escape;
                continue;
            }

            rested = true;
            map.push((sand_pos, Block::Sand));
        }
    }
}

fn part_two(map: &mut Vec<(Vector2<i32>, Block)>, source: &Vector2<i32>) -> u32 {
    let (_, max) = min_max(&map);

    loop {
        let mut sand_pos = source.clone();
        let mut rested = false;

        while !rested {
            if let Some(escape) = first_valid_escape(&map, sand_pos) {
                if escape.y != max.y + 2 {
                    sand_pos = escape;
                    continue;
                }
            }

            map.push((sand_pos, Block::Sand));

            if sand_pos == *source {
                return total_sand(&map) as u32;
            }

            rested = true;
        }
    }
}

fn main() {
    if !std::env::args().any(|a| a == "--noimage") {
        println!("Image mode enabled.");
    } else {
        println!("Image mode disabled.");
    }
   
    let input = include_str!("input.txt");
    let data = parse(input).expect("Failed to parse input.").1;
    let map = build_map(data);

    let mut map_one = map.clone();
    let mut map_two = map.clone();
    std::mem::drop(map);

    let source = vec2(500, 0);

    let ans_one = part_one(&mut map_one, &source);
    println!("Part One: {}", ans_one);

    println!("Working on part two... It takes longer, but will finish within about a minute or two.");

    let ans_two = part_two(&mut map_two, &source);
    println!("Part Two: {}", ans_two);

    if !std::env::args().any(|a| a == "--noimage") {
        let desktop = dirs::desktop_dir().unwrap();

        let image = graphic::snapshot(&map_one);
        let mut path = desktop.clone();
        path.push("Day 14 Part 1.png");
        image.save(path.clone()).expect("Failed to save image.");
        println!("Saved part one image to {}", path.to_str().unwrap());

        graphic::build_floor(&mut map_two);
        let image = graphic::snapshot(&map_two);
        let mut path = desktop;
        path.push("Day 14 Part 2.png");
        image.save(path.clone()).expect("Failed to save image.");
        println!("Saved part two image to {}", path.to_str().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::{build_map, parse, part_one, part_two};
    use cgmath::{vec2, Vector2};

    #[test]
    fn parsing() {
        let example = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        let result = parse(example).unwrap();

        assert_eq!(result.0, "");
        assert_eq!(
            result.1,
            vec![
                vec![vec2(498, 4), vec2(498, 6), vec2(496, 6)],
                vec![vec2(503, 4), vec2(502, 4), vec2(502, 9), vec2(494, 9)],
            ]
        );
    }

    #[test]
    fn mapping() {
        let sort = |a: &mut Vec<Vector2<i32>>| {
            a.sort_by(|v0, v1| v0.x.cmp(&v1.x).then(v0.y.cmp(&v1.y)));
        };

        let data = vec![
            vec![vec2(498, 4), vec2(498, 6), vec2(496, 6)],
            vec![vec2(503, 4), vec2(502, 4), vec2(502, 9), vec2(494, 9)],
        ];

        let map = build_map(data.clone());
        let mut keys = map.into_iter().map(|p| p.0).collect::<Vec<Vector2<i32>>>();
        sort(&mut keys);

        let mut expected = data.into_iter().flatten().collect::<Vec<Vector2<i32>>>();
        expected.append(&mut vec![
            vec2(498, 5),
            vec2(497, 6),
            vec2(502, 5),
            vec2(502, 6),
            vec2(502, 7),
            vec2(502, 8),
            vec2(501, 9),
            vec2(500, 9),
            vec2(499, 9),
            vec2(498, 9),
            vec2(497, 9),
            vec2(496, 9),
            vec2(495, 9),
        ]);
        sort(&mut expected);

        assert_eq!(keys, expected);
    }

    #[test]
    fn part_one_example() {
        let data = vec![
            vec![vec2(498, 4), vec2(498, 6), vec2(496, 6)],
            vec![vec2(503, 4), vec2(502, 4), vec2(502, 9), vec2(494, 9)],
        ];
        let mut map = build_map(data);
        let sand_source = vec2(500, 0);
        let result = part_one(&mut map, &sand_source);
        assert_eq!(result, 24);
    }

    #[test]
    fn part_two_example() {
        let data = vec![
            vec![vec2(498, 4), vec2(498, 6), vec2(496, 6)],
            vec![vec2(503, 4), vec2(502, 4), vec2(502, 9), vec2(494, 9)],
        ];
        let mut map = build_map(data);
        let sand_source = vec2(500, 0);
        let result = part_two(&mut map, &sand_source);
        assert_eq!(result, 93);
    }
}

fn min_max(map: &Vec<(Vector2<i32>, Block)>) -> (Vector2<i32>, Vector2<i32>) {
    let mut min = map
        .first()
        .expect("The map must contain at least one value to minmax it.")
        .0;
    let mut max = min.clone();

    for pos in map.iter().map(|p| p.0) {
        if pos.x > max.x {
            max.x = pos.x;
        }

        if pos.y > max.y {
            max.y = pos.y;
        }

        if pos.x < min.x {
            min.x = pos.x;
        }

        if pos.y < min.y {
            min.y = pos.y;
        }
    }

    (min, max)
}

mod graphic {
    use crate::{min_max, Block};
    use cgmath::{vec2, Vector2};
    use image::{
        imageops::{resize, FilterType},
        Rgb, RgbImage,
    };

    const FLOOR_PADDING: u32 = 1;
    const IMAGE_SCALE: u32 = 8;
    const IMAGE_PADDING: i32 = 3;

    pub(crate) fn snapshot(map: &Vec<(Vector2<i32>, Block)>) -> RgbImage {
        let (mut min, mut max) = min_max(&map);

        max += vec2(IMAGE_PADDING, IMAGE_PADDING) + vec2(1, 1);
        min += vec2(-1 * IMAGE_PADDING, -1 * IMAGE_PADDING);

        let size = max - min;
        let mut image = RgbImage::new(size.x as u32, size.y as u32);

        for x in min.x..max.x {
            for y in min.y..max.y {
                let point = vec2(x, y);
                let block = map.iter().find(|v| v.0 == point).map(|p| &p.1);

                let color: Rgb<u8> = match block {
                    Some(value) => match value {
                        Block::Rock => Rgb([53, 50, 56]),
                        Block::Sand => Rgb([190, 90, 56]),
                    },
                    None => Rgb([193, 180, 174]),
                };

                let image_pos = point - min;
                assert!(image_pos.x > -1);
                assert!(image_pos.y > -1);
                image.put_pixel(image_pos.x as u32, image_pos.y as u32, color);
            }
        }

        resize(
            &image,
            size.x as u32 * IMAGE_SCALE,
            size.y as u32 * IMAGE_SCALE,
            FilterType::Nearest,
        )
    }

    pub(crate) fn build_floor(map: &mut Vec<(Vector2<i32>, Block)>) {
        let (min, max) = min_max(&map);
        let y = max.y + 1;

        for x in (min.x - FLOOR_PADDING as i32)..=(max.x + FLOOR_PADDING as i32) {
            let pos = vec2(x, y);
            map.push((pos, Block::Rock));
        }
    }
}
