use std::time;
use regex::Regex;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/21");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    // println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn part1(data: &str) -> usize {
    let re = Regex::new(r"^Player \d starting position: (?P<pos>\d+)$").unwrap();
    let lines = data.lines().collect::<Vec<&str>>();
    let mut pos1 = re.captures(lines[0]).unwrap().name("pos").unwrap().as_str().parse::<usize>().unwrap();
    let mut pos2 = re.captures(lines[1]).unwrap().name("pos").unwrap().as_str().parse::<usize>().unwrap();

    let mut score1 = 0;
    let mut score2 = 0;
    let mut die = (1..=100).cycle();
    let mut rolls = 0;
    loop {
        let roll: usize = (&mut die).take(3).sum();
        rolls += 3;
        pos1 += roll;
        if pos1 > 10 {
            pos1 %= 10;
            if pos1 == 0 {
                pos1 = 10;
            }
        }
        score1 += pos1;
        if score1 >= 1000 {
            return score2 * rolls;
        }
        let roll: usize = (&mut die).take(3).sum();
        rolls += 3;
        pos2 += roll;
        if pos2 > 10 {
            pos2 %= 10;
            if pos2 == 0 {
                pos2 = 10;
            }
        }
        score2 += pos2;
        if score2 >= 1000 {
            return score1 * rolls;
        }
    }
}

// fn part2(data: &str) -> usize {
//
// }


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 739785);
    }

    #[test]
    fn part2_matches_sample() {
        // assert_eq!(part2(DATA), );
    }
}
