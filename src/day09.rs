use std::iter;

pub struct Step {
    pub direction: Direction,
    pub count: u8,
}

impl Step {
    pub fn new(dir: u8, count: u8) -> Step {
        Self {
            direction: dir.into(),
            count,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn offset(self, x: &mut i16, y: &mut i16) {
        match self {
            Direction::Left => *x -= 1,
            Direction::Right => *x += 1,
            Direction::Up => *y -= 1,
            Direction::Down => *y += 1,
        }
    }
}

impl From<u8> for Direction {
    fn from(shorthand: u8) -> Self {
        match shorthand {
            b'L' => Self::Left,
            b'R' => Self::Right,
            b'U' => Self::Up,
            b'D' => Self::Down,
            _ => unreachable!(),
        }
    }
}

pub fn parse_step(slice: &[u8]) -> Step {
    let (dir, count) = match slice {
        [dir, b' ', count] => (*dir, count - b'0'),
        [dir, b' ', count_a, count_b] => (*dir, (count_a - b'0') * 10 + (count_b - b'0')),
        _ => unreachable!(),
    };

    Step::new(dir, count)
}

pub fn get_steps(input: &str) -> impl Iterator<Item = Direction> + '_ {
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|s| !s.is_empty())
        .map(parse_step)
        .flat_map(|s| iter::once(s.direction).cycle().take(s.count as usize))
}

// Need an extra bit for the signedness.
const BITS_NEEDED: u32 = u8::BITS + 1;
const BIT_MASK: usize = (1 << BITS_NEEDED) - 1;

#[aoc(day9, part1)]

pub fn run_part1(input: &str) -> i64 {
    let (mut head_x, mut head_y) = (0i16, 0i16);
    let (mut tail_x, mut tail_y) = (0i16, 0i16);

    let mut positions = 1;
    let mut visited = [false; (1 << (BITS_NEEDED * 2 + 1)) - 1];
    visited[hash_pos(0, 0)] = true;

    for dir in get_steps(input) {
        dir.offset(&mut head_x, &mut head_y);

        let Some(new_tail) = follow_pos((tail_x, tail_y), (head_x, head_y)) else {
            continue;
        };

        (tail_x, tail_y) = new_tail;

        let hash = hash_pos(tail_x, tail_y);

        if !visited[hash] {
            visited[hash] = true;
            positions += 1;
        }
    }

    positions
}

#[aoc(day9, part2)]
fn run_part2(input: &str) -> i64 {
    let mut rope = [(0i16, 0i16); 10];

    let mut positions = 1;
    let mut visited = [false; (1 << (BITS_NEEDED * 2 + 1)) - 1];
    visited[hash_pos(0, 0)] = true;

    for dir in get_steps(input) {
        dir.offset(&mut rope[0].0, &mut rope[0].1);

        for i in 1..10 {
            let Some(new_pos) = follow_pos(rope[i], rope[i - 1]) else {
                break;
            };

            rope[i] = new_pos;

            if i == 9 {
                let hash = hash_pos(rope[9].0, rope[9].1);

                if !visited[hash] {
                    visited[hash] = true;
                    positions += 1;
                }
            }
        }
    }

    positions
}

#[allow(overlapping_range_endpoints)]
fn follow_pos((x, y): (i16, i16), (target_x, target_y): (i16, i16)) -> Option<(i16, i16)> {
    Some(match ((target_x - x) as i8, (target_y - y) as i8) {
        (-1..=1, -1..=1) => return None,
        (-2, 0) => (x - 1, y),
        (2, 0) => (x + 1, y),
        (0, -2) => (x, y - 1),
        (0, 2) => (x, y + 1),

        (1..=2, 1..=2) => (x + 1, y + 1),
        (1..=2, -2..=-1) => (x + 1, y - 1),
        (-2..=-1, -2..=-1) => (x - 1, y - 1),
        (-2..=-1, 1..=2) => (x - 1, y + 1),

        _ => unreachable!(),
    })
}

fn hash_pos(x: i16, y: i16) -> usize {
    ((x + 128) as usize & BIT_MASK) << BITS_NEEDED | ((y + 128) as usize & BIT_MASK)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_A: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static TEST_INPUT_B: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_example() {
        assert_eq!(run_part1(TEST_INPUT_A), 13);
    }

    #[test]
    fn part2_example_a() {
        assert_eq!(run_part2(TEST_INPUT_A), 1);
    }

    #[test]
    fn part2_example_b() {
        assert_eq!(run_part2(TEST_INPUT_B), 36);
    }
}
