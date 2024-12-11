use std::{fmt::Display, iter, mem::swap};
const USE_TEST_INPUT: bool = false;
static TEST_INPUT: &[u8] = br#"2333133121414131402"#;
static INPUT: &[u8] = include_bytes!("../inputs/day09");

#[derive(Debug)]
enum BlockEntry {
    Taken { id: u64, size: u8, start_idx: usize },
    Free { size: u8, start_idx: usize },
}
impl Display for BlockEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            BlockEntry::Taken {
                id,
                size,
                start_idx: _,
            } => {
                for c in iter::repeat_n(&Some(*id), *size as usize).map(block_as_char) {
                    write!(f, "{c}")?;
                }
            }
            BlockEntry::Free { size, start_idx: _ } => {
                write!(f, "{}", ".".repeat(*size as usize))?;
            }
        }
        Ok(())
    }
}
fn block_as_char(blok: &Option<u64>) -> char {
    match blok {
        Some(id) => {
            let id = {
                if *id <= 9 {
                    *id as u8 + b'0'
                } else {
                    b'b'
                }
            };

            id as char
        }
        None => '.',
    }
}
fn print_blocks<'a>(bloks: impl Iterator<Item = &'a Option<u64>>) {
    for block in bloks {
        print!("{}", block_as_char(block));
    }
    println!();
}
fn fill_aggressive(line: &[u8], debug: bool) -> u64 {
    let mut slots: Vec<_> = line
        .chunks(2)
        .enumerate()
        .flat_map(|(id, chars)| {
            let size = chars[0] - b'0';
            let free_after = chars.get(1).unwrap_or(&b'0') - b'0';
            let mut vec = vec![None; (size + free_after).into()];
            for i in 0..size as usize {
                vec[i] = Some(id as u64);
            }
            vec
        })
        .collect();
    if debug {
        print_blocks(slots.iter());
    }
    let mut sum = 0;
    let mut right = slots.len() - 1;
    for left in 0..slots.len() {
        if slots[left].is_none() {
            while slots[right].is_none() {
                right -= 1;
            }
            if right < left {
                break;
            }
            slots.swap(left, right);
        }
        sum += (left as u64 * slots[left].unwrap()) as u64;
    }

    if debug {
        print_blocks(slots.iter());
    }
    sum
}
fn fill_whole_blocks(line: &[u8], debug: bool) -> u64 {
    let (mut blocks, _) = line.chunks(2).enumerate().fold(
        // to prevent allocs, theoretical max capacity is line.len() (one BlockEntry be it Taken or
        // free) + 1 for odd line length
        (Vec::with_capacity(line.len() + 1), 0),
        |(mut vec, pos), (id, chars)| {
            let size = chars[0] - b'0';
            vec.push(BlockEntry::Taken {
                id: id as u64,
                size,
                start_idx: pos,
            });
            let free_after = chars.get(1).map(|c| c - b'0').unwrap_or(0);
            vec.push(BlockEntry::Free {
                size: free_after,
                start_idx: pos + size as usize,
            });
            (vec, pos + (size + free_after) as usize)
        },
    );
    assert_eq!(blocks.capacity(), line.len() + 1);
    if debug {
        for blk in &blocks {
            print!("{}", blk);
        }
        println!()
    }

    for right in (0..blocks.len()).rev() {
        let (start, end) = blocks.split_at_mut(right);
        let BlockEntry::Taken {
            id: _,
            size,
            start_idx,
        } = &mut end[0]
        else {
            continue;
        };
        let Some(BlockEntry::Free {
            size: free_size,
            start_idx: free_start_idx,
        }) = start.iter_mut().find(|b| match b {
            BlockEntry::Taken {
                id: _,
                size: _,
                start_idx: _,
            } => false,
            BlockEntry::Free {
                size: free_size,
                start_idx: free_start_idx,
            } => free_size >= size && free_start_idx < start_idx,
        })
        else {
            continue;
        };
        swap(free_start_idx, start_idx);
        if free_size != size {
            *free_start_idx = *start_idx + *size as usize;
            *free_size -= *size;
        }
    }
    blocks.sort_by_key(|b| *match b {
        BlockEntry::Taken {
            id: _,
            size: _,
            start_idx,
        } => start_idx,
        BlockEntry::Free { size: _, start_idx } => start_idx,
    });
    if debug {
        for blk in &blocks {
            print!("{}", blk);
        }
        println!()
    }
    blocks.into_iter().fold(0, |mut acc, blk| match blk {
        BlockEntry::Taken {
            id,
            size,
            start_idx,
        } => {
            for i in 0..size {
                acc += (start_idx + i as usize) as u64 * id
            }
            acc
        }
        _ => acc,
    })
}
// https://adventofcode.com/2024/day/9
fn main() {
    let debug = USE_TEST_INPUT || INPUT.len() == 0;
    let line = if debug { TEST_INPUT } else { INPUT }.trim_ascii();
    let res1 = fill_aggressive(line, debug);
    println!("{}", res1);
    let res2 = fill_whole_blocks(line, debug);
    println!("{}", res2);
}
