use std::collections::HashMap;
use std::time;

fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/14");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}

struct Polymer {
    bonds: HashMap<(char, char), usize>,
    process: HashMap<(char, char), char>,
    start: char,
    end: char,
}

impl Polymer {
    fn new(s: &str) -> Self {
        let mut bonds: HashMap<(char, char), usize> = HashMap::new();
        let mut sections = s.split("\n\n");
        let initial: Vec<char> = sections.next().unwrap().chars().collect();

        for bond in initial.windows(2) {
            let bond = (bond[0], bond[1]);
            let count = bonds.entry(bond).or_insert(0);
            *count += 1;
        }

        let process_str = sections.next().unwrap();
        let mut process = HashMap::new();
        for line in process_str.lines() {
            let mut pair = line.split(" -> ");
            let bond: Vec<char> = pair.next().unwrap().chars().collect();
            let bond = (bond[0], bond[1]);
            let insert = pair.next().unwrap();
            process.insert(bond, insert.chars().next().unwrap());
        }

        Self { bonds, process, start: initial[0], end: initial[initial.len()-1]}
    }

    fn step(&mut self) {
        let mut new_bonds = HashMap::new();
        for (bond, count) in self.bonds.iter() {
            let &insert = self.process.get(bond).unwrap();
            *new_bonds.entry((bond.0, insert)).or_insert(0) += count;
            *new_bonds.entry((insert, bond.1)).or_insert(0) += count;
        }
        self.bonds = new_bonds;
    }

    fn totals(&self) -> HashMap<char, usize> {
        let mut totals = HashMap::new();
        for (bond, count) in self.bonds.iter() {
            *totals.entry(bond.0).or_insert(0) += count;
            *totals.entry(bond.1).or_insert(0) += count;
        }
        *totals.get_mut(&self.start).unwrap() += 1;
        *totals.get_mut(&self.end).unwrap() += 1;

        for count in totals.values_mut() {
            *count /= 2;
        }

        totals
    }

    fn greatest_diff(&self) -> usize {
        let totals = self.totals();
        totals.values().max().unwrap() - totals.values().min().unwrap()
    }
}

fn get_final_polymer(data: &str, steps: usize) -> Polymer {
    let mut polymer = Polymer::new(data);
    for _ in 0..steps {
        polymer.step();
    }
    polymer
}

fn part1(data: &str) -> usize {
    let polymer = get_final_polymer(data, 10);
    polymer.greatest_diff()
}

fn part2(data: &str) -> usize {
    let polymer = get_final_polymer(data, 40);
    polymer.greatest_diff()
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(get_final_polymer(DATA, 5).totals().values().sum::<usize>(), 97);
        assert_eq!(get_final_polymer(DATA, 10).totals().values().sum::<usize>(), 3073);
        assert_eq!(get_final_polymer(DATA, 10).totals().get(&'B').unwrap(), &1749);
        assert_eq!(get_final_polymer(DATA, 10).totals().get(&'C').unwrap(), &298);
        assert_eq!(get_final_polymer(DATA, 10).totals().get(&'H').unwrap(), &161);
        assert_eq!(get_final_polymer(DATA, 10).totals().get(&'N').unwrap(), &865);
        assert_eq!(get_final_polymer(DATA, 10).greatest_diff(), 1588);
    }

    #[test]
    fn part2_matches_sample() {
        // assert_eq!(part2(DATA), );
    }
}
