use std::cmp::max;

pub(crate) fn part1(inp: &str) -> usize {
    parse_input(inp)
        .enumerate()
        .map(|(i, mut parts)| {
            if parts.all(|(num, color)| match color {
                "red" => num <= 12,
                "green" => num <= 13,
                "blue" => num <= 14,
                _ => unreachable!(),
            }) {
                i + 1
            } else {
                0
            }
        })
        .sum()
}

fn part2(inp: &str) -> usize {
    parse_input(inp)
        .map(|parts| {
            let (red, green, blue) =
                parts.fold((0, 0, 0), |(red, green, blue), (num, color)| match color {
                    "red" => (max(red, num), green, blue),
                    "green" => (red, max(green, num), blue),
                    "blue" => (red, green, max(blue, num)),
                    _ => unreachable!(),
                });
            red * green * blue
        })
        .sum()
}

fn parse_input(inp: &str) -> impl Iterator<Item = impl Iterator<Item = (usize, &str)>> {
    inp.lines().map(|line| {
        line.split_once(": ")
            .unwrap()
            .1
            .split("; ")
            .flat_map(|p| p.split(", "))
            .map(|part| {
                let (num, color) = part.split_once(" ").unwrap();
                (num.parse().unwrap(), color)
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(8, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(2416, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(2286, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(63307, result);
    }
}
