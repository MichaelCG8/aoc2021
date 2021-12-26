use std::time;

fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/25");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    // println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}

#[derive(PartialEq)]
enum Cucumbers {
    EMPTY,
    EAST,
    SOUTH,
}

impl aoc2021::EnumFromInt for Cucumbers {
    fn from_int(index: usize) -> Self {
        // TODO: Replace with derive macro.
        // TODO: Do something like Self.iter().find_where(|v| v as usize == index)
        match index {
            0 => Cucumbers::EMPTY,
            1 => Cucumbers::EAST,
            2 => Cucumbers::SOUTH,
            _ => panic!(),
        }
        // for variant in Self {
        //     if variant as usize == index {
        //         variant
        //     }
        // }
        // panic!();
    }
}

fn part1(data: &str) -> usize {
    let mut grid = aoc2021::Grid::enum_grid(data, &['.', '>', 'v']);

    let mut steps = 0;

    loop {
        let mut east_moves = Vec::new();
        for row_idx in 0..grid.height() {
            let row = &grid[row_idx];
            for col_idx in 0..grid.width() {
                let mut next_col_idx = col_idx + 1;
                if next_col_idx == grid.width() { next_col_idx = 0; }
                if row[col_idx] == Cucumbers::EAST && row[next_col_idx] == Cucumbers::EMPTY {
                    east_moves.push((row_idx, col_idx, next_col_idx));
                }
            }
        }
        for &(row, col, next) in east_moves.iter() {
            grid[row][col] = Cucumbers::EMPTY;
            grid[row][next] = Cucumbers::EAST;
        }

        let mut south_moves = Vec::new();
        for row_idx in 0..grid.height() {
            let mut next_row_idx = row_idx + 1;
            if next_row_idx == grid.height() { next_row_idx = 0; }
            let row = &grid[row_idx];
            let next_row = &grid[next_row_idx];
            for col_idx in 0..grid.width() {
                if row[col_idx] == Cucumbers::SOUTH && next_row[col_idx] == Cucumbers::EMPTY {
                    south_moves.push((row_idx, col_idx, next_row_idx));
                }
            }
        }
        for &(row, col, next) in south_moves.iter() {
            grid[row][col] = Cucumbers::EMPTY;
            grid[next][col] = Cucumbers::SOUTH;
        }

        steps += 1;

        if east_moves.is_empty() && south_moves.is_empty() { break; }
    }

    steps
}

// fn part2(data: &str) -> usize {
//
// }

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = "";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), );
    }

    #[test]
    fn part2_matches_sample() {
        // assert_eq!(part2(DATA), );
    }
}
