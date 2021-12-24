use std::time;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/06");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data, 80), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    let part2 = part1;
    println!("Part 2: {} in {:?}", part2(data, 256), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn part1(data: &str, days: isize) -> isize {
    let ages_list: Vec<usize> = aoc2021::comma_separated_to_vec::<usize>(data);
    let mut ages = [0; 9];
    for age in ages_list {
        ages[age] += 1;
    }

    for _day in 0..days {
        // Rotate the array so every timer decrements, and the re-spawning ones
        // wrap to index 8. Also, the amount that wrapped to 8 should add to index 6.
        ages.rotate_left(1);
        ages[6] += ages[8];
    }
    ages.iter().sum()
}




#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "3,4,3,1,2";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA, 18), 26);
        assert_eq!(part1(DATA, 80), 5934);
    }

    #[test]
    fn part2_matches_sample() {
        let part2 = part1;
        assert_eq!(part2(DATA, 256), 26984457539);
    }
}
