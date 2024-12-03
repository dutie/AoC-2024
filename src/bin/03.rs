advent_of_code::solution!(3);
use regex::Regex;

// Expected: mul(X,Y), where X, Y are int and 0 <= X, Y <= 999
pub fn part_one(input: &str) -> Option<u32> {
    let mut product = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(input) {
        let num1: u32 = cap[1].parse().unwrap();
        let num2: u32 = cap[2].parse().unwrap();
        product += num1 * num2;
    }
    Some(product)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
