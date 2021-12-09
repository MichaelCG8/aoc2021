use std::collections::HashMap;
use std::str::FromStr;
use std::time;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/08");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


struct IO7Seg {
    input: Vec<String>,
    output: Vec<String>,
    patterns: Option<HashMap<String, usize>>,
}

impl FromStr for IO7Seg {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split("|");

        let input = sections
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s1| { let mut v = s1.chars().collect::<Vec<char>>(); v.sort(); v.iter().collect::<String>()})
            .collect();

        let output = sections
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s1| { let mut v = s1.chars().collect::<Vec<char>>(); v.sort(); v.iter().collect::<String>()})
            .collect();

        let patterns = None;

        Ok(Self{ input, output, patterns })
    }
}

impl IO7Seg {
    fn count_output_1478(&self) -> usize {
        self.output
            .iter()
            .filter(|d| match d.len() { 2 | 4 | 3 | 7 => true, _ => false })
            .count()
    }

    fn identify_patterns(&'_ mut self) -> () {
        if self.patterns.is_some() { return; }
        let mut patterns = HashMap::new();

        for digit in self.input.iter() {
            let value = match digit.len() {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,
                _ => continue,
            };
            patterns.insert(value, digit.clone());
        }
        // 2, 3, and 5 use five segments.
        let fives: Vec<&String> = self.input.iter().filter(|d| d.len() == 5).collect();

        // 0, 6, and 9 use six segments.
        let sixes: Vec<&String> = self.input.iter().filter(|d| d.len() == 6).collect();

        // Of the five segment numbers, only 3 shares two segments with 1.
        for &digit in fives.iter() {
            if patterns.get(&1).unwrap().chars().filter(|&c| digit.contains(c)).count() == 2 {
                patterns.insert(3, digit.clone());
                break;
            }
        }

        // Of the six segment numbers, only 6 doesn't share two segments with 1.
        for &digit in sixes.iter() {
            if patterns.get(&1).unwrap().chars().filter(|&c| digit.contains(c)).count()  != 2 {
                patterns.insert(6, digit.clone());
                break;
            }
        }

        // Of the five segment numbers, only 5 is contained in 6.
        for &digit in fives.iter() {
            if patterns.get(&6).unwrap().chars().filter(|&c| digit.contains(c)).count()  == 5 {
                patterns.insert(5, digit.clone());
                break;
            }
        }

        // The remaining five segment number is 2.
        for &digit in fives.iter() {
            if (patterns.get(&3).unwrap().chars().filter(|&c| digit.contains(c)).count()  != 5) && (patterns.get(&5).unwrap().chars().filter(|&c| digit.contains(c)).count()  != 5) {
                patterns.insert(2, digit.clone());
                break;
            }
        }


        // Of the six segment numbers, only 9 contains all of 3.
        for &digit in sixes.iter() {
            if patterns.get(&3).unwrap().chars().filter(|&c| digit.contains(c)).count()  == 5 {
                patterns.insert(9, digit.clone());
                break;
            }
        }

        // The remaining six segment number is 0.
        for &digit in sixes.iter() {
            if (patterns.get(&6).unwrap().chars().filter(|&c| digit.contains(c)).count()  != 6) && (patterns.get(&9).unwrap().chars().filter(|&c| digit.contains(c)).count()  != 6) {
                patterns.insert(0, digit.clone());
                break;
            }
        }

        self.patterns = Some(patterns.into_iter().map(|(k, v)| (v, k)).collect());
    }

    fn get_output_number(&mut self) -> usize {
        self.identify_patterns();
        let mut result = 0;
        for digit in self.output.iter() {
            result *= 10;
            result += self.patterns.as_ref().unwrap().get(digit).unwrap();
        }
        result
    }
}


fn part1(data: &str) -> usize {
    data
        .lines()
        .map(|s| s.parse::<IO7Seg>().unwrap())
        .map(|io| io.count_output_1478())
        .sum()
}


fn part2(data: &str) -> usize {
    let mut data: Vec<IO7Seg> = data.lines().map(|s| s.parse().unwrap()).collect();
    data.iter_mut().map(|d| d.get_output_number()).sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 26);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 61229);
    }
}
