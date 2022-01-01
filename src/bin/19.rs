use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;
use std::time;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/19");
    let start_part1 = time::Instant::now();
    // println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    let start_part2 = time::Instant::now();
    println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

impl Coord {
    fn new(s: &str) -> Self {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        Self { x, y, z }
    }
}

struct Scanner {
    beacons: [Option<Vec<Coord>>; 24],
    index: usize,
}

impl FromStr for Scanner {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.lines()
                .skip(1)
                .map(|l| Coord::new(l))
                .collect()
        ))
    }
}

impl Scanner {
    fn new(mut beacons: Vec<Coord>) -> Self {
        beacons.sort_unstable();
        const INIT: Option<Vec<Coord>> = None;
        let mut beacons_arr = [INIT; 24];
        beacons_arr[0] = Some(beacons);
        Self { beacons: beacons_arr, index: 0 }
    }

    fn current_coords(&self) -> &Vec<Coord> {
        self.beacons[self.index].as_ref().unwrap()
    }

    fn rotate(&mut self) {
        self.index += 1;
        self.index %= 24;
        if self.beacons[self.index].is_none() {
            if self.index % 4 == 0 {
                // Rotate around x
                self.rotate_x_clockwise();
            } else {
                // Rotate around z.
                // Each 4 we change the direction of rotation.
                match (self.index / 4) % 2 == 0 {
                    true => self.rotate_z_clockwise(),
                    false => self.rotate_z_anticlockwise(),
                }
            }
            self.beacons[self.index].as_mut().unwrap().sort_unstable();
        }
    }

    fn rotate_x_clockwise(&mut self) {
        let original = self.beacons[self.index - 1].as_ref().unwrap();
        self.beacons[self.index] = Some(
            original
                .iter()
                .map(|Coord{x, y, z}| Coord { x: *x, y: *z, z: -y })
                .collect()
        )
    }

    fn rotate_z_clockwise(&mut self) {
        let original = self.beacons[self.index - 1].as_ref().unwrap();
        self.beacons[self.index] = Some(
            original
                .iter()
                .map(|Coord{x, y, z}| Coord { x: *y, y: -x, z: *z })
                .collect()
        )
    }

    fn rotate_z_anticlockwise(&mut self) {
        let original = self.beacons[self.index - 1].as_ref().unwrap();
        self.beacons[self.index] = Some(
            original
                .iter()
                .map(|Coord{x, y, z}| Coord { x: -y, y: *x, z: *z })
                .collect()
        )
    }

    fn count_in_volume(&self, low: &Coord, high: &Coord, relative: &Coord) -> usize {
        let low = coord_sub(low, relative);
        let high = coord_sub(high, relative);
        self.beacons[self.index]
            .as_ref()
            .unwrap()
            .iter()
            .filter(|beacon|
                beacon.x >= low.x
                    && beacon.x <= high.x
                    &&beacon.y >= low.y
                    && beacon.y <= high.y
                    &&beacon.z >= low.z
                    && beacon.z <= high.z
            )
            .count()
    }
}


fn coords_to_diffs(coords: &Vec<Coord>, relative_to_idx: usize) -> Vec<Coord> {
    let reference = &coords[relative_to_idx];
    coords.iter().map(|c| coord_sub(c, reference)).collect()
}

struct Map {
    scanners: Vec<HashSet<Coord>>,
    relatives: Vec<Coord>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanners = vec![s.lines().skip(1).map(|l| Coord::new(l)).collect()];
        Ok(Self { scanners, relatives: vec![Coord { x: 0, y: 0, z: 0 }]})
    }
}

