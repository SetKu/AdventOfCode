#![allow(non_snake_case)]

use num_bigint::BigUint;

const INPUT_ERR: &str = "The formatting of the input was unexpected.";
const CLOSURE_USAGE_ERR: &str =
    "This closure should never have been called, as it's just a placeholder.";
const OPP_ERR: &str = "Unknown operation in the input.";

fn main() {
    let input = include_str!("input.txt");
    let part_one = monkey_business(input, 3, 20);
    println!("Part One Business: {part_one}");
    let part_two = monkey_business(input, 1, 10000);
    println!("Part Two Business: {part_two}");
}

struct Monkey {
    inspections: u64,
    items: Vec<BigUint>,
    operation: Box<dyn Fn(BigUint) -> BigUint>,
    test_divisor: u64,
    true_index: usize,
    false_index: usize,
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            inspections: 0,
            items: vec![],
            operation: Box::new(|_| panic!("{CLOSURE_USAGE_ERR}")),
            test_divisor: 0,
            true_index: 0,
            false_index: 0,
        }
    }
}

fn monkey_business(input: &str, worry_divider: u32, rounds: u32) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![Monkey::default()];

    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            monkeys.push(Monkey::default());
            continue;
        }

        if trimmed.starts_with("Starting items") {
            let list = trimmed.strip_prefix("Starting items: ").expect(INPUT_ERR);
            let item_strings = list.split(", ");
            let items: Vec<BigUint> = item_strings.map(|s| s.parse().expect(INPUT_ERR)).collect();
            monkeys.last_mut().unwrap().items = items;
        }

        if trimmed.starts_with("Operation") {
            let operation_string = trimmed
                .strip_prefix("Operation: new = old ")
                .expect(INPUT_ERR);
            let segments: Vec<&str> = operation_string.split(' ').collect();
            let last_monkey = monkeys.last_mut().unwrap();
            let failure = |val: &str| panic!("{OPP_ERR} -> '{val}'");

            match segments[0] {
                "*" => match segments[1] {
                    "5" => last_monkey.operation = Box::new(|old| old * 5u8),
                    "old" => last_monkey.operation = Box::new(|old| old.pow(2)),
                    "7" => last_monkey.operation = Box::new(|old| old * 7u8),
                    value => failure(value),
                },
                "+" => match segments[1] {
                    "1" => last_monkey.operation = Box::new(|old| old + 1u8),
                    "3" => last_monkey.operation = Box::new(|old| old + 3u8),
                    "5" => last_monkey.operation = Box::new(|old| old + 5u8),
                    "8" => last_monkey.operation = Box::new(|old| old + 8u8),
                    "2" => last_monkey.operation = Box::new(|old| old + 2u8),
                    value => failure(value),
                },
                value => failure(value),
            }
        }

        if trimmed.starts_with("Test") {
            let test_string = trimmed
                .strip_prefix("Test: divisible by ")
                .expect(INPUT_ERR);
            let division_value: u64 = test_string.parse().expect(INPUT_ERR);
            monkeys.last_mut().unwrap().test_divisor = division_value;
        }

        if trimmed.starts_with("If ") {
            let stripped = trimmed.strip_prefix("If ").expect(INPUT_ERR);

            if let Some(index) = stripped.strip_prefix("true: throw to monkey ") {
                monkeys.last_mut().unwrap().true_index = index.parse().expect(INPUT_ERR);
            } else {
                let index = stripped.strip_prefix("false: throw to monkey ").unwrap();
                monkeys.last_mut().unwrap().false_index = index.parse().expect(INPUT_ERR);
            }
        }
    }

    // The divisor product is the common multiple of all the testing divisors.
    // The reason using this product makes the sequence so much faster
    // is because it caps the ceiling of the integers were working with.
    //
    // The reason the common product is taken and not the greatest divisor
    // is because the common product is a multiple of all the values, whereas the 
    // greatest is not.
    //
    // This allows us to trim the big integers down by removing additional groupings of
    // the common multiple for all modulus operations.
    //
    // This technique is congruent with the multiplication and addition, but not division.
    // This is why this technique can only be used when the worry divisor is 1.
    let divisor_product: u64 = monkeys.iter().map(|m| m.test_divisor.to_owned()).product();

    for _ in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            let current_monkey = &mut monkeys[monkey_index];
            let items = current_monkey.items.clone();
            current_monkey.items.clear();
            current_monkey.inspections += items.len() as u64;

            for mut item in items {
                let monkey_ref = &monkeys[monkey_index];

                // Modulo trick only works if using a worry divisor of one.
                // This is because the trick only applies via the laws of 
                // modular multiplication and addition.
                // Division is a different story...
                //
                // https://is.gd/QzvXLH
                // https://is.gd/qCosZt
                if worry_divider == 1 {
                    item %= divisor_product;
                }

                item = (monkey_ref.operation)(item);
                item /= worry_divider;

                // BigUint doesn't enable the copy trait, so we must clone it.
                if item.clone() % monkey_ref.test_divisor == BigUint::from(0u8) {
                    let true_index = monkey_ref.true_index;
                    monkeys[true_index].items.push(item);
                } else {
                    let false_index = monkey_ref.false_index;
                    monkeys[false_index].items.push(item);
                }
            }
        }
    }

    let mut sorted_inspections: Vec<u64> = monkeys.iter().map(|m| m.inspections).collect();
    sorted_inspections.sort();
    sorted_inspections.reverse();

    sorted_inspections[0] * sorted_inspections[1]
}
