use std::cmp::max;
use std::collections::HashSet;

fn part1(inp: &str) -> usize {
    inp.lines()
        .map(|line| {
            let (nums, winning_nums) = line
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .split_once('|')
                .unwrap();
            let nums = nums
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<HashSet<u8>>();
            let correct = winning_nums
                .split_whitespace()
                .filter(|winning_num| nums.contains(&winning_num.parse::<u8>().unwrap()))
                .count();
            (1 << correct) >> 1
        })
        .sum()
}

fn part2(inp: &str) -> usize {
    let mut cards = Vec::new();

    for (card_num, line) in inp.lines().enumerate() {
        let (nums, winning_nums) = line
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split_once('|')
            .unwrap();
        let nums = nums
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<HashSet<u8>>();
        let correct = winning_nums
            .split_whitespace()
            .filter(|winning_num| nums.contains(&winning_num.parse::<u8>().unwrap()))
            .count();

        cards.resize(max(cards.len(), card_num + correct + 1), 1);
        let this_cards = cards[card_num];
        for card in &mut cards[card_num + 1..card_num + 1 + correct] {
            *card += this_cards;
        }
    }

    cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(13, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(20829, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(30, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(12648035, result);
    }
}
