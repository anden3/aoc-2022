#[aoc(day4, part1)]
pub fn run_part1(input: &str) -> i64 {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|s| !s.is_empty())
        .filter(|pair| {
            let comma = pair.iter().position(|&b| b == b',').unwrap();
            let [a_start, a_end] = parse_pair(&pair[..comma]);
            let [b_start, b_end] = parse_pair(&pair[(comma + 1)..]);

            use std::cmp::Ordering::*;

            matches!(
                (a_start.cmp(&b_start), a_end.cmp(&b_end)),
                (Less | Equal, Equal | Greater) | (Equal | Greater, Less | Equal)
            )
        })
        .count() as i64
}

#[aoc(day4, part2)]
pub fn run_part2(input: &str) -> i64 {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|s| !s.is_empty())
        .filter(|pair| {
            let comma = pair.iter().position(|&b| b == b',').unwrap();
            let [a_start, a_end] = parse_pair(&pair[..comma]);
            let [b_start, b_end] = parse_pair(&pair[(comma + 1)..]);

            !((a_start < b_start && a_end < b_start) || (b_start < a_start && b_end < a_start))
        })
        .count() as i64
}

pub fn parse_pair(slice: &[u8]) -> [u8; 2] {
    match slice {
        [x, b'-', y] => [x - b'0', y - b'0'],
        [x1, x2, b'-', y] => [(x1 - b'0') * 10 + (x2 - b'0'), y - b'0'],
        [x, b'-', y1, y2] => [x - b'0', (y1 - b'0') * 10 + (y2 - b'0')],
        [x1, x2, b'-', y1, y2] => [
            (x1 - b'0') * 10 + (x2 - b'0'),
            (y1 - b'0') * 10 + (y2 - b'0'),
        ],

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_example() {
        assert_eq!(run_part1(TEST_INPUT), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(run_part2(TEST_INPUT), 4);
    }
}
