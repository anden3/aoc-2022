use std::{
    cmp::{max, min},
    ops::AddAssign,
};

use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vertex {
    x: i16,
    y: i16,
}

impl Vertex {
    #[allow(clippy::comparison_chain)]
    fn get_points_to(self, dest: Vertex) -> impl Iterator<Item = Vertex> {
        let direction = if self.x == dest.x {
            if self.y < dest.y {
                [0, 1]
            } else {
                [0, -1]
            }
        } else if self.x < dest.x {
            [1, 0]
        } else {
            [-1, 0]
        };

        VertexIter {
            current: self,
            to: dest,
            direction,
            started: false,
            finished: false,
        }
    }

    fn to_index(self, bounds: &Rect) -> usize {
        let x = self.x - bounds.left;
        let y = self.y - bounds.top as i16;

        (y as usize * bounds.width()) + x as usize
    }
}

impl AddAssign<[i16; 2]> for Vertex {
    fn add_assign(&mut self, [dx, dy]: [i16; 2]) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }
}

struct VertexIter {
    current: Vertex,
    to: Vertex,
    direction: [i16; 2],
    started: bool,
    finished: bool,
}

impl Iterator for VertexIter {
    type Item = Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            Some(self.current)
        } else if self.finished {
            None
        } else if self.current == self.to {
            self.finished = true;
            Some(self.to)
        } else {
            self.current += self.direction;
            Some(self.current)
        }
    }
}

#[derive(Debug)]
struct Polygon(Vec<Vertex>);

impl Polygon {
    fn bounds(&self) -> Rect {
        self.0.iter().copied().collect()
    }

    fn points(&self) -> impl Iterator<Item = Vertex> + '_ {
        self.0
            .windows(2)
            .flat_map(|line| line[0].get_points_to(line[1]))
    }
}

#[derive(Debug, Copy, Clone)]
struct Rect {
    left: i16,
    right: i16,
    top: u16,
    bottom: u16,
}

impl Rect {
    fn extend_to_point(&mut self, point: Vertex) {
        self.left = min(self.left, point.x);
        self.right = max(self.right, point.x);
        self.top = min(self.top, point.y as u16);
        self.bottom = max(self.bottom, point.y as u16);
    }

    fn extend_from_rect(&mut self, rect: Rect) {
        self.left = min(self.left, rect.left);
        self.right = max(self.right, rect.right);
        self.top = min(self.top, rect.top);
        self.bottom = max(self.bottom, rect.bottom);
    }

    fn width(self) -> usize {
        (self.right - self.left) as usize + 1
    }

    fn height(self) -> usize {
        (self.bottom - self.top) as usize + 1
    }

    fn area(self) -> usize {
        self.width() * self.height()
    }
}

impl FromIterator<Vertex> for Rect {
    fn from_iter<T: IntoIterator<Item = Vertex>>(iter: T) -> Self {
        let mut rect = Self::default();
        rect.extend(iter);
        rect
    }
}

impl FromIterator<Rect> for Rect {
    fn from_iter<T: IntoIterator<Item = Rect>>(iter: T) -> Self {
        let mut rect = Self::default();
        rect.extend(iter);
        rect
    }
}

impl Extend<Rect> for Rect {
    fn extend<T: IntoIterator<Item = Rect>>(&mut self, iter: T) {
        for rect in iter.into_iter() {
            self.extend_from_rect(rect);
        }
    }
}

impl Extend<Vertex> for Rect {
    fn extend<T: IntoIterator<Item = Vertex>>(&mut self, iter: T) {
        for vertex in iter.into_iter() {
            self.extend_to_point(vertex);
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            top: u16::MAX,
            bottom: 0,
            left: i16::MAX,
            right: 0,
        }
    }
}

fn parse_vertex(i: &[u8]) -> IResult<&[u8], Vertex> {
    let pair = separated_pair(parse_number, tag(b","), parse_number);
    let to_vertex = |(x, y): (i16, i16)| Vertex { x, y };

    map(pair, to_vertex)(i)
}

