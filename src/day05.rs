use std::fmt::Display;

#[derive(Debug)]
pub struct Stacks {
    stacks: [[u8; 64]; 16],
    stack_sizes: [u8; 16],
    stack_count: u8,
}

impl Stacks {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    fn stack(&self, index: u8) -> *const u8 {
        (&self.stacks[index as usize]) as *const u8
    }

    fn stack_mut(&mut self, index: u8) -> *mut u8 {
        (&mut self.stacks[index as usize]) as *mut u8
    }

    fn stack_end(&self, index: u8) -> *const u8 {
        unsafe { self.stack(index).add(self.stack_size(index) as _) }
    }

    fn stack_end_mut(&mut self, index: u8) -> *mut u8 {
        unsafe { self.stack_mut(index).add(self.stack_size(index) as _) }
    }

    fn stack_size(&self, stack: u8) -> usize {
        self.stack_sizes[stack as usize] as usize
    }

    fn stack_size_mut(&mut self, stack: u8) -> &mut u8 {
        &mut self.stack_sizes[stack as usize]
    }

    pub fn push_crate(&mut self, index: u8, new_crate: u8) {
        self.stack_sizes[index as usize] -= 1;

        unsafe {
            self.stack_end_mut(index).write(new_crate);
        }
    }

    pub fn slide_stacks_back(&mut self) {
        for stack_idx in 0..self.stack_count {
            let src_start = self.stack_size(stack_idx);
            let count = 64 - src_start;

            self.stacks[stack_idx as usize].copy_within(src_start..(src_start + count), 0);

            *self.stack_size_mut(stack_idx) = count as u8;
        }
    }

    pub fn move_crates_p1(&mut self, count: usize, from: u8, to: u8) {
        let (from_len, to_len) = (self.stack_size(from), self.stack_size(to));

        let mut temp_buffer = [0u8; 64];

        for (i, b) in self.stacks[from as usize][(from_len - count)..from_len]
            .iter()
            .rev()
            .enumerate()
        {
            temp_buffer[i] = *b;
        }

        self.stacks[to as usize][to_len..(to_len + count)].copy_from_slice(&temp_buffer[..count]);

        *self.stack_size_mut(from) -= count as u8;
        *self.stack_size_mut(to) += count as u8;
    }

    pub fn move_crates_p2(&mut self, count: usize, from: u8, to: u8) {
        let (from_len, to_len) = (self.stack_size(from), self.stack_size(to));

        let mut temp_buffer = [0u8; 64];

        temp_buffer[..count]
            .copy_from_slice(&self.stacks[from as usize][(from_len - count)..from_len]);

        self.stacks[to as usize][to_len..(to_len + count)].copy_from_slice(&temp_buffer[..count]);

        *self.stack_size_mut(from) -= count as u8;
        *self.stack_size_mut(to) += count as u8;
    }

    pub fn get_top_crates(&self) -> impl Iterator<Item = u8> + '_ {
        (0..self.stack_count)
            .map(|stack_idx| unsafe { self.stack_end(stack_idx).offset(-1).read() })
    }
}

impl Default for Stacks {
    fn default() -> Self {
        Self {
            stacks: [[0u8; 64]; 16],
            stack_sizes: [64u8; 16],
            stack_count: 0u8,
        }
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, stack) in self
            .stacks
            .iter()
            .take(self.stack_count as usize)
            .enumerate()
        {
            f.write_fmt(format_args!(
                "{i} {}\n",
                std::str::from_utf8(&stack[..(self.stack_sizes[i] as usize)]).unwrap()
            ))?;
        }

        Ok(())
    }
}

#[aoc(day5, part1)]
pub fn run_part1(input: &str) -> String {
    let input = input.as_bytes();
    let (mut stacks, offset, double_digit_stacks) = read_crates(input);

    stacks.slide_stacks_back();

    let stacks = if double_digit_stacks {
        read_instructions_double_stacks(&input[offset..], stacks)
    } else {
        read_instructions_single_stacks(&input[offset..], stacks)
    };

    let mut out_buffer = [0u8; 64];

    for (i, top_crate) in stacks.get_top_crates().enumerate() {
        out_buffer[i] = top_crate;
    }

    unsafe { std::str::from_utf8_unchecked(&out_buffer[..(stacks.stack_count as usize)]) }
        .to_owned()
}

