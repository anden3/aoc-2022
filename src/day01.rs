#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.as_bytes()
                .split(|b| *b == b'\n')
                .map(parse_ascii_number)
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut elves = input
        .split("\n\n")
        .map(|elf| {
            elf.as_bytes()
                .split(|b| *b == b'\n')
                .map(parse_ascii_number)
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    elves.sort_unstable_by(|a, b| a.cmp(b).reverse());
    elves[0..3].iter().sum()
}

pub fn parse_ascii_number(slice: &[u8]) -> i32 {
    slice
        .iter()
        .fold(0i32, |num, &byte| (num * 10) + i32::from(byte - b'0'))
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    static TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST_INPUT), 24000);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST_INPUT), 45000);
    }
}
