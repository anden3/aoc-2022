use std::{cmp::Reverse, mem::MaybeUninit};

use regex::bytes::{Captures, Regex};

pub struct Monkey {
    items: [u64; 64],
    item_count: usize,

    operation: Operation,

    divisor: u64,
    index_true: usize,
    index_false: usize,

    inspection_count: usize,
}

impl Monkey {
    pub fn add_item(&mut self, item: u64) {
        self.items[self.item_count] = item;
        self.item_count += 1;
    }

    pub fn items(&mut self) -> ([u64; 64], usize) {
        let items = std::mem::replace(&mut self.items, [0; 64]);
        let count = std::mem::take(&mut self.item_count);

        self.inspection_count += count;
        (items, count)
    }

    pub fn test(&self, value: u64) -> usize {
        match (value % self.divisor) == 0 {
            true => self.index_true,
            false => self.index_false,
        }
    }
}

impl<'a> From<Captures<'a>> for Monkey {
    fn from(cap: Captures<'a>) -> Self {
        let mut items = [0; 64];
        let mut item_count = 0;

        for item in cap["items"].split(|&b| b == b',').map(slice_to_int) {
            items[item_count] = item as _;
            item_count += 1;
        }

        Self {
            items,
            item_count,
            operation: Operation::new(cap["op"][0], &cap["value"]),
            divisor: slice_to_int(&cap["test_param"]) as _,
            index_true: slice_to_int(&cap["true_idx"]) as usize,
            index_false: slice_to_int(&cap["false_idx"]) as usize,
            inspection_count: 0,
        }
    }
}

pub enum Operation {
    Add(u8),
    Multiply(u8),
    MultiplySelf,
}

impl Operation {
    pub fn new(op: u8, value: &[u8]) -> Self {
        match (op, value) {
            (b'*', b"old") => Self::MultiplySelf,
            (b'+', val) => Self::Add(slice_to_int(val)),
            (b'*', val) => Self::Multiply(slice_to_int(val)),
            _ => unreachable!(),
        }
    }

    pub fn apply(&self, val: u64) -> u64 {
        match *self {
            Operation::Add(arg) => val + (arg as u64),
            Operation::Multiply(arg) => val * (arg as u64),
            Operation::MultiplySelf => val * val,
        }
    }
}

pub fn slice_to_int(slice: &[u8]) -> u8 {
    match slice.trim_ascii_start() {
        [count @ b'0'..=b'9'] => count - b'0',
        [count_a @ b'0'..=b'9', count_b @ b'0'..=b'9'] => (count_a - b'0') * 10 + (count_b - b'0'),
        _ => unreachable!(),
    }
}

#[cfg(not(test))]
const MONKEY_COUNT: usize = 8;
#[cfg(test)]
const MONKEY_COUNT: usize = 4;

#[cfg(not(test))]
const LEAST_COMMON_PRIME_MULTIPLE: u64 = 9699690;
#[cfg(test)]
const LEAST_COMMON_PRIME_MULTIPLE: u64 = 96577;

pub fn parse_monkeys(input: &str) -> [Monkey; MONKEY_COUNT] {
    let regex = Regex::new(
        r"(?m)^Monkey [0-9]:
  Starting items: (?P<items>(?:[0-9]{1,2}(?:, )?)+)
  Operation: new = old (?P<op>[*+]) (?P<value>.+)
  Test: divisible by (?P<test_param>[0-9]+)
    If true: throw to monkey (?P<true_idx>[0-9]+)
    If false: throw to monkey (?P<false_idx>[0-9]+)$",
    )
    .unwrap();

    let mut monkeys: [MaybeUninit<Monkey>; MONKEY_COUNT] =
        unsafe { MaybeUninit::uninit().assume_init() };

    for (i, monkey) in regex
        .captures_iter(input.as_bytes())
        .map(Monkey::from)
        .enumerate()
    {
        monkeys[i].write(monkey);
    }

    unsafe { std::mem::transmute(monkeys) }
}

#[aoc(day11, part1)]
pub fn run_part1(input: &str) -> i64 {
    monkey_rounds::<20, 3>(parse_monkeys(input)) as i64
}

#[aoc(day11, part2)]
pub fn run_part2(input: &str) -> i64 {
    monkey_rounds::<10_000, 1>(parse_monkeys(input)) as i64
}

pub fn monkey_rounds<const ROUNDS: usize, const WORRY_DIV: u64>(
    mut monkeys: [Monkey; MONKEY_COUNT],
) -> usize {
    for _ in 0..ROUNDS {
        for i in 0..MONKEY_COUNT {
            let (mut items, count) = monkeys[i].items();

            for item in &mut items[..count] {
                *item = monkeys[i].operation.apply(*item) % LEAST_COMMON_PRIME_MULTIPLE;
                *item /= WORRY_DIV;

                let next_monkey = monkeys[i].test(*item);
                monkeys[next_monkey].add_item(*item);
            }
        }
    }

    monkeys.select_nth_unstable_by_key(1, |m| Reverse(m.inspection_count));
    monkeys[..2].iter().map(|m| m.inspection_count).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_example() {
        assert_eq!(run_part1(TEST_INPUT), 10605);
    }

    #[test]
    fn part2_example() {
        assert_eq!(run_part2(TEST_INPUT), 2713310158);
    }
}