fn coord_sub(a: &Coord, b: &Coord) -> Coord {
    Coord {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

fn coord_add(a: &Coord, b: &Coord) -> Coord {
    Coord {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

impl Map {
    fn matches(&self, other: &mut Scanner) -> Option<(HashSet<Coord>, Coord)> {
        for _ in 0..24 {
            for (scanner, this_relative) in self.scanners.iter().zip(self.relatives.iter()) {
                for this_reference in scanner.iter() {
                    let this_diffs: HashSet<Coord> = scanner
                        .iter()
                        .map(|beacon| coord_sub(beacon, this_reference))
                        .collect();
                    for other_reference in other.current_coords().iter() {
                        let other_diffs: HashSet<Coord> = other
                            .current_coords()
                            .iter()
                            .map(|beacon| coord_sub(beacon, other_reference))
                            .collect();
                        let intersection: Vec<_> = this_diffs.intersection(&other_diffs).collect();
                        let n_matches = intersection.len();
                        if n_matches >= 12 {
                            let relative = coord_sub(this_reference, other_reference);

                            let (low, high) = self.shared_volume(&relative, &this_relative);

                            let other_in_volume = other.count_in_volume(&low, &high, &relative);
                            let this_in_volume = self.count_in_volume(&low, &high);
                            if (other_in_volume == n_matches) && (this_in_volume == n_matches) {
                                let new_coords = other_diffs
                                    .iter()
                                    .map(|d| coord_add(d, this_reference))
                                    .collect();
                                return Some((new_coords, relative));
                            }
                        }
                    }
                }
            }
            other.rotate();
        }
        None
    }

    fn merge(&mut self, new_coords: HashSet<Coord>, relative: Coord) {
        self.scanners.push(new_coords);
        self.relatives.push(relative);
    }

    fn count_beacons(&self) -> usize {
        let mut beacons = HashSet::new();
        for scanner in self.scanners.iter() {
            beacons.extend(scanner);
        }
        beacons.len()
    }

    fn count_in_volume(&self, low: &Coord, high: &Coord) -> usize {
        let mut beacons = HashSet::new();
        for scanner in self.scanners.iter() {
            beacons.extend(scanner)
        }
        beacons
            .iter()
            .filter(|beacon| beacon.x >= low.x
                && beacon.x <= high.x
                && beacon.y >= low.y
                && beacon.y <= high.y
                && beacon.z >= low.z
                && beacon.z <= high.z)
            .count()
    }

    fn shared_volume(&self, other: &Coord, this_relative: &Coord) -> (Coord, Coord) {
        let (x_lo, x_hi) = if other.x < this_relative.x {
            (this_relative.x - 1000, other.x + 1000)
        } else {
            (other.x - 1000, this_relative.x + 1000)
        };
        let (y_lo, y_hi) = if other.y < this_relative.y {
            (this_relative.y - 1000, other.y + 1000)
        } else {
            (other.y - 1000, this_relative.y + 1000)
        };
        let (z_lo, z_hi) = if other.z < this_relative.z {
            (this_relative.z - 1000, other.z + 1000)
        } else {
            (other.z - 1000, this_relative.z + 1000)
        };

        (Coord { x: x_lo, y: y_lo, z: z_lo }, Coord { x: x_hi, y: y_hi, z: z_hi })
    }

    fn largest_manhattan_diff(&self) -> isize {
        self.relatives
            .iter()
            .combinations(2)
            .map(|ab| (ab[0].x - ab[1].x).abs() + (ab[0].y - ab[1].y).abs() + (ab[0].z - ab[1].z).abs())
            .max()
            .unwrap()
    }
}

fn part1(data: &str) -> usize {
    let mut scanners = data.split("\n\n");
    let mut map = Map::from_str(scanners.next().unwrap()).unwrap();
    let mut others: Vec<Scanner> = scanners
        .map(|scanner| Scanner::from_str(scanner).unwrap())
        .collect();

    let mut n_left = others.len();
    loop {
        let mut matching_scanners = Vec::new();
        for (i, scanner) in others.iter_mut().enumerate() {
            if let Some((new_coords, relative)) = map.matches(scanner) {
                // scanner is now in the correct rotation.
                map.merge(new_coords, relative);
                matching_scanners.push(i);
                n_left -= 1;
                println!("n_left: {}", n_left);
            }
        }
        others = others
            .into_iter()
            .enumerate()
            .filter(|(i, _scanner)| !matching_scanners.contains(i))
            .map(|(_i, scanner)| scanner)
            .collect();
        if others.is_empty() { break; }
    }

    map.count_beacons()
}

fn part2(data: &str) -> isize {
    let mut scanners = data.split("\n\n");
    let mut map = Map::from_str(scanners.next().unwrap()).unwrap();
    let mut others: Vec<Scanner> = scanners
        .map(|scanner| Scanner::from_str(scanner).unwrap())
        .collect();

    let mut n_left = others.len();
    loop {
        let mut matching_scanners = Vec::new();
        for (i, scanner) in others.iter_mut().enumerate() {
            if let Some((new_coords, relative)) = map.matches(scanner) {
                // scanner is now in the correct rotation.
                map.merge(new_coords, relative);
                matching_scanners.push(i);
                n_left -= 1;
                println!("n_left: {}", n_left);
            }
        }
        others = others
            .into_iter()
            .enumerate()
            .filter(|(i, _scanner)| !matching_scanners.contains(i))
            .map(|(_i, scanner)| scanner)
            .collect();
        if others.is_empty() { break; }
    }

    map.largest_manhattan_diff()
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn part1_matches_sample() {
        assert_eq!(part1(DATA), 79);
    }

    #[test]
    fn part2_matches_sample() {
        assert_eq!(part2(DATA), 3621);
    }
}
