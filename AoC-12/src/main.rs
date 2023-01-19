#![allow(non_snake_case)]

use dirs::desktop_dir;
use std::{clone::Clone, collections::HashMap, fmt::Debug};

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    // Excludes negative values.
    fn neighbours(&self) -> Vec<Self> {
        [[-1, 0], [0, -1], [1, 0], [0, 1]]
            .into_iter()
            .map(|p| (self.x as isize + p[0], self.y as isize + p[1]))
            .filter_map(|v| {
                if v.0 > -1 && v.1 > -1 {
                    Some(Position {
                        x: v.0 as usize,
                        y: v.1 as usize,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Debug)]
struct Node {
    distance: u32,
    position: Position,
}

fn main() {
    let input = include_str!("input.txt");
    let mut grid_info = grid(input);

    if !std::env::args().any(|a| a == "--noimage") {
        if let Some(mut desktop) = desktop_dir() {
            let image = graphic::imagify(&grid_info);
            desktop.push("height_map.png");
            let path = desktop.as_path();
            image.save(path).expect("Failed to save image.");
            println!("Saved image to {}.", path.to_str().unwrap());
        }
    }

    // Swap star† and end to find the distance
    // from the end to any position.
    let start = grid_info.start.clone();
    grid_info.start = grid_info.end.clone();
    grid_info.end = start.clone();

    let table = dijkstra(&grid_info);

    println!("Shortest Path from Start: {}", table[&start]);
    println!(
        "Shortest Path from Ground: {}",
        shortest_from_ground(&grid_info, &table)
    );
}

struct GridInfo {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    start: Position,
    end: Position,
}

fn grid(input: &str) -> GridInfo {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut start = Position { x: 0, y: 0 };
    let mut end: Position = Position { x: 0, y: 0 };
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        grid.push(vec![]);

        for (x, character) in line.trim().chars().enumerate() {
            if x + 1 >= width {
                width = x + 1;
            }

            if y + 1 >= height {
                height = y + 1;
            }

            let position = Position { x, y };

            let height = match character {
                'S' => {
                    start = position;
                    b'a'
                }
                'E' => {
                    end = position;
                    b'z'
                }
                other => other as u8,
            };

            grid[y].push(height);
        }
    }

    GridInfo {
        grid,
        width,
        height,
        start,
        end,
    }
}

fn dijkstra(grid_info: &GridInfo) -> HashMap<Position, u32> {
    // Dijkstra's Algorithm
    // Prioritizes searching shorter paths first.
    // https://youtu.be/GazC3A4OQTE

    let mut visited: Vec<Position> = vec![];
    visited.reserve(grid_info.width * grid_info.height);

    let mut priority_queue = vec![Node {
        distance: 0,
        position: grid_info.start.clone(),
    }];

    let mut distances = HashMap::new();

    loop {
        if priority_queue.is_empty() {
            break;
        }

        priority_queue.sort_by_key(|n| n.distance);

        // Remove will never panic as the queue was
        // checked to be empty above.
        let local_node = priority_queue.remove(0);

        visited.push(local_node.position.clone());

        // Unsafe index here means any nodes outside the
        // grid space need to be removed prior to this point.
        let local_height = grid_info.grid[local_node.position.y][local_node.position.x];

        for neighbour_pos in local_node.position.neighbours() {
            if neighbour_pos.x >= grid_info.width || neighbour_pos.y >= grid_info.height {
                continue;
            }

            let neighbour_height = grid_info.grid[neighbour_pos.y][neighbour_pos.x];

            if !visited.contains(&neighbour_pos) {
                if let Some(existing_node) = priority_queue
                    .iter_mut()
                    .find(|n| n.position == neighbour_pos)
                {
                    if existing_node.distance > local_node.distance + 1 {
                        existing_node.distance = local_node.distance + 1;
                    }

                    continue;
                }

                // The height won't overflow as the min value ('a') is above 90.
                // The max ('z') also doesn't reach to 254.
                //
                // Can only go up by 1 in height.
                // However, can go infinitely down (drop/fall) in height.
                //
                // ^ Confusingly, this is implemented in the inverse here to allow
                // searching from the end instead of the start. So, from the end
                // navigation can be done 1 down and an infinite height up.
                if local_height - 2 < neighbour_height {
                    priority_queue.push(Node {
                        distance: local_node.distance + 1,
                        position: neighbour_pos,
                    });
                }
            }
        }

        distances.insert(local_node.position, local_node.distance);
    }

    if distances.is_empty() {
        panic!("Failed to find any path.");
    }

    distances
}

fn shortest_from_ground(grid_info: &GridInfo, search: &HashMap<Position, u32>) -> u32 {
    let mut shortest = u32::MAX;

    for (pos, distance) in search {
        if grid_info.grid[pos.y][pos.x] == b'a' && *distance < shortest {
            shortest = *distance;
        }
    }

    shortest
}

#[cfg(test)]
mod tests {
    use crate::{dijkstra, grid, Position};

    #[test]
    fn char_comparison() {
        let mut last: Option<u8> = None;

        for character in b'a'..=b'z' {
            if last.is_none() {
                last = Some(character);
                continue;
            }

            assert!(last.unwrap() < character);
        }
    }

    #[test]
    fn neighbours_work() {
        let mut neighbours = (Position { x: 1, y: 1 }).neighbours();
        let mut expected = vec![
            Position { x: 0, y: 1 },
            Position { x: 1, y: 0 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 1 },
        ];

        neighbours.sort();
        expected.sort();
        assert_eq!(neighbours, expected);

        let zero = Position { x: 0, y: 0 };
        assert_eq!(zero.neighbours().len(), 2);
    }

    #[test]
    fn sample_input() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
        let expected = 31;
        let mut info = grid(input);

        let start = info.start.clone();
        info.start = info.end.clone();
        info.end = start;

        let result = dijkstra(&info);
        assert_eq!(expected, result[&info.end]);
    }
}

mod graphic {
    use super::GridInfo;
    use image::{
        imageops::{resize, FilterType},
        ImageBuffer, Rgb,
    };

    pub(crate) fn imagify(grid_info: &GridInfo) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let color: Rgb<u8> = Rgb([191, 204, 148]);
        let color_factor: u8 = 5;
        let scale_factor: usize = 8;

        let mut buffer = ImageBuffer::new(grid_info.width as u32, grid_info.height as u32);

        for x in 0..grid_info.width {
            for y in 0..grid_info.height {
                let value = grid_info.grid[y][x];
                let mut pixel = color;
                let diff = (value - 96) * color_factor;
                pixel.0[0] -= diff;
                pixel.0[1] -= diff;
                pixel.0[2] -= diff;
                buffer.put_pixel(x as u32, y as u32, pixel)
            }
        }

        resize(
            &buffer,
            (grid_info.width * scale_factor) as u32,
            (grid_info.height * scale_factor) as u32,
            FilterType::Nearest,
        )
    }
}
