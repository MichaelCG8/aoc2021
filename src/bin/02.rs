use std::fs;
use std::time;
use aoc2021;


fn main() {
    let start_total = time::Instant::now();
    let data = fs::read_to_string("inputs/02").unwrap();

    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(&data), start_part1.elapsed());

    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(&data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn part1(data: &str) -> isize {
    let data = aoc2021::str_to_str_isize_vec(&data);
    let forward: isize = data.iter().filter(|(direction, _num)| direction == &"forward").map(|(_direction, num)| num).sum();
    let up: isize = data.iter().filter(|(direction, _num)| direction == &"up").map(|(_direction, num)| num).sum();
    let down: isize = data.iter().filter(|(direction, _num)| direction == &"down").map(|(_direction, num)| num).sum();

    forward * (down - up)
}


fn part2(data: &str) -> isize {
    let data = aoc2021::str_to_str_isize_vec(&data);
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (command, value) in data {
        match command {
            "up" => aim -= value,
            "down" => aim += value,
            "forward" => { horizontal += value; depth += aim * value; },
            _ => panic!("Got an unrecognised command.")
        }
    }
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 150);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 900);
    }
}
