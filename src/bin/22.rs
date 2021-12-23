use std::time;
use aoc2021;
use regex::Regex;

// 597600, too low

fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/22");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    // println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn part1(data: &str) -> usize {
    let mut engine = [[[false; 101]; 101]; 101];
    let re = Regex::new(r"^(?P<op>on|off) x=(?P<x_lo>-?\d+)..(?P<x_hi>-?\d+),y=(?P<y_lo>-?\d+)..(?P<y_hi>-?\d+),z=(?P<z_lo>-?\d+)..(?P<z_hi>-?\d+)$").unwrap();

    for line in data.lines() {
        let setting = re.captures(line).unwrap().name("op").unwrap().as_str() == "on";
        let mut x_lo = re.captures(line).unwrap().name("x_lo").unwrap().as_str().parse::<isize>().unwrap();
        if x_lo < -50 { x_lo = -50; }
        else if x_lo > 50 { continue; }
        let x_lo = (x_lo + 50) as usize;
        let mut x_hi = re.captures(line).unwrap().name("x_hi").unwrap().as_str().parse::<isize>().unwrap();
        if x_hi < -50 { continue; }
        else if x_hi > 50 { x_hi = 50; }
        let x_hi = (x_hi + 50) as usize;
        let mut y_lo = re.captures(line).unwrap().name("y_lo").unwrap().as_str().parse::<isize>().unwrap();
        if y_lo < -50 { y_lo = -50; }
        else if y_lo > 50 { continue; }
        let y_lo = (y_lo + 50) as usize;
        let mut y_hi = re.captures(line).unwrap().name("y_hi").unwrap().as_str().parse::<isize>().unwrap();
        if y_hi < -50 { continue; }
        else if y_hi > 50 { y_hi = 50; }
        let y_hi = (y_hi + 50) as usize;
        let mut z_lo = re.captures(line).unwrap().name("z_lo").unwrap().as_str().parse::<isize>().unwrap();
        if z_lo < -50 { z_lo = -50; }
        else if z_lo > 50 { continue; }
        let z_lo = (z_lo + 50) as usize;
        let mut z_hi = re.captures(line).unwrap().name("z_hi").unwrap().as_str().parse::<isize>().unwrap();
        if z_hi < -50 { continue; }
        else if z_hi > 50 { z_hi = 50; }
        let z_hi = (z_hi + 50) as usize;

        for x in x_lo..=x_hi {
            let plane = engine.get_mut(x).unwrap();
            for y in y_lo..=y_hi {
                let line = plane.get_mut(y).unwrap();
                for z in z_lo..=z_hi {
                    *line.get_mut(z).unwrap() = setting;
                }
            }
        }
    }

    engine
        .iter()
        .map(
            |plane|
            plane
                .iter()
                .map(
                    |line|
                    line
                        .iter()
                        .filter(|&&el| el)
                        .count()
                )
                .sum::<usize>()
        )
        .sum()
}


// fn part2(data: &str) -> usize {
//
// }


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1("on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"), 39);
        assert_eq!(part1(DATA), 590784);
    }

    #[test]
    fn part2_matches_sample() {
        // assert_eq!(part2(DATA), );
    }
}
