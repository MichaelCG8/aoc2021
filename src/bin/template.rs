use std::fs;
use std::time;
use aoc2021;


fn main() {
    let start_total = time::Instant::now();
    let data = fs::read_to_string("inputs/0").unwrap();
    let start_part1 = time::Instant::now();
    // println!("Part 1: {} in {:?}", part1(&data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    // println!("Part 2: {} in {:?}", part2(&data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
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
        // assert_eq!(part1(DATA), 7);
    }

    #[test]
    fn part2_matches_sample() {
        // assert_eq!(part2(DATA), 5);
    }
}
