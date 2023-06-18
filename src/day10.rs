#[derive(Clone, Copy)]
pub enum Opcode {
    Noop,
    AddX(i8),
}

impl Opcode {
    pub fn cycles(self) -> u8 {
        match self {
            Opcode::Noop => 1,
            Opcode::AddX(_) => 2,
        }
    }
}

fn parse_instruction(instruction: &[u8]) -> Opcode {
    match &instruction[..4] {
        b"noop" => Opcode::Noop,
        b"addx" => {
            let immediate = match &instruction[5..] {
                [b'-', num @ b'0'..=b'9'] => -((*num - b'0') as i8),
                [b'-', a @ b'0'..=b'9', b @ b'0'..=b'9'] => {
                    -(((*a - b'0') * 10 + (*b - b'0')) as i8)
                }
                [num @ b'0'..=b'9'] => (*num - b'0') as i8,
                [a @ b'0'..=b'9', b @ b'0'..=b'9'] => ((*a - b'0') * 10 + (*b - b'0')) as i8,
                _ => unreachable!(),
            };

            Opcode::AddX(immediate)
        }
        _ => unreachable!(),
    }
}

pub fn cpu_iter(input: &str) -> impl Iterator<Item = i64> + '_ {
    let mut x = 1;
    let mut v = None;

    let mut instructions = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|s| !s.is_empty())
        .map(parse_instruction);

    std::iter::from_fn(move || {
        if let Some(v) = v.take() {
            x += v as i64;
            Some(x)
        } else {
            let opcode = instructions.next()?;

            if let Opcode::AddX(imm) = opcode {
                v = Some(imm);
            }

            Some(x)
        }
    })
}

#[aoc(day10, part1)]
pub fn run_part1(input: &str) -> i64 {
    cpu_iter(input)
        .enumerate()
        .map(|(i, x)| (i as i64 + 2) * x)
        .skip(18)
        .step_by(40)
        .sum()
}

#[aoc(day10, part2)]
pub fn run_part2(input: &str) -> String {
    let mut output = [[b'.'; 40]; 6];
    let mut sprite_pos = 1;
    let mut sprite_positions = cpu_iter(input);

    for row in &mut output {
        for col in 0..40 {
            if ((col - 1)..=(col + 1)).contains(&sprite_pos) {
                row[col as usize] = b'#';
            }

            sprite_pos = sprite_positions.next().unwrap();
        }
    }

    let mut string = String::with_capacity(41 * 6);

    let mut row_buffer = [0u8; 41];
    row_buffer[40] = b'\n';

    for row in output {
        row_buffer[..40].copy_from_slice(&row);
        string.push_str(std::str::from_utf8(&row_buffer).unwrap());
    }

    string.pop();
    println!("{string}");

    string
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = {
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
    };

    #[test]
    fn part1_example() {
        assert_eq!(run_part1(TEST_INPUT), 13140);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            &run_part2(TEST_INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
