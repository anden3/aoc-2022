#[aoc(day6, part1, naive)]
pub fn run_part1_naive(input: &str) -> i64 {
    input
        .as_bytes()
        .windows(4)
        .take_while(|w| !is_unique(w))
        .count() as i64
        + 4
}

#[aoc(day6, part2, naive)]
pub fn run_part2_naive(input: &str) -> i64 {
    input
        .as_bytes()
        .windows(14)
        .take_while(|w| !is_unique(w))
        .count() as i64
        + 14
}

fn is_unique(slice: &[u8]) -> bool {
    let mut found_chars = [false; (b'z' + 1) as usize];

    for byte in slice {
        if found_chars[*byte as usize] {
            return false;
        }

        found_chars[*byte as usize] = true;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_naive_example() {
        assert_eq!(run_part1_naive("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(run_part1_naive("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(run_part1_naive("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(run_part1_naive("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2_naive_example() {
        assert_eq!(run_part2_naive("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(run_part2_naive("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(run_part2_naive("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(run_part2_naive("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(run_part2_naive("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