#[aoc(day5, part2)]
pub fn run_part2(input: &str) -> String {
    let input = input.as_bytes();
    let (mut stacks, offset, double_digit_stacks) = read_crates(input);

    stacks.slide_stacks_back();

    let stacks = if double_digit_stacks {
        read_instructions_double_stacks(&input[offset..], stacks)
    } else {
        read_instructions_single_stacks(&input[offset..], stacks)
    };

    let mut out_buffer = [0u8; 64];

    for (i, top_crate) in stacks.get_top_crates().enumerate() {
        out_buffer[i] = top_crate;
    }

    unsafe { std::str::from_utf8_unchecked(&out_buffer[..(stacks.stack_count as usize)]) }
        .to_owned()
}

fn read_crates(input: &[u8]) -> (Stacks, usize, bool) {
    let mut offset = 0;
    let mut current_index = 0;
    let mut stacks = Stacks::new();

    loop {
        let chunk = &input[offset..(offset + 4)];

        let reached_line_end = match chunk {
            [b'[', stacked_crate @ b'A'..=b'Z', b']', last @ (b' ' | b'\n')] => {
                stacks.push_crate(current_index, *stacked_crate);
                *last == b'\n'
            }
            [b' ', b' ', b' ', last @ (b' ' | b'\n')] => *last == b'\n',
            [b' ', b'1'..=b'9', _, _] => {
                break;
            }
            _ => unreachable!(),
        };

        offset += 4;

        if reached_line_end {
            stacks.stack_count = current_index + 1;
            current_index = 0;
        } else {
            current_index += 1;
        }
    }

    offset += &input[offset..].iter().position(|&b| b == b'\n').unwrap();

    let double_digit_stacks = !matches!(
        &input[(offset - 3)..=offset],
        [b' ', b'0'..=b'9', b' ', b'\n']
    );

    (stacks, offset + 2, double_digit_stacks)
}

fn read_instructions_single_stacks(input: &[u8], mut stacks: Stacks) -> Stacks {
    for line in input.split(|&b| b == b'\n').filter(|l| !l.is_empty()) {
        const TO_LEN: usize = " TO ".len();

        let (count, offset) = get_crate_count(line);

        let from = line[offset] - b'0' - 1;
        let to = line[offset + 1 + TO_LEN] - b'0' - 1;

        stacks.move_crates_p2(count, from, to);
    }

    stacks
}

fn read_instructions_double_stacks(input: &[u8], mut stacks: Stacks) -> Stacks {
    for line in input.split(|&b| b == b'\n').filter(|l| !l.is_empty()) {
        const TO_LEN: usize = " TO ".len();

        let (count, offset) = get_crate_count(line);
        let (from, width) = parse_one_or_two_digit_number(&line[offset..(offset + 2)]);

        let offset = offset + width + TO_LEN;

        let (to, _) = parse_one_or_two_digit_number(&line[offset..(offset + 2)]);

        stacks.move_crates_p2(count, from, to);
    }

    stacks
}

fn get_crate_count(input: &[u8]) -> (usize, usize) {
    const MOVE_LEN: usize = b"move ".len();
    const FROM_LEN: usize = b" from ".len();

    let (num, width) = parse_one_or_two_digit_number(&input[MOVE_LEN..(MOVE_LEN + 2)]);
    (num as usize, width + MOVE_LEN + FROM_LEN)
}

fn parse_one_or_two_digit_number(input: &[u8]) -> (u8, usize) {
    match &input[..2] {
        [x @ b'0'..=b'9', b' ' | b'\n'] => (*x - b'0', 1),
        [x @ b'0'..=b'9', y @ b'0'..=b'9'] => ((*x - b'0') * 10 + (*y - b'0'), 2),
        _ => unreachable!(),
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_example() {
        assert_eq!(&run_part1(TEST_INPUT), "CMZ");
    }

    #[test]
    fn part2_example() {
        assert_eq!(run_part2(TEST_INPUT), 4);
    }
} */
