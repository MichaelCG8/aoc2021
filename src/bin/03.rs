use std::fs;
use std::time;


fn main() {
    let start_total = time::Instant::now();
    let data = fs::read_to_string("inputs/03").unwrap();
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(&data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(&data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn part1(data: &str) -> isize {
    let mut lines = data.lines();
    let first_line = lines.next().unwrap();
    let mut zeros: Vec<isize> = first_line.trim().chars().map(|c| match c { '0' => 1, _ => 0 }).collect();
    let mut ones: Vec<isize> = zeros.iter().map(|z| match z { 1 => 0, _ => 1 }).collect();

    for line in lines {
        for (i, c) in line.trim().chars().enumerate() {
            match c {
                '0' => zeros[i]+=1,
                _ => ones[i]+=1,
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for (&z, o) in zeros.iter().zip(ones) {
        gamma <<= 1;
        epsilon <<= 1;
        if o > z {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    gamma * epsilon
}


fn part2(data: &str) -> isize {
    let mut lines: Vec<&str> = data.lines().collect();
    let line_length = lines[0].trim().len();

    for i in 0..line_length {
        let zeros = lines.iter().filter(|line| line[i..i+1] == '0'.to_string()).count();
        let ones = lines.iter().filter(|line| line[i..i+1] == '1'.to_string()).count();
        let most_common = if zeros > ones {'0'} else {'1'};
        lines = lines.iter().filter(|line| line[i..i+1] == most_common.to_string()).map(|&s| s).collect();
        if lines.len() == 1 { break; }
    }
    let ogr = isize::from_str_radix(lines[0], 2).unwrap();

    let mut lines: Vec<&str> = data.lines().collect();
    for i in 0..line_length {
        let zeros = lines.iter().filter(|line| line[i..i+1] == '0'.to_string()).count();
        let ones = lines.iter().filter(|line| line[i..i+1] == '1'.to_string()).count();
        let most_common = if zeros > ones {'0'} else {'1'};
        lines = lines.iter().filter(|line| line[i..i+1] != most_common.to_string()).map(|&s| s).collect();
        if lines.len() == 1 { break; }
    }
    let csr = isize::from_str_radix(lines[0], 2).unwrap();

    ogr * csr
}


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 198);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 230);
    }
}
