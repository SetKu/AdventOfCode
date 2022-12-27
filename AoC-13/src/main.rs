#![allow(non_snake_case)]

use parser::parse_input;

const INPUT_ERR: &str = "Unexpected input.";

struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(std::clone::Clone, Eq, Debug)]
enum Packet {
    Number(u32),
    Container(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Container(l0), Self::Container(r0)) => l0 == r0,
            // Mixed matches forward to container to container comparison ^.
            (Self::Number(l0), Self::Container(r0)) => &vec![Self::Number(*l0)] == r0,
            (Self::Container(l0), Self::Number(r0)) => l0 == &vec![Self::Number(*r0)],
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Number(l0), Packet::Number(r0)) => l0.cmp(r0),
            (Packet::Container(l0), Packet::Container(r0)) => l0.cmp(r0),
            (Packet::Container(l0), Packet::Number(r0)) => l0.cmp(&vec![Packet::Number(*r0)]),
            (Packet::Number(l0), Packet::Container(r0)) => vec![Packet::Number(*l0)].cmp(r0),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::{correctly_ordered_index_sum, parser::parse_input};

    const TEST_DATA: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn example() {
        let parsed = parse_input(TEST_DATA);
        let sum = correctly_ordered_index_sum(&parsed);
        assert_eq!(sum, 13);
    }

    #[test]
    fn compare() {
        let input = "[[1],[2,3,4]]\n[[1],4]";
        let parsed = parse_input(input);
        let only = parsed.first().unwrap();
        assert_eq!(only.left.cmp(&only.right), std::cmp::Ordering::Less);
    }
}

mod parser {
    use super::INPUT_ERR;
    use crate::{Packet, Pair};

    fn pair_strings(input: &str) -> Vec<(&str, &str)> {
        input
            .split("\n\n")
            // This handles trimming the whitespace off
            // the ends automatically.
            .map(|s| s.split_once('\n').expect(INPUT_ERR))
            .collect()
    }

    fn scope_into(mut values: &mut Vec<Packet>, mut depth: usize, last: bool) -> &mut Vec<Packet> {
        while depth > 0 {
            depth -= 1;

            let optional = if last {
                values.last_mut()
            } else {
                values.first_mut()
            };

            if let Packet::Container(content) = optional.unwrap() {
                values = content;
                continue;
            }

            // Value must have been a `Node::Number` to reach this point.
            panic!("A last value in the node tree is a number, not a container.");
        }

        values
    }

    // Things that trip this function up:
    // * Newlines
    // * Interspersed whitespace
    //      * E.g. ' 2' will cause a panic.
    fn nodify(input: &str) -> Packet {
        let mut nodes = vec![];
        let mut depth = 0;
        let mut number_buffer: String = "".to_string();

        for character in input.chars() {
            match character {
                '[' => {
                    scope_into(&mut nodes, depth, true).push(Packet::Container(vec![]));
                    depth += 1;
                }
                ']' => {
                    // Check if the previous buffer contains a number
                    if number_buffer
                        .chars()
                        .next()
                        .map(|s| s.is_numeric())
                        .unwrap_or(false)
                    {
                        let node =
                            Packet::Number(number_buffer.to_string().parse().expect(INPUT_ERR));
                        number_buffer.clear();
                        scope_into(&mut nodes, depth, true).push(node);
                    }

                    if depth == 0 {
                        panic!("Unmatched closing bracket '[' found.");
                    }

                    depth -= 1;
                }
                value => {
                    if value == ',' {
                        if !number_buffer.is_empty() {
                            // A new container node will be created for the single element.
                            let node =
                                Packet::Number(number_buffer.to_string().parse().expect(INPUT_ERR));
                            scope_into(&mut nodes, depth, true).push(node);
                            number_buffer.clear();
                        }

                        continue;
                    }

                    number_buffer.push(value);
                }
            }
        }

        nodes
            .into_iter()
            .next()
            .expect("Why was an empty line parsed?")
    }

    pub(crate) fn parse_input(input: &str) -> Vec<Pair> {
        let line_pairs = pair_strings(input);
        let mut pairs = vec![];

        for set in line_pairs {
            let pair = Pair {
                left: nodify(set.0),
                right: nodify(set.1),
            };

            pairs.push(pair);
        }

        pairs
    }

    #[cfg(test)]
    mod tests {
        use super::nodify;
        use crate::Packet;

        #[test]
        fn parsing() {
            use Packet::*;

            let string = "[[[2],3,[],[]]]";
            let node = nodify(string);

            let expected = Container(vec![Container(vec![
                Container(vec![Number(2)]),
                Number(3),
                Container(vec![]),
                Container(vec![]),
            ])]);

            assert_eq!(node, expected);
        }
    }
}

fn correctly_ordered_index_sum(data: &[Pair]) -> usize {
    data.iter()
        .enumerate()
        .flat_map(|(i, Pair { left, right })| match left.cmp(right) {
            std::cmp::Ordering::Less => Some(i), // Correct order for part one.
            std::cmp::Ordering::Equal => panic!("Equal comparison should never happen."),
            std::cmp::Ordering::Greater => None, // Discard results where left > right
        })
        .map(|i| i + 1)
        .sum::<usize>()
}

fn divider_packets_index_product(data: &[Pair]) -> usize {
    let div_1 = Packet::Container(vec![Packet::Container(vec![Packet::Number(2)])]);
    let div_2 = Packet::Container(vec![Packet::Container(vec![Packet::Number(6)])]);

    let mut copy = data
        .iter()
        .flat_map(|p| [p.left.clone(), p.right.clone()])
        .chain([div_1.clone(), div_2.clone()].into_iter())
        .collect::<Vec<Packet>>();

    copy.sort();

    copy.into_iter()
        .enumerate()
        .filter(|(_, p)| p == &div_1 || p == &div_2)
        .map(|(i, _)| i + 1)
        .product::<usize>()
}

fn main() {
    let input = include_str!("input.txt");
    let data = parse_input(input);

    let part_one = correctly_ordered_index_sum(&data);
    let part_two = divider_packets_index_product(&data);

    println!("Correctly Ordered Pairs' Index Sum: {}", part_one);
    println!("Sorted Packets Divider Index Product: {}", part_two);
}
