#![allow(non_snake_case)]

fn main() {
    let input = include_str!("input.txt");
    let (visible_trees, best_scenic_score) = process_input(input);
    println!("Visible Trees: {}", visible_trees);
    println!("Best Scenic Score: {}", best_scenic_score);
}

fn process_input(input: &str) -> (usize, usize) {
    // Grid is in [y][x] format.
    let mut grid: Vec<Vec<u8>> = vec![];

    for (line_index, line) in input.lines().enumerate() {
        grid.push(vec![]);

        for character in line.chars() {
            let parsed: u8 = character
                .to_string()
                .parse()
                .expect("Failed to parse character.");
            grid[line_index].push(parsed);
        }
    }

    let height = grid.len() as isize;
    assert!(height != 0, "The program input is invalid.");
    let width = grid[0].len() as isize;

    // Top, down, left, right.
    let direction_increments = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut visible_locations = 0usize;
    let mut best_scenic_score = 0usize;

    for (y, row) in grid.iter().enumerate() {
        for (x, tree_height) in row.iter().enumerate() {
            let position = (x, y);

            let mut scenic_elements = [0, 0, 0, 0];
            let mut visible = false;

            for (i, increment) in direction_increments.iter().enumerate() {
                let mut copy = (
                    position.0 as isize + increment.0,
                    position.1 as isize + increment.1,
                );

                let mut found_blocker = false;
                let mut viewable_trees: usize = 0;

                while is_valid(copy, width, height) {
                    viewable_trees += 1;
                    let other_tree_height = grid[copy.1 as usize][copy.0 as usize];

                    if other_tree_height >= *tree_height {
                        found_blocker = true;
                        break;
                    }

                    copy.0 += increment.0;
                    copy.1 += increment.1;
                }

                scenic_elements[i] = viewable_trees;

                if !found_blocker && !visible {
                    visible = true;
                }
            }

            if visible {
                visible_locations += 1;
            }

            let scenic_score: usize = scenic_elements.into_iter().product();

            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }

    (visible_locations, best_scenic_score)
}

fn is_valid(position: (isize, isize), width: isize, height: isize) -> bool {
    if position.0 < 0 || position.1 < 0 {
        return false;
    }

    if position.0 >= width || position.1 >= height {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_works() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let (visible_locations, best_scenic_score) = process_input(input);
        assert_eq!(visible_locations, 21);
        assert_eq!(best_scenic_score, 8);
    }

    #[test]
    fn is_valid_works() {
        assert!(is_valid((0, 0), 10, 10));
        assert!(is_valid((0, 9), 10, 10));
        assert!(!is_valid((0, -1), 10, 10));
        assert!(!is_valid((-1, 0), 10, 10));
        assert!(!is_valid((-1, -1), 10, 10));
        assert!(!is_valid((0, 10), 10, 10));
    }
}
