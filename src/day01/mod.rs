fn part1(inp: &str) -> usize {
    inp.lines()
        .map(|line| {
            let first = line.chars().filter_map(|n| n.to_digit(10)).next().unwrap();
            let last = line
                .chars()
                .rev()
                .filter_map(|n| n.to_digit(10))
                .next()
                .unwrap();
            (first * 10 + last) as usize
        })
        .sum()
}

pub fn part2(inp: &str) -> usize {
    inp.lines()
        .map(|mut line| {
            let mut first = None;
            let mut last = 0;
            let mut got = |n| {
                first.get_or_insert(n);
                last = n;
            };

            while !line.is_empty() {
                if let Some(d) = line.chars().next().unwrap().to_digit(10) {
                    got(d as usize)
                } else {
                    for (i, num) in NUMS.iter().enumerate() {
                        if line.starts_with(num) {
                            got(i + 1)
                        }
                    }
                }
                line = &line[1..];
            }

            first.unwrap() * 10 + last
        })
        .sum()
}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(142, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(54597, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example2"));
        assert_eq!(281, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(54504, result);
    }
}
