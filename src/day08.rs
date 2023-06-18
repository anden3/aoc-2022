use std::cmp::max;

#[cfg(not(test))]
const ROW_LEN: usize = 99;
#[cfg(test)]
const ROW_LEN: usize = 5;

#[aoc(day8, part1)]
pub fn run_part1(input: &str) -> i64 {
    let mut marked = [false; ROW_LEN * ROW_LEN];
    let mut rows = input.as_bytes().split(|&b| b == b'\n').enumerate();

    let (_, first_row) = rows.by_ref().next().unwrap();

    let mut buffer = [0u8; ROW_LEN];
    buffer[..ROW_LEN].copy_from_slice(first_row);
    let mut top_visibility = buffer;

    // Top and bottom rows.
    let mut total_visible = ROW_LEN * 2;

    for (row_idx, row) in rows.by_ref().take(ROW_LEN - 2) {
        // Left and right trees.
        total_visible += 2;

        let mut left_visibility = row[0];

        if row[0] > top_visibility[0] {
            top_visibility[0] = row[0];
        }

        for (i, tree) in row.iter().copied().enumerate().skip(1).take(ROW_LEN - 2) {
            if tree > top_visibility[i] || tree > left_visibility {
                total_visible += 1;
                marked[(row_idx * ROW_LEN) + i] = true;

                top_visibility[i] = max(top_visibility[i], tree);
                left_visibility = max(left_visibility, tree);
            }
        }
    }

    let (_, last_row) = rows.next().unwrap();
    top_visibility[..ROW_LEN].copy_from_slice(last_row);
    let mut bottom_visibility = top_visibility;

    for (row_idx, row) in input
        .as_bytes()
        .split(|&b| b == b'\n')
        .rev()
        .enumerate()
        .skip(1)
        .take(ROW_LEN - 2)
    {
        let row_idx = ROW_LEN - row_idx - 1;
        let index = ROW_LEN - 1;
        let mut right_visibility = row[index];

        if row[index] > bottom_visibility[index] {
            bottom_visibility[index] = row[index];
        }

        for (i, tree) in row
            .iter()
            .copied()
            .enumerate()
            .rev()
            .skip(1)
            .take(ROW_LEN - 2)
        {
            if tree > bottom_visibility[i] || tree > right_visibility {
                if !marked[(row_idx * ROW_LEN) + i] {
                    total_visible += 1;
                }

                bottom_visibility[i] = max(bottom_visibility[i], tree);
                right_visibility = max(right_visibility, tree);
            }
        }
    }

    total_visible as i64
}

fn scenic_score(rows: &[&[u8]], (start_x, start_y): (i64, i64)) -> i64 {
    const OFFSETS: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let height = rows[start_y as usize][start_x as usize];

    OFFSETS
        .into_iter()
        .map(|(dx, dy)| {
            let (mut x, mut y) = (start_x + dx, start_y + dy);

            while let Some(&tree) = rows.get(y as usize).and_then(|row| row.get(x as usize)) {
                (x, y) = (x + dx, y + dy);

                if tree >= height {
                    break;
                }
            }

            (x.abs_diff(start_x) + y.abs_diff(start_y) - 1) as i64
        })
        .product()
}

#[aoc(day8, part2)]
fn run_part2(input: &str) -> i64 {
    let row_vec = input.as_bytes().split(|&b| b == b'\n').collect::<Vec<_>>();
    let rows = &row_vec;

    (0..rows.len())
        .flat_map(|y| (0..rows[y].len()).map(move |x| scenic_score(rows, (x as i64, y as i64))))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_example() {
        assert_eq!(run_part1(TEST_INPUT), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(run_part2(TEST_INPUT), 8);
    }
}