fn parse_arrow(i: &[u8]) -> IResult<&[u8], ()> {
    map(tag(" -> "), drop)(i)
}

fn parse_number(i: &[u8]) -> IResult<&[u8], i16> {
    map(nom::character::complete::i16, |n| n as _)(i)
}

fn parse_vertex_list(i: &[u8]) -> IResult<&[u8], Polygon> {
    let vertices = separated_list1(parse_arrow, parse_vertex);
    map(vertices, Polygon)(i)
}

fn parse_polygons(i: &[u8]) -> Vec<Polygon> {
    all_consuming(separated_list1(tag("\n"), parse_vertex_list))(i)
        .finish()
        .map(|(_, v)| v)
        .unwrap()
}

#[aoc(day14, part1)]
pub fn run_part1(input: &str) -> i64 {
    let polygons = parse_polygons(input.as_bytes());

    let mut bounds: Rect = polygons.iter().map(|p| p.bounds()).collect();
    bounds.top = 0;

    let mut board = vec![false; bounds.area()];

    for point in polygons.iter().flat_map(|p| p.points()) {
        board[point.to_index(&bounds)] = true;
    }

    let mut sand_blocks = 0;

    while simulate_sand_void(&mut board, &bounds) {
        sand_blocks += 1;
    }

    sand_blocks
}

fn simulate_sand_void(board: &mut [bool], bounds: &Rect) -> bool {
    let mut sand_idx = 500 - bounds.left as usize;

    loop {
        let down_pos = sand_idx + bounds.width();
        // println!("{sand_idx} {down_pos}");

        match (
            board.get(down_pos).copied(),
            board.get(down_pos - 1).copied(),
            board.get(down_pos + 1).copied(),
        ) {
            (Some(false), _, _) => {
                sand_idx = down_pos;
            }
            (_, Some(false), _) => {
                sand_idx = down_pos - 1;
            }
            (_, _, Some(false)) => {
                sand_idx = down_pos + 1;
            }
            (Some(true), Some(true), Some(true)) => {
                board[sand_idx] = true;
                return true;
            }
            (None, _, _) => return false,
            _ => unreachable!(),
        }
    }
}

#[aoc(day14, part2)]
pub fn run_part2(input: &str) -> i64 {
    let polygons = parse_polygons(input.as_bytes());

    let mut bounds: Rect = polygons.iter().map(|p| p.bounds()).collect();
    bounds.top = 0;
    bounds.bottom += 2;
    bounds.left -= 200;
    bounds.right += 200;

    let mut board = vec![false; bounds.area()];

    board[(500 - bounds.left) as usize] = true;

    for point in polygons.iter().flat_map(|p| p.points()) {
        board[point.to_index(&bounds)] = true;
    }

    let mut sand_blocks = 0;

    while simulate_sand_floor(&mut board, &bounds) {
        sand_blocks += 1;
    }

    sand_blocks + 1
}

fn simulate_sand_floor(board: &mut [bool], bounds: &Rect) -> bool {
    let start = (500 - bounds.left) as usize;
    let floor = bounds.width() * bounds.bottom as usize;

    let mut sand_idx = start;

    let last_pos = loop {
        let down_pos = sand_idx + bounds.width();

        if down_pos >= floor {
            board[sand_idx] = true;
            return true;
        }

        match (
            board.get(down_pos).copied(),
            board.get(down_pos - 1).copied(),
            board.get(down_pos + 1).copied(),
        ) {
            (Some(false), _, _) => {
                sand_idx = down_pos;
            }
            (_, Some(false), _) => {
                sand_idx = down_pos - 1;
            }
            (_, _, Some(false)) => {
                sand_idx = down_pos + 1;
            }
            (Some(true), Some(true), Some(true)) => {
                board[sand_idx] = true;
                break sand_idx;
            }
            _ => unreachable!(),
        }
    };

    last_pos != start
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_example() {
        assert_eq!(run_part1(TEST_INPUT), 24);
    }

    #[test]
    fn part2_example() {
        assert_eq!(run_part2(TEST_INPUT), 93);
    }
}
