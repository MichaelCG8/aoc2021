use std::time;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/10");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


fn part1(data: &str) -> usize {
    let openings = "([{<";

    let mut score = 0;

    for line in data.lines() {
        let mut token_stack = Vec::new();
        for c in line.chars() {
            if openings.contains(c) {
                let closer = match c {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => panic!("Unrecognized character."),
                };
                token_stack.push(closer);
            } else if match token_stack.last() {
                Some(&last) => c == last,
                None => false,  // token_stack is empty.
            } {
                token_stack.pop();
            } else {
                // token_stack is empty, or character didn't match expected closer.
                score += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("Unrecognized character."),
                };
                break;
            }
        }
    }

    score

}


fn part2(data: &str) -> usize {
    let openings = "([{<";

    let mut scores = Vec::new();

    'outer: for line in data.lines() {
        let mut token_stack = Vec::new();
        for c in line.chars() {
            if openings.contains(c) {
                let closer = match c {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => panic!("Unrecognized character."),
                };
                token_stack.push(closer);
            } else if match token_stack.last() {
                Some(&last) => c == last,
                None => false,  // token_stack is empty.
            } {
                token_stack.pop();
            } else {
                // token_stack is empty, or character didn't match expected closer.
                continue 'outer;
            }
        }
        let mut this_score = 0;
        for c in token_stack.iter().rev() {
            this_score *= 5;
            this_score += match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Unrecognized character."),
            };
        }
        scores.push(this_score);
    }

    scores.sort();
    let mid_idx = scores.len() / 2;
    scores[mid_idx]
}


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 26397);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 288957);
    }
}
