use std::str::FromStr;
use std::time;
use aoc2021;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/11");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}

struct Dumbo {
    charge: usize,
    neighbours: Vec<(usize, usize)>,
    flashed: bool,
    n_flashes: usize,
}

impl FromStr for Dumbo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Dumbo {
            charge: s.parse().unwrap(),
            neighbours: Vec::with_capacity(8),
            flashed: false,
            n_flashes: 0,
        })
    }
}

impl Dumbo {
    pub fn increment(&mut self, stack: &mut Vec<(usize, usize)>) {
        if self.flashed {
            return;
        }
        self.charge += 1;
        if self. charge > 9 {
            self.flashed = true;
            self.charge = 0;
            self.n_flashes += 1;
            for &neighbour in self.neighbours.iter() {
                stack.push(neighbour);
            }
        }
    }
}


struct Cave {
    grid: Vec<Vec<Dumbo>>,
    height: usize,
    width: usize,
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Vec<Dumbo>> = aoc2021::grid(s);
        let height = grid.len() as isize;
        let width = grid[0].len() as isize;

        for row in 0..height {
            for col in 0..width {
                for neighbour_row in (row-1)..=(row+1) {
                    if neighbour_row < 0 || neighbour_row >= height { continue; }
                    for neighbour_col in (col-1)..=(col+1) {
                        if neighbour_col < 0 || neighbour_col >= width { continue; }
                        grid[row as usize][col as usize].neighbours.push((neighbour_row as usize, neighbour_col as usize));
                    }
                }
            }
        }
        Ok(Cave{ grid, height: height as usize, width: width as usize })
    }
}


impl Cave {
    pub fn step(&mut self) {
        let mut stack = Vec::with_capacity(100);
        for row in 0..self.height {
            for col in 0..self.width {
                stack.push((row, col));
            }
        }
        loop {
            match stack.pop() {
                Some((row, col)) => {
                    self.grid[row][col].increment(&mut stack)
                },
                None => break,
            }
        }
    }

    pub fn reset(&mut self) {
        for row in self.grid.iter_mut() {
            for dumbo in row {
                dumbo.flashed = false;
            }
        }
    }

    pub fn all_flashed(&self) -> bool {
        self
            .grid
            .iter()
            .all(|row| row.iter().all(|dumbo| dumbo.flashed))
    }

    pub fn get_flashes(&self) -> usize {
        let mut total = 0;
        for row in &self.grid {
            for dumbo in row {
                total += dumbo.n_flashes;
            }
        }
        total
    }
}


fn part1(data: &str) -> usize {
    let mut cave = Cave::from_str(data).unwrap();
    for _step in 0..100 {
        cave.step();
        cave.reset();
    }
    cave.get_flashes()
}


fn part2(data: &str) -> isize {
    let mut cave = Cave::from_str(data).unwrap();
    let mut step = 0;
    loop {
        cave.step();
        step += 1;
        if cave.all_flashed() {
            break;
        }
        cave.reset();
    }
    step
}


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 1656);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 195);
    }
}
