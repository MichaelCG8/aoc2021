use std::fs;
use std::time;
use aoc2021;


fn main() {
    let start_total = time::Instant::now();
    let data = fs::read_to_string("inputs/01").unwrap();

    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(&data), start_part1.elapsed());

    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(&data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn part1(data: &str) -> isize {
    let depths = aoc2021::str_to_isize_vec(data);
    let mut previous = depths[0];
    let mut n_increases = 0;
    for &depth in depths[1..].iter() {
        if depth > previous {
            n_increases += 1;
        }
        previous = depth;
    }
    n_increases
}


fn part2(data: &str) -> isize {
    let depths = aoc2021::str_to_isize_vec(data);
    let mut previous : isize = depths[0..3].iter().sum();
    let mut n_increases = 0;
    for depth_group in depths[1..].windows(3) {
        let new_sum = depth_group.iter().sum();
        if new_sum > previous {
            n_increases += 1;
        }
        previous = new_sum;
    }
    n_increases
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
