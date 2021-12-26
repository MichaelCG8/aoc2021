use std::collections::HashSet;
use std::time;

fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/09");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}

fn part1(data: &str) -> isize {
    let map: Vec<Vec<isize>> = aoc2021::grid(data);
    let height = map.len();
    let width = map[0].len();

    let mut risk_factor = 0;
    for row in 0..height {
        let this_row = &map[row];
        for col in 0..width {
            if (row != 0) && map[row - 1][col] <= this_row[col] { continue; }
            if (row != height - 1) && map[row + 1][col] <= this_row[col] { continue; }
            if (col != 0) && (this_row[col - 1] <= this_row[col]) { continue; }
            if (col != (width - 1)) && (this_row[col + 1] <= this_row[col]) { continue; }

            risk_factor += this_row[col] + 1;
        }
    }
    risk_factor
}

fn part2(data: &str) -> usize {
    let map: Vec<Vec<isize>> = aoc2021::grid(data);
    let height = map.len();
    let width = map[0].len();

    let mut basins = Vec::new();
    for row in 0..height {
        let this_row = &map[row];
        for col in 0..width {
            if (row != 0) && map[row - 1][col] <= this_row[col] { continue; }
            if (row != height - 1) && map[row + 1][col] <= this_row[col] { continue; }
            if (col != 0) && (this_row[col - 1] <= this_row[col]) { continue; }
            if (col != (width - 1)) && (this_row[col + 1] <= this_row[col]) { continue; }

            basins.push(get_basin_size(&map, row, col, height, width));
        }
    }
    basins.sort_unstable();
    basins.reverse();
    basins[..3].iter().product()
}

fn get_basin_size(
    map: &[Vec<isize>],
    start_row: usize,
    start_col: usize,
    height: usize,
    width: usize,
) -> usize {
    let mut basin_size = 0;

    // Process this row
    let start_cols = get_basin_row_around_point(&map[start_row], start_col, width);
    basin_size += start_cols.len();

    // Process rows below
    let mut previous_cols = start_cols.clone();
    for row in (start_row + 1)..height {
        let mut these_cols = HashSet::new();
        for &c in &previous_cols {
            let new_cols = get_basin_row_around_point(&map[row], c, width);
            these_cols = these_cols.union(&new_cols).copied().collect();
        }
        previous_cols = these_cols;
        basin_size += previous_cols.len();
        if previous_cols.is_empty() { break; }
    }

    // Process rows above
    let mut previous_cols = start_cols;
    for row in (0..start_row).rev() {
        let mut these_cols = HashSet::new();
        for &c in &previous_cols {
            let new_cols = get_basin_row_around_point(&map[row], c, width);
            these_cols = these_cols.union(&new_cols).copied().collect();
        }
        previous_cols = these_cols;
        basin_size += previous_cols.len();
        if previous_cols.is_empty() { break; }
    }

    basin_size
}

fn get_basin_row_around_point(row: &[isize], point: usize, width: usize) -> HashSet<usize> {
    let mut cols = HashSet::new();
    for col in point..width {
        if row[col] == 9 { break; }
        cols.insert(col);
    }
    for col in (0..=point).rev() {
        if row[col] == 9 { break; }
        cols.insert(col);
    }
    cols
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

//     static DATA: &str = "9899965678
// 8767896789
// 9856789892
// 3987894921
// 2199943210";

//     static DATA: &str = "8765799989
// 9876987678
// 2989876589
// 1294987893
// 0123499912";

//     static DATA: &str = "0123499912
// 1294987893
// 2989876589
// 9876987678
// 8765799989";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 15);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 1134);
    }
}
