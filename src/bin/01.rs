use std::time;
use aoc2021;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/01");

    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());

    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn part1(data: &str) -> usize {
    let depths: Vec<isize> = aoc2021::lines_to_vec(data);
    depths.windows(2).filter(|w| w[1] > w[0]).count()
}


fn part2(data: &str) -> usize {
    let depths: Vec<isize> = aoc2021::lines_to_vec(data);
    depths.windows(4).filter(|w| w[3] > w[0]).count()
}


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 7);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 5);
    }
}
