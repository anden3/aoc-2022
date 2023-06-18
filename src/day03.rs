use itertools::Itertools;

#[aoc(day3, part1)]
pub fn run_part1(input: &str) -> i64 {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a, b)| find_common_item([a, b]))
        .map(get_priority)
        .sum()
}

#[aoc(day3, part2)]
pub fn run_part2(input: &str) -> i64 {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .tuples()
        .map(|(a, b, c)| find_common_item([a, b, c]))
        .map(get_priority)
        .sum()
}

pub fn find_common_item<const SLICE_COUNT: usize>(slices: [&[u8]; SLICE_COUNT]) -> u8 {
    let mut found_items = [0u8; (b'z' - b'A') as usize + 1];

    for slice in &slices[..(SLICE_COUNT - 1)] {
        let mut found_in_slice = [false; (b'z' - b'A') as usize + 1];

        for item in *slice {
            let index = (*item - b'A') as usize;

            if !found_in_slice[index] {
                found_items[index] += 1;
                found_in_slice[index] = true;
            }
        }
    }

    for item in slices[SLICE_COUNT - 1] {
        if found_items[(*item - b'A') as usize] == (SLICE_COUNT - 1) as u8 {
            return *item;
        }
    }

    unreachable!()
}

pub fn get_priority(item: u8) -> i64 {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as i64,
        b'A'..=b'Z' => (item - b'A' + 27) as i64,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{run_part1, run_part2};

    static TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_example() {
        assert_eq!(run_part1(TEST_INPUT), 157);
    }

    #[test]
    fn part2_example() {
        assert_eq!(run_part2(TEST_INPUT), 70);
    }
}
