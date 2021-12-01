use std::fs;
use aoc2021;


fn main() {
    let data = fs::read_to_string("inputs/01").unwrap();
    println!("Part 1: {}", count_increases(&data));
    println!("Part 2: {}", count_increases_window(&data));
}


fn count_increases(data: &str) -> isize {
    let depths = aoc2021::str_to_isize_vec(data);
    let mut previous = depths[0];
    let mut n_increases = 0;
    for depth in depths {
        if depth > previous {
            n_increases += 1;
        }
        previous = depth;
    }
    n_increases
}


fn count_increases_window(data: &str) -> isize {
    let depths = aoc2021::str_to_isize_vec(data);
    let mut previous : isize = depths[0..3].iter().sum();
    let mut n_increases = 0;
    for index in 1..=(depths.len())-3 {
        let new_sum = depths[index..index+3].iter().sum();
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
    fn part1() {
        assert_eq!(count_increases(DATA), 7);
    }

    #[test]
    fn part2() {
        assert_eq!(count_increases_window(DATA), 5);
    }
}
