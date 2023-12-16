use itertools::Itertools;
use num::integer::sqrt;

fn part1(inp: &str) -> usize {
    let (times, distances)= inp.lines().map(|line| line.split_once(':').unwrap().1.split_whitespace().map(|n| n.parse().unwrap())).collect_tuple().unwrap();
    times.zip(distances).map(|(time, record)| {
        calc(time, record)
    }).product()
}

fn part2(inp: &str) -> usize {
    let (time, record) = inp.lines().map(|line| line.split_once(':').unwrap().1.replace(' ', "").parse::<usize>().unwrap()).collect_tuple().unwrap();
    calc(time, record)
}

fn calc(time: usize, record: usize) -> usize {
    let discriminant = time * time - 4 * record;
    if discriminant >= 0 {
        let lhs_bound = (time as f64 - (discriminant as f64).sqrt()) / 2.;
        let rhs_bound = (time as f64 + (discriminant as f64).sqrt()) / 2.;
        (rhs_bound - 1.).ceil() as usize + 1 - (lhs_bound + 1.).floor() as usize
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(288, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(160816, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(71503, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(46561107, result);
    }
}
