use std::{collections::HashMap, mem::swap, usize};

use aoc_2024::{count_digits, split_number};

const USE_TEST_INPUT: bool = false;
static TEST_INPUT: &str = r#"125 17"#;
static INPUT: &str = include_str!("../inputs/day11");

// ~8ms for 75 blinks
fn blink_iterative(line: &[usize], count: usize) -> usize {
    let mut stones: HashMap<usize, usize> = line.iter().map(|stone| (*stone, 1)).collect();
    let mut new_stones = HashMap::with_capacity(stones.capacity());
    for _ in 0..count {
        new_stones.clear();
        for (&stone, &count) in stones.iter() {
            if stone == 0 {
                new_stones
                    .entry(1)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            } else if count_digits(stone) % 2 == 0 {
                let (left, right) = split_number(stone);
                for v in [left, right] {
                    new_stones
                        .entry(v)
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                }
            } else {
                new_stones
                    .entry(stone * 2024)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }
        swap(&mut stones, &mut new_stones);
    }
    stones.into_iter().fold(0, |acc, (_, count)| acc + count)
}
// https://adventofcode.com/2024/day/11
fn main() {
    let debug = USE_TEST_INPUT || INPUT.len() == 0;
    let line: Vec<_> = if debug { TEST_INPUT } else { INPUT }
        .trim()
        .split(' ')
        .map(|x| (x.parse::<usize>().unwrap()))
        .collect();
    let res1 = blink_iterative(&line, 25);
    let res2 = blink_iterative(&line, 75);
    println!("{}", res1);
    println!("{}", res2);
}

#[allow(dead_code)]
// ~23ms for 75 blinks
fn blink_recursive(line: &[usize], count: usize) -> usize {
    let mut sum = 0;
    fn _blink(
        stone: usize,
        count: usize,
        cache: HashMap<(usize, usize), usize>,
    ) -> (usize, HashMap<(usize, usize), usize>) {
        if count == 0 {
            return (1, cache);
        }
        if let Some(stone_count) = cache.get(&(stone, count)) {
            return (*stone_count, cache);
        }
        if stone == 0 {
            let (stone_count, mut cache) = _blink(1, count - 1, cache);
            cache.insert((1, count - 1), stone_count);
            return (stone_count, cache);
        } else if count_digits(stone) % 2 == 0 {
            let (left, right) = split_number(stone);
            let (stone_count_left, mut cache) = _blink(left, count - 1, cache);
            cache.insert((left, count - 1), stone_count_left);
            let (stone_count_right, mut cache) = _blink(right, count - 1, cache);
            cache.insert((right, count - 1), stone_count_right);
            return (stone_count_left + stone_count_right, cache);
        } else {
            let (stone_count, mut cache) = _blink(stone * 2024, count - 1, cache);
            cache.insert((stone * 2024, count - 1), stone_count);
            return (stone_count, cache);
        }
    }

    let mut cache = HashMap::new();
    for stone in line {
        let (count, filled_cache) = _blink(*stone, count, cache);
        sum += count;
        cache = filled_cache;
    }
    sum
}
