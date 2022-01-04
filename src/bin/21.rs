use regex::Regex;
use std::time;

fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/21");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}

fn starting_positions(data: &str) -> (usize, usize) {
    let re = Regex::new(r"^Player \d starting position: (?P<pos>\d+)$").unwrap();
    let lines = data.lines().collect::<Vec<&str>>();
    let mut pos1 = re
        .captures(lines[0])
        .unwrap()
        .name("pos")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let mut pos2 = re
        .captures(lines[1])
        .unwrap()
        .name("pos")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();

    (pos1, pos2)
}

fn part1(data: &str) -> usize {
    let (mut pos1, mut pos2) = starting_positions(data);

    let mut score1 = 0;
    let mut score2 = 0;
    let mut die = (1..=100).cycle();
    let mut rolls = 0;
    loop {
        let roll: usize = (&mut die).take(3).sum();
        rolls += 3;
        pos1 += roll;
        if pos1 > 10 {
            pos1 %= 10;
            if pos1 == 0 {
                pos1 = 10;
            }
        }
        score1 += pos1;
        if score1 >= 1000 {
            return score2 * rolls;
        }
        let roll: usize = (&mut die).take(3).sum();
        rolls += 3;
        pos2 += roll;
        if pos2 > 10 {
            pos2 %= 10;
            if pos2 == 0 {
                pos2 = 10;
            }
        }
        score2 += pos2;
        if score2 >= 1000 {
            return score1 * rolls;
        }
    }
}

fn turn_win_map(mut pos: usize, score: usize, turn: usize, arr: &mut Vec<usize>) {
    if arr.len() <= turn {
        arr.push(0);
    }

    for i0 in 1..=3 {
        for i1 in 1..=3 {
            for i2 in 1..=3 {
                let mut new_pos = pos + i0 + i1 + i2;
                if new_pos > 10 { new_pos -= 10; }
                let new_score = score + new_pos;
                if new_score >= 21 {
                    arr[turn] += 1;
                } else {
                    turn_win_map(new_pos, new_score, turn + 1, arr);
                }
            }
        }
    }
}

fn part2(data: &str) -> usize {
    let (mut pos1, mut pos2) = starting_positions(data);

    let mut player1_wins = Vec::new();
    turn_win_map(pos1, 0, 0, &mut player1_wins);

    let mut player1_losses = Vec::new();
    let mut last_cases = 1;
    for val in player1_wins.iter() {
        let mut cases = last_cases * 27;
        cases -= val;
        last_cases = cases;
        player1_losses.push(cases);
    }

    let mut player2_wins = Vec::new();
    turn_win_map(pos2, 0, 0, &mut player2_wins);

    let mut player2_losses = Vec::new();
    let mut last_cases = 1;
    for val in player2_wins.iter() {
        let mut cases = last_cases * 27;
        cases -= val;
        last_cases = cases;
        player2_losses.push(cases);
    }

    let mut n_p1_wins = 0;
    for (a, b) in player1_wins[1..].iter().zip(player2_losses) {
        n_p1_wins += a * b;
    }

    let mut n_p2_wins = 0;
    for (a, b) in player2_wins.iter().zip(player1_losses) {
        n_p2_wins += a * b;
    }

    if n_p1_wins > n_p2_wins { n_p1_wins } else { n_p2_wins }
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 739785);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 444356092776315);
    }
}


/*
Start at position 8

1:09:09 1:10:19 1:01:20 1:02:22
                        2:03:23
                        3:04:24

                2:02:21
                3:03:22

        2:01:10 1:02:12 1:03:15
                        2:04:16
                        3:05:17

                2:03:13 1:04:17
                        2:05:18
                        3:06:19

                3:04:14 1:05:19
                        2:06:20
                        3:07:21

        3:02:11 1:03:13 1:04:17
                        2:05:18
                        3:06:19

                2:04:14 1:05:19
                        2:06:20
                        3:07:21

                3:05:15 1:06:21
                        2:07:22
                        3:08:23

2:10:10 1:01:11 1:02:13 1:03:16
                        2:04:17
                        3:05:18

                2:03:14 1:04:18
                        2:05:19
                        3:06:20

                3:04:15 1:05:20
                        2:06:21
                        3:07:22

        2:02:12 1:03:15 1:04:19
                        2:05:20
                        3:06:21

                2:04:16 1:05:21
                        2:06:22
                        3:07:23

                3:05:17 1:06:23
                        2:07:24
                        3:08:25

        3:03:13 1:04:17 1:05:22
                        2:06:23
                        3:07:24

                2:05:18 1:06:24
                        2:07:25
                        3:08:26

                3:06:19 1:07:26
                        2:08:27
                        3:09:28

3:01:01 1:02:03 1:03:06 1:04:10
                        2:05:11
                        3:06:12

                2:04:07 1:05:12
                        2:06:13
                        3:07:14

                3:05:08 1:06:14
                        2:07:15
                        3:08:16

        2:03:04 1:04:08 1:05:13
                        2:06:14
                        3:07:15

                2:05:09 1:06:15
                        2:07:16
                        3:08:17

                3:06:10 1:07:17
                        2:08:18
                        3:09:19

        3:04:05 1:05:10 1:06:16
                        2:07:17
                        3:08:18

                2:06:11 1:07:18
                        2:08:19
                        3:09:20

                3:07:12 1:08:20
                        2:09:21
                        3:10:22


3 1
4 3
5 6
6 7
7 6
8 3
9 1



 */


