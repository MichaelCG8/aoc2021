use std::fs;
use aoc2021;


fn main() {
    let data = fs::read_to_string("inputs/0").unwrap();
    // println!("Part 1: {}", part1(&data));
    // println!("Part 2: {}", part2(&data));
}


// fn part1(data: &str) -> isize {
//
// }


// fn part2(data: &str) -> isize {
//
// }


#[cfg(test)]
mod tests {
    use super::*;
    // static DATA : &str = "199
// 260
// 263";

    #[test]
    fn part1_matches_sample() {
        // assert_eq!(count_increases(DATA), 7);
    }

    #[test]
    fn part2_matches_sample() {
        // assert_eq!(count_increases_window(DATA), 5);
    }
}
