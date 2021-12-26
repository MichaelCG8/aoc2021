use aoc2021::math::tri_f64;
use std::time;

fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/07");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}

fn part1(data: &str) -> isize {
    let mut locations: Vec<isize> = aoc2021::comma_separated_to_vec(data);
    locations.sort_unstable();
    let median = locations[locations.len() / 2];
    locations.iter().map(|&l| (l - median).abs()).sum()
}

fn part2(data: &str) -> f64 {
    let locations: Vec<f64> = aoc2021::comma_separated_to_vec(data);
    let mean = locations.iter().sum::<f64>() / locations.len() as f64;

    let low = mean.floor();
    let at_low = locations.iter().map(|&l| tri_f64((l - low).abs())).sum();

    let high = mean.ceil();
    let at_high = locations.iter().map(|&l| tri_f64((l - high).abs())).sum();

    if at_low < at_high { at_low } else { at_high }
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 37);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 168.0);
    }
}
