#[cfg(not(test))]
const WIDTH: usize = 167;
#[cfg(test)]
const WIDTH: usize = 8;

#[cfg(not(test))]
const HEIGHT: usize = 41;
#[cfg(test)]
const HEIGHT: usize = 5;

static NEIGHBORS: [([usize; 4], usize); WIDTH * HEIGHT] = {
    const RIGHT: usize = WIDTH - 1;
    const BOTTOM: usize = HEIGHT - 1;

    let mut neighbors = [([0; 4], 0); WIDTH * HEIGHT];

    let mut row = 0;

    while row < HEIGHT {
        let mut col = 0;

        while col < WIDTH {
            let data = match (row, col) {
                (0, 0) => ([1, WIDTH, 0, 0], 2),
                (BOTTOM, 0) => ([shift(BOTTOM - 1, 0), shift(BOTTOM, 1), 0, 0], 2),
                (0, RIGHT) => ([shift(0, RIGHT - 1), shift(1, RIGHT), 0, 0], 2),
                (BOTTOM, RIGHT) => (
                    [shift(BOTTOM - 1, RIGHT), shift(BOTTOM, RIGHT - 1), 0, 0],
                    2,
                ),

                (0, y) => ([y - 1, y + 1, WIDTH + y, 0], 3),
                (BOTTOM, y) => (
                    [
                        shift(BOTTOM, y - 1),
                        shift(BOTTOM, y + 1),
                        shift(BOTTOM - 1, y),
                        0,
                    ],
                    3,
                ),
                (x, 0) => ([shift(x - 1, 0), shift(x + 1, 0), shift(x, 1), 0], 3),
                (x, RIGHT) => (
                    [
                        shift(x - 1, RIGHT),
                        shift(x + 1, RIGHT),
                        shift(x, RIGHT - 1),
                        0,
                    ],
                    3,
                ),

                (x, y) => (
                    [
                        shift(x - 1, y),
                        shift(x + 1, y),
                        shift(x, y - 1),
                        shift(x, y + 1),
                    ],
                    4,
                ),
            };

            neighbors[shift(row, col)] = data;
            col += 1;
        }

        row += 1;
    }

    neighbors
};

static NEIGHBOR_SLICES: [&[usize]; WIDTH * HEIGHT] = {
    let mut neighbors = [&[] as &[usize]; WIDTH * HEIGHT];

    let mut row = 0;

    while row < HEIGHT {
        let mut col = 0;

        while col < WIDTH {
            let index = shift(row, col);
            let (arr, len) = &NEIGHBORS[index];
            let slice = unsafe { std::slice::from_raw_parts(arr.as_ptr(), *len) };
            neighbors[shift(row, col)] = slice;
            col += 1;
        }

        row += 1;
    }

    neighbors
};

const fn shift(row: usize, col: usize) -> usize {
    row * WIDTH + col
}

#[aoc(day12, part1)]
pub fn run_part1(input: &str) -> i64 {
    let input = input.as_bytes();
    let mut map = [0u8; WIDTH * HEIGHT];

    for row in 0..HEIGHT {
        let start = row * WIDTH;
        map[start..(start + WIDTH)].copy_from_slice(&input[(start + row)..(start + row + WIDTH)]);
    }

    let starting_pos = map.iter().position(|&b| b == b'S').unwrap();
    let goal_pos = map.iter().position(|&b| b == b'E').unwrap();

    map[starting_pos] = b'a';
    map[goal_pos] = b'z';

    let path = pathfinding::prelude::bfs(
        &starting_pos,
        |&p| {
            NEIGHBOR_SLICES[p]
                .iter()
                .filter(move |&&n| (0..=map[p] + 1).contains(&map[n]))
                .copied()
        },
        |&p| p == goal_pos,
    )
    .unwrap();

    path.len() as i64 - 1
}

#[aoc(day12, part2)]
pub fn run_part2(input: &str) -> i64 {
    let input = input.as_bytes();
    let mut map = [0u8; WIDTH * HEIGHT];

    for row in 0..HEIGHT {
        let start = row * WIDTH;
        map[start..(start + WIDTH)].copy_from_slice(&input[(start + row)..(start + row + WIDTH)]);
    }

    let mut starting_positions = Vec::new();

    for (i, val) in map
        .iter_mut()
        .enumerate()
        .filter(|(_i, el)| matches!(el, b'a' | b'S'))
    {
        *val = b'a';
        starting_positions.push(i);
    }

    let goal_pos = map.iter().position(|&b| b == b'E').unwrap();
    map[goal_pos] = b'z';

    let (paths, optimal) = pathfinding::prelude::dijkstra_partial(
        &goal_pos,
        |&p| {
            NEIGHBOR_SLICES[p]
                .iter()
                .filter(move |&&n| ((map[p] - 1)..).contains(&map[n]))
                .copied()
                .map(|n| (n, 1))
        },
        |p| starting_positions.contains(p),
    );

    paths[&optimal.unwrap()].1 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_example() {
        assert_eq!(run_part1(TEST_INPUT), 31);
    }

    #[test]
    fn part2_example() {
        assert_eq!(run_part2(TEST_INPUT), 29);
    }
}
