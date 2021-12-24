use std::time;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/1");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    // println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn part1(data: &str) -> usize {
    todo!()
}


// fn part2(data: &str) -> usize {
//
// }


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), );
    }

    #[test]
    fn part2_matches_sample() {
        // assert_eq!(part2(DATA), );
    }
}
