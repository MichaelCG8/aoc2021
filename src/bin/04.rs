use std::time;

struct Card {
    numbers: [[isize; 5]; 5],
    row_totals: [isize; 5],
    column_totals: [isize; 5],
    sum: isize,
    result: isize,
}

impl Card {
    pub fn new(data: &str) -> Self {
        let mut numbers: [[isize; 5]; 5] = [[0; 5]; 5];  // TODO: Try to have initialized in the loop.
        let row_totals= [0; 5];
        let column_totals = [0; 5];
        let mut sum = 0;
        let result = 0;

        for (idx_r, line) in data.split("\n").enumerate() {
            for (idx_c, entry) in line.trim().split_whitespace().enumerate() {
                let value = entry.parse().unwrap();
                sum += value;
                numbers[idx_r][idx_c] = value;
            }
        }

        Card{ numbers, row_totals, column_totals, sum, result }
    }

    pub fn mark(&mut self, value: isize) -> isize {
        for (idx_r, row) in self.numbers.iter().enumerate() {
            for (idx_c, &entry) in row.iter().enumerate() {
                if entry == value {
                    self.sum -= value;
                    self.row_totals[idx_r] += 1;
                    self.column_totals[idx_c] += 1;
                    if (self.row_totals[idx_r] == 5) | (self.column_totals[idx_c] == 5) {
                        self.result = self.sum * value;
                        return self.result;
                    }
                }
            }
        }
        0
    }
}


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/04");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn part1(data: &str) -> isize {
    let mut blocks = data.split("\n\n");
    let numbers = blocks.next().unwrap().split(",").map(|s| s.parse().unwrap());

    let mut cards: Vec<Card> = blocks.map(|s| Card::new(s)).collect();

    for value in numbers {
        for card in cards.iter_mut() {
            let result = card.mark(value);
            if result != 0 {
                return result;
            }
        }
    }
    panic!("Didn't find an answer");
}


fn part2(data: &str) -> isize {
    let mut blocks = data.split("\n\n");
    let numbers: Vec<isize> = blocks.next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();

    let mut cards: Vec<Card> = blocks.map(|s| Card::new(s)).collect();

    let mut idx = 0;
    for (i, &value) in numbers.iter().enumerate() {
        for card in cards.iter_mut() {
            card.mark(value);
        }
        cards.retain(|c| {c.result == 0});
        if cards.len() == 1 { idx = i; break; }
    }
    let card = &mut cards[0];
    for &value in numbers[idx+1..].iter() {
        let result = card.mark(value);
        if result != 0 {
            return result;
        }
    }

    panic!("Didn't find an answer.");
}


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 4512);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 1924);
    }
}
