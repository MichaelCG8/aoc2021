use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::time;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/20");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}

#[derive(Clone)]
struct Element {
    now: usize,
    next: usize,
}

impl Element {
    fn new(now: usize) -> Self {
        Self { now, next: 0 }
    }

    fn update(&mut self) {
        self.now = self.next;
    }
}

impl ToString for Element {
    fn to_string(&self) -> String {
        match self.now {
            0 => '.',
            _ => '#',
        }.to_string()
    }
}

struct Image {
    data: VecDeque<VecDeque<Element>>,
}

impl ToString for Image {
    fn to_string(&self) -> String {
        self
            .data
            .iter()
            .map(
                |row|
                row.iter().map(|el| el.to_string()).collect::<String>()
            )
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Deref for Image {
    type Target = VecDeque<VecDeque<Element>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for Image {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

fn part1(data: &str) -> usize {
    process(data, 2)
}

fn process(data: &str, iterations: usize) -> usize {
    let mut data = data.split("\n\n");
    let algorithm: Vec<usize> = data
        .next()
        .unwrap()
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c { '.' => 0, '#' => 1, _ => panic!() })
        .collect();

    let mut image = VecDeque::new();
    for row in data.next().unwrap().lines() {
        image.push_back( {
            let mut row_vec = row
                .chars()
                .map(|c| match c { '.' => Element::new(0), '#' => Element::new(1), _ => panic!() })
                .collect::<VecDeque<Element>>();
            row_vec.push_front(Element::new(0));
            row_vec.push_back(Element::new(0));
            row_vec
        });
    }
    let width = image[0].len();
    image.push_front(std::iter::repeat(Element::new(0)).take(width).collect());
    image.push_back(std::iter::repeat(Element::new(0)).take(width).collect());


    let first_fill = 0;
    let first_toggle = algorithm[0];
    let second_toggle = if first_toggle == 0 { 0 } else { algorithm[(1<<9) - 1] };
    // TODO: Make this be read from the algorithm string.
    // NOTE: Fillers are 0, 0 for test data, 0, 1 for real data.
    let mut image = Image{ data: image};
    // println!("{}\n", image.to_string());
    enhance(&mut image, &algorithm, first_fill, first_toggle);
    // println!("{}\n", image.to_string());
    enhance(&mut image, &algorithm, first_toggle, second_toggle);
    // println!("{}\n", image.to_string());

    assert_eq!(iterations % 2, 0);

    for _ in 0..((iterations - 2)/2) {
        enhance(&mut image, &algorithm, second_toggle, first_toggle);
        enhance(&mut image, &algorithm, first_toggle, second_toggle);
    }

    image.iter().map(|row| row.iter().map(|el| el.now).sum::<usize>()).sum()
}

fn part2(data: &str) -> usize {
    process(data, 50)
}


fn enhance(image: &mut Image, algorithm: &Vec<usize>, now_filler: usize, next_filler: usize) {
    for row in image.iter_mut() {
        row.push_front(Element{ now: now_filler, next: next_filler});
        row.push_back(Element{ now: now_filler, next: next_filler});
    }
    let new_width = image[0].len();

    image.push_front(std::iter::repeat(Element{ now: now_filler, next: next_filler}).take(new_width).collect());
    image.push_back(std::iter::repeat(Element{ now: now_filler, next: next_filler}).take(new_width).collect());

    // println!("{}\n", image.to_string());

    for row in 1..(new_width as isize)-1 {
        for col in 1..(image.len() as isize)-1 {
            let mut index = 0;
            for r_offset in -1..=1 {
                for c_offset in -1..=1 {
                    index <<= 1;
                    index += image[(row + r_offset) as usize][(col + c_offset) as usize].now;
                }
            }
            image[row as usize][col as usize].next = algorithm[index];
        }
    }
    for row in 0..new_width {
        for col in 0..image.len() {
            image[row][col].update();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 35);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 3351);
    }
}
