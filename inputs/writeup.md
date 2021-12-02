# Advent of Code 2021

# Intro

For this year's Advent of Code (AoC) I've decided to write up how I solve each problem.
The most interesting part of these puzzles for me is the algorithmic aspect, rather than the code itself, so that'll be
my focus.

I will provide my coded solutions, written in Rust, but a seasoned Rustacean will no doubt see that I'm still
somewhat new to the language - indeed improving my familiarity with it is exactly the reason why I've chosen it over a
language I've more experienced in, such as Python, C, or C++.
The snippets I provide will contain the algorithms needed for each day's puzzle, and I also have a few helper functions
for tasks such as converting the input strings to various formats, such as vectors of integers.  You can find all of my
code on my [GitHub account](https://github.com/MichaelCG8/aoc2021), with the shared code located in `lib.rs`.

The puzzles themselves are all available at https://adventofcode.com/2021, so I won't copy the whole puzzle here, instead
only summarising the problem.

So without further ado, let's jump in!

# Day 01
Today's puzzle involves a list of depth measurements of the sea floor, that look something like this:

```
199
200
208
210
200
207
240
269
260
263
```
The sample input for testing has 10 measurements, whereas my own puzzle input has 2000.
## Part 1

For part 1 we have to identify how many times the depth increases from the previous measurement, 7 in this sample input.

```
199 (N/A - no previous measurement)
200 (increased)
208 (increased)
210 (increased)
200 (decreased)
207 (increased)
240 (increased)
269 (increased)
260 (decreased)
263 (increased)
```

AoC starts easier towards the beginning of advent, and this puzzle can be solved simply by keeping a note of the
previous measurement and incrementing a counter whenever the current measurement is larger.

```rust
fn part1(data: &str) -> isize {
    let depths = aoc2021::str_to_isize_vec(data);
    let mut previous = depths[0];
    let mut n_increases = 0;
    for &depth in depths[1..].iter() {
        if depth > previous {
            n_increases += 1;
        }
        previous = depth;
    }
    n_increases
}
```

It would have been possible to iterate through all of `depths`, rather than skipping the first entry, which would have
given the slightly more readable line `for depth in depths {`. However,  this gives us the small speedup of processing
1999 values instead of 2000, plus the benefit of learning (and enjoying) more Rust language features.

# Part 2

The next part of day 01 ups the complexity a little. It turns out we're actually interested in where a rolling sum of
three adjacent depths increases!  For the same sample input, this is equivalent to summing all the values marked A, and
all the values marked B, then comparing sumA to sumB to check for an increase.

```
199  A
200  A B
208  A B C
210    B C D
200  E   C D
207  E F   D
240  E F G
269    F G H
260      G H
263        H
```

The sums are then as follows, resulting in 5 increases.

```
A: 607 (N/A - no previous sum)
B: 618 (increased)
C: 618 (no change)
D: 617 (decreased)
E: 647 (increased)
F: 716 (increased)
G: 769 (increased)
H: 792 (increased)
```

A similar approach can be taken as for part 1, keeping a note of the previous total and comparing it to a new total.

```rust
fn part2(data: &str) -> isize {
    let depths = aoc2021::str_to_isize_vec(data);
    let mut previous : isize = depths[0..3].iter().sum();
    let mut n_increases = 0;
    for depth_group in depths.windows(3) {
        let new_sum = depth_group.iter().sum();
        if new_sum > previous {
            n_increases += 1;
        }
        previous = new_sum;
    }
    n_increases
}
```

Rust vectors have the really nice `windows(size: usize)` method for tasks like these, which returns an iterator over all
the windows of length `size`.

# Day 02
