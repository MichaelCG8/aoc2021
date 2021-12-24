use std::collections::HashSet;
use std::time;
use aoc2021::math::{tri_isize, tri_base_isize, tri_inv};

use regex::Regex;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/17");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn part1(data: &str) -> isize {
    let re = Regex::new(r"^target area: x=(?P<x0>-?\d+)..(?P<x1>-?\d+), y=(?P<y0>-?\d+)..(?P<y1>-?\d+)\s*$").unwrap();
    let parsed = re.captures(data).unwrap();
    let x0 = parsed.name("x0").unwrap().as_str().parse::<isize>().unwrap();
    let x1 = parsed.name("x1").unwrap().as_str().parse::<isize>().unwrap();
    let y0 = parsed.name("y0").unwrap().as_str().parse::<isize>().unwrap();
    let y1 = parsed.name("y1").unwrap().as_str().parse::<isize>().unwrap();

    let v_max = x1;
    let v_min = tri_inv(x0 as f64).ceil() as isize;

    let n_min = 1;
    let n_max = 10000; // v_min;  // TODO: See if I can find the upper bound.

    let mut largest_peak = None;
    for steps in n_min..=n_max {
        for vx in v_min..=v_max {
            let x = if vx > steps { tri_base_isize(vx, steps) } else { tri_isize(vx) };
            if (x0 <= x) && (x <= x1) {
                // y = T(vy) - T(n - vy - 1) = (-n^2 + 2n*vy + n) / 2
                // Rearrange -> vy = (2y + n^2 - n) / (2n)
                // Interested in integer vy only, i.e. when 2y+n^2-n = k*2n
                // 2y/n + n - 1 = 2k
                // 2y/n + n is an odd integer.
                // 2y / n must be an integer, so 2y = a*n
                // Find lowest y where a*n/2 is in the range [y0, y1], then we can increment y by n/2 each time.
                let increment = if steps % 2 == 0 { steps / 2 } else { steps };
                let mut y_search_lo = (y0 / increment) * increment;
                if y_search_lo - increment >= y0 { y_search_lo -= increment; }
                if y_search_lo < y0 { y_search_lo += increment; }
                // // Find lowest y where a*n/2 is in the range [y0, y1], then we can increment y by n/2 each time.
                // let y_times_2 = (y0 / steps) * steps;
                // let increment = if steps % 2 == 0 { steps / 2 } else { steps };
                // let mut y_search_lo = y_times_2 / 2;
                // if y_search_lo - increment >= y0 { y_search_lo -= increment; }
                // if y_search_lo < y0 { y_search_lo += increment; }
                // if y_search_lo < y0 { y_search_lo += increment; }
                for y in (y_search_lo..=y1).step_by(increment as usize) {
                    if (2 * y / steps + steps) % 2 == 1 || (2 * y / steps + steps) % 2 == -1 {
                        let vy = (2 * y + steps * steps - steps) / (2 * steps);
                        // Max height is:
                        //  If vy < 0 => 0.
                        //  Min of (peak of flight, the number of steps being tested)
                        //  peak of flight occurs when n = vy. Use n = min(vy, steps)
                        let peak = match vy < 0 {
                            true => 0,
                            false => {
                                let n = if vy < steps { vy } else { steps };
                                tri_base_isize(vy, n)
                            }
                        };
                        match largest_peak {
                            Some(lp) => if peak > lp { largest_peak = Some(peak) },
                            None => largest_peak = Some(peak),
                        }
                    }
                }
            }
        }
    }
    largest_peak.unwrap()
}


fn part2(data: &str) -> usize {
    let re = Regex::new(r"^target area: x=(?P<x0>-?\d+)..(?P<x1>-?\d+), y=(?P<y0>-?\d+)..(?P<y1>-?\d+)\s*$").unwrap();
    let parsed = re.captures(data).unwrap();
    let x0 = parsed.name("x0").unwrap().as_str().parse::<isize>().unwrap();
    let x1 = parsed.name("x1").unwrap().as_str().parse::<isize>().unwrap();
    let y0 = parsed.name("y0").unwrap().as_str().parse::<isize>().unwrap();
    let y1 = parsed.name("y1").unwrap().as_str().parse::<isize>().unwrap();

    let v_max = x1;
    let v_min = tri_inv(x0 as f64).ceil() as isize;

    let n_min = 1;
    let n_max = 10000; // v_min;  // TODO: See if I can find the upper bound.

    let mut initial_velocities = HashSet::new();
    for steps in n_min..=n_max {
        for vx in v_min..=v_max {
            let x = if vx > steps { tri_base_isize(vx, steps) } else { tri_isize(vx) };
            if (x0 <= x) && (x <= x1) {
                // y = T(vy) - T(n - vy - 1) = (-n^2 + 2n*vy + n) / 2
                // Rearrange -> vy = (2y + n^2 - n) / (2n)
                // Interested in integer vy only, i.e. when 2y+n^2-n = k*2n
                // 2y/n + n - 1 = 2k
                // 2y/n + n is an odd integer.
                // 2y / n must be an integer, so 2y = a*n
                // Find lowest y where a*n/2 is in the range [y0, y1], then we can increment y by n/2 each time.
                let increment = if steps % 2 == 0 { steps / 2 } else { steps };
                let mut y_search_lo = (y0 / increment) * increment;
                if y_search_lo - increment >= y0 { y_search_lo -= increment; }
                if y_search_lo < y0 { y_search_lo += increment; }
                for y in (y_search_lo..=y1).step_by(increment as usize) {
                    if (2 * y / steps + steps) % 2 == 1 || (2 * y / steps + steps) % 2 == -1 {
                        let vy = (2 * y + steps * steps - steps) / (2 * steps);
                        initial_velocities.insert((vx, vy));
                    }
                }
            }
        }
    }
    initial_velocities.len()
}


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 45);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 112);
    }
}
