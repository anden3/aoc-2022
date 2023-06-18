use std::cmp::Ordering;

use tinyvec::ArrayVec;

#[derive(Debug, Clone)]
pub enum Value {
    Int(u8),
    List(ArrayVec<[u16; 16]>),
}

pub struct Values {
    values: Vec<Value>,
}

impl Values {
    pub fn add_value(&mut self, value: Value) -> u16 {
        let len = self.values.len();
        self.values.push(value);
        len as u16
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn cmp_indices(&self, left: u16, right: u16) -> Ordering {
        self.cmp(&self.values[left as usize], &self.values[right as usize])
    }

    pub fn cmp(&self, left: &Value, right: &Value) -> Ordering {
        match (left, right) {
            (Value::Int(l), Value::Int(r)) => l.cmp(r),
            (Value::List(l), Value::List(r)) => self
                .get_children(l)
                .cmp_by(self.get_children(r), |a, b| self.cmp(a, b)),
            (Value::Int(_l), Value::List(r)) => {
                if r.is_empty() {
                    Ordering::Greater
                } else {
                    match self.cmp(left, &self.values[r[0] as usize]) {
                        Ordering::Equal => Ordering::Less,
                        ord => ord,
                    }
                }
            }
            (Value::List(l), Value::Int(_r)) => {
                if l.is_empty() {
                    Ordering::Less
                } else {
                    match self.cmp(&self.values[l[0] as usize], right) {
                        Ordering::Equal => Ordering::Greater,
                        ord => ord,
                    }
                }
            }
        }
    }

    pub fn get_children<'a>(&'a self, indices: &'a [u16]) -> impl Iterator<Item = &Value> + 'a {
        indices.iter().map(|i| &self.values[*i as usize])
    }
}

#[aoc(day13, part1)]
pub fn run_part1(input: &str) -> i64 {
    let mut values = Values {
        values: Vec::with_capacity(8192),
    };

    get_pairs(input)
        .map(|(left, right)| {
            let left = process_packet(left.as_bytes(), &mut values);
            let right = process_packet(right.as_bytes(), &mut values);

            values.cmp_indices(left, right)
        })
        .enumerate()
        .filter_map(|(i, cmp)| cmp.is_lt().then_some(i + 1))
        .sum::<usize>() as i64
}

#[aoc(day13, part2)]
pub fn run_part2(input: &str) -> i64 {
    let mut values = Values {
        values: Vec::with_capacity(8192),
    };

    let mut packet_indices = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|s| !s.is_empty())
        .map(|packet| process_packet(packet, &mut values))
        .collect::<Vec<_>>();

    let decoder_keys = [b"[[2]]", b"[[6]]"].map(|divider| process_packet(divider, &mut values));
    packet_indices.extend(&decoder_keys);

    packet_indices.sort_unstable_by(|&left, &right| values.cmp_indices(left, right));

    packet_indices
        .into_iter()
        .enumerate()
        .filter_map(|(i, k)| decoder_keys.contains(&k).then_some(i + 1))
        .product::<usize>() as i64
}

fn get_pairs(input: &str) -> impl Iterator<Item = (&str, &str)> + '_ {
    input
        .split("\n\n")
        .map(|pair| pair.trim().split_once('\n').unwrap())
}

fn process_packet(packet: &[u8], values: &mut Values) -> u16 {
    if packet.is_empty() {
        return values.add_value(Value::List(ArrayVec::new()));
    } else if packet[0] != b'[' {
        return values.add_value(Value::Int(parse_int(packet)));
    }

    let mut open_brackets: u8 = 0;
    let mut list = ArrayVec::new();
    let mut item_start = 0;

    for (i, byte) in packet.iter().enumerate() {
        match byte {
            b'[' => open_brackets += 1,
            b']' => open_brackets -= 1,
            b',' if open_brackets == 1 => {
                list.push(process_packet(&packet[item_start + 1..i], values));
                item_start = i;
            }
            _ => (),
        }
    }
    list.push(process_packet(
        &packet[item_start + 1..packet.len() - 1],
        values,
    ));

    values.add_value(Value::List(list))
}

pub fn parse_int(slice: &[u8]) -> u8 {
    match slice {
        [x] => x - b'0',
        [x, y] => (x - b'0') * 10 + (y - b'0'),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_example() {
        assert_eq!(run_part1(TEST_INPUT), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(run_part2(TEST_INPUT), 140);
    }
}
