use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RpsChoice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for RpsChoice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use RpsChoice::*;

        Some(match (*self, *other) {
            (Paper, Rock) | (Rock, Scissors) | (Scissors, Paper) => Ordering::Greater,
            (Rock, Paper) | (Scissors, Rock) | (Paper, Scissors) => Ordering::Less,
            _ => Ordering::Equal,
        })
    }
}

impl Ord for RpsChoice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<u8> for RpsChoice {
    fn from(byte: u8) -> Self {
        match byte {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum DesiredOutcome {
    Loss,
    Draw,
    Win,
}

impl From<u8> for DesiredOutcome {
    fn from(byte: u8) -> Self {
        match byte {
            b'X' => Self::Loss,
            b'Y' => Self::Draw,
            b'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &[u8]) -> i32 {
    input
        .split(|&b| b == b'\n')
        .map(|round| evaluate_round(round[0].into(), round[2].into()))
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[u8]) -> i32 {
    input
        .split(|&b| b == b'\n')
        .map(|round| {
            let opponent_choice = round[0].into();
            let desired_player_choice =
                get_best_choice_for_outcome(opponent_choice, round[2].into());
            evaluate_round(opponent_choice, desired_player_choice)
        })
        .sum()
}

pub fn get_best_choice_for_outcome(
    opponent_choice: RpsChoice,
    desired_outcome: DesiredOutcome,
) -> RpsChoice {
    use DesiredOutcome::*;
    use RpsChoice::*;

    match (opponent_choice, desired_outcome) {
        (opponent, DesiredOutcome::Draw) => opponent,

        (Rock, Loss) | (Paper, Win) => Scissors,
        (Rock, Win) | (Scissors, Loss) => Paper,
        (Paper, Loss) | (Scissors, Win) => Rock,
    }
}

pub fn evaluate_round(opponent_choice: RpsChoice, player_choice: RpsChoice) -> i32 {
    player_choice as i32
        + match player_choice.cmp(&opponent_choice) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    static TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST_INPUT.as_bytes()), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST_INPUT.as_bytes()), 12);
    }
}
