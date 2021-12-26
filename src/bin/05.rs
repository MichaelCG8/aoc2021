use std::collections::HashSet;
use std::time;

fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/05");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(s: &str) -> Self {
        let (x, y) = s.split_once(",").unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        Self { x, y }
    }
}

struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn new(s: &str) -> Self {
        let (p1, p2) = s.split_once(" -> ").unwrap();
        let p1 = Point::new(p1);
        let p2 = Point::new(p2);
        Self { p1, p2 }
    }

    pub fn iter_points_v_h(&self) -> Box<dyn Iterator<Item = Point>> {
        let (x1, x2) = (self.p1.x, self.p2.x);
        let (y1, y2) = (self.p1.y, self.p2.y);

        if x1 == x2 {
            let x = x1;
            let range = if y1 < y2 { y1..=y2 } else { y2..=y1 };
            Box::new(range.map(move |y| Point { x, y }))
        } else if y1 == y2 {
            let y = y1;
            let range = if x1 < x2 { x1..=x2 } else { x2..=x1 };
            Box::new(range.map(move |x| Point { x, y }))
        } else {
            Box::new((0..0).map(|_| Point { x: 0, y: 0 }))
        }
    }

    pub fn iter_points_all(&self) -> Box<dyn Iterator<Item = Point>> {
        let (x1, x2) = (self.p1.x, self.p2.x);
        let (y1, y2) = (self.p1.y, self.p2.y);

        if x1 == x2 {
            let x = x1;
            let range = if y1 < y2 { y1..=y2 } else { y2..=y1 };
            Box::new(range.map(move |y| Point { x, y }))
        } else if y1 == y2 {
            let y = y1;
            let range = if x1 < x2 { x1..=x2 } else { x2..=x1 };
            Box::new(range.map(move |x| Point { x, y }))
        } else {
            let xrange = if x1 < x2 { x1..=x2 } else { x2..=x1 };
            let yrange = if y1 < y2 { y1..=y2 } else { y2..=y1 };
            if (x1 < x2 && y1 < y2) || (x1 > x2 && y1 > y2) {
                Box::new(xrange.zip(yrange).map(move |(x, y)| Point { x, y }))
            } else {
                Box::new(xrange.rev().zip(yrange).map(move |(x, y)| Point { x, y }))
            }
        }
    }
}

fn part1(data: &str) -> usize {
    let lines = data.lines().map(|l| Line::new(l));

    let mut vents = HashSet::new();
    let mut more_vents = HashSet::new();
    for line in lines {
        for point in line.iter_points_v_h() {
            if vents.contains(&point) {
                more_vents.insert(point);
            } else {
                vents.insert(point);
            }
        }
    }
    more_vents.len()
}

fn part2(data: &str) -> usize {
    let lines = data.lines().map(|l| Line::new(l));

    let mut vents = HashSet::new();
    let mut more_vents = HashSet::new();
    for line in lines {
        for point in line.iter_points_all() {
            if vents.contains(&point) {
                more_vents.insert(point);
            } else {
                vents.insert(point);
            }
        }
    }
    more_vents.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 5);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 12);
    }
}
