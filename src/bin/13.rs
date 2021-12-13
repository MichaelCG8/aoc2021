use std::str::FromStr;
use std::time;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/13");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


struct Grid {
    elements: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .map(|l| l.split(",").nth(0).unwrap().parse::<usize>().unwrap())
            .max()
            .unwrap()
            + 1;
        let height = s
            .lines()
            .map(|l| l.split(",").nth(1).unwrap().parse::<usize>().unwrap())
            .max()
            .unwrap()
            + 1;
        let mut elements = vec![vec![false; width]; height];

        for line in s.lines() {
            let mut split = line.split(",");
            let x: usize = split.next().unwrap().parse().unwrap();
            let y: usize = split.next().unwrap().parse().unwrap();
            elements[y][x] = true;
        }

        Ok(Self { elements, height, width })
    }
}


impl Grid {
    fn count_dots(&self) -> usize {
        self.elements.iter().map(|row| row.iter().filter(|&&el| el).count()).sum()
    }

    fn fold(&mut self, fold: &Fold) {
        match fold.axis {
            'x' => self.fold_x(fold.line),
            'y' => self.fold_y(fold.line),
            _ => panic!(),
        }
    }

    fn fold_x(&mut self, line: usize) {
        for source in (line+1)..self.width {
            let offset = source - line;
            let dest = line - offset;
            for row in 0..self.height {
                self.elements[row][dest] |= self.elements[row][source]
            }
        }
        for row in self.elements.iter_mut() {
            row.truncate(line);
        }
        self.width = line;
    }

    fn fold_y(&mut self, line: usize) {
        for source in (line+1)..self.height {
            let offset = source - line;
            let dest = line - offset;
            for col in 0..self.width {
                self.elements[dest][col] |= self.elements[source][col]
            }
        }
        self.elements.truncate(line);
        self.height = line;
    }

    fn to_string(&self) -> String {
        self.elements
            .iter()
            .map(|row| row.iter().map(|&c| if c {'#'} else {'.'}).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}


struct Fold {
    axis: char,
    line: usize,
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let axis = s.chars().nth(11).unwrap();
        let line = s[13..].parse().unwrap();
        Ok(Self { axis, line })
    }
}


fn parse_input(data: &str) -> (Grid, Vec<Fold>) {
    let mut sections = data.split("\n\n");
    let grid = Grid::from_str(sections.next().unwrap()).unwrap();
    let folds = sections.next().unwrap().lines().map(|l| Fold::from_str(l).unwrap()).collect();
    (grid, folds)
}


fn part1(data: &str) -> usize {
    let (mut grid, folds) = parse_input(data);
    grid.fold(&folds[0]);
    grid.count_dots()
}

fn part2(data: &str) -> isize {
    let (mut grid, folds) = parse_input(data);
    for fold in folds.iter() {
        grid.fold(fold);
    }
    println!("{}", grid.to_string());
    0
}


#[cfg(test)]
mod tests {
    use super::*;
    static DATA_0 : &str = "...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........";
    static DATA_1 : &str = "#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###
...........
...........";
    static DATA_2 : &str = "#####
#...#
#...#
#...#
#####
.....
.....";
    static INSTRUCTIONS : &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part1_matches_sample() {
        let (mut grid, folds) = parse_input(INSTRUCTIONS);
        assert_eq!(grid.to_string(), DATA_0.to_string());
        grid.fold(&folds[0]);
        assert_eq!(grid.to_string(), DATA_1.to_string());
        grid.fold(&folds[1]);
        assert_eq!(grid.to_string(), DATA_2.to_string());
    }

    #[test]
    fn part2_matches_sample() {
        // assert_eq!(part2(DATA), );
    }
}
