use std::cmp::{max, min};
use itertools::Itertools;
use std::ops::Range;

fn part1(inp: &str) -> usize {
    let (seeds, blocks) = parse_input(inp);

    blocks.into_iter().fold(seeds, |cur, block| {
        cur.into_iter().map(|cur| {
            block.iter().find_map(|(to, from)| {
                from.contains(&cur).then(|| cur + to.start - from.start)
            }).unwrap_or(cur)
        }).collect()
    }).into_iter().min().unwrap()
}

fn part2(inp: &str) -> usize {
    let (seeds, blocks) = parse_input(inp);
    let seeds = seeds.into_iter().tuples().map(|(src, step)| src..src+step).collect::<Vec<_>>();

    blocks.into_iter().fold(seeds, |cur, block| {
        cur.into_iter().flat_map(|cur| {
            let (unmapped, mapped) = block.iter().fold((vec![cur], vec![]), |(unmapped, mapped_old), (to, from)| {
                let (unmapped, mapped): (Vec<_>, Vec<_>) = unmapped.into_iter().map(|range| {
                    let left = range.start..min(range.end, from.start);
                    let middle = max(range.start, from.start)+ to.start - from.start ..(min(range.end, from.end) + to.start).saturating_sub(from.start);
                    let right = max(range.start, from.end)..range.end;
                    ([left,right], middle)
                }).unzip();

                (
                    unmapped.into_iter().flatten().filter(|um| !um.is_empty()).collect(),
                    mapped.into_iter().filter(|um| !um.is_empty()).chain(mapped_old).collect()
                )
            });
            unmapped.into_iter().chain(mapped.into_iter()).collect::<Vec<_>>()
        }).collect()
    }).into_iter().map(|r| r.start).min().unwrap()
}

fn parse_input(inp: &str) -> (Vec<usize>, Vec<Vec<(Range<usize>, Range<usize>)>>) {
    let mut blocks = inp.trim().split("\n\n");
    let seeds = blocks
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let blocks = blocks
        .map(|block| {
            let mut lines = block.lines().skip(1);

            lines
                .map(|line| {
                    let nums = line
                        .split_whitespace()
                        .map(|num| num.parse().unwrap());
                    let (dst, src, step) = nums.collect_tuple().unwrap();
                    (dst..dst + step, src..src + step)
                })
                .collect()
        })
        .collect();

    (seeds, blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(35, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(322500873, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(46, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(108956227, result);
    }
}
