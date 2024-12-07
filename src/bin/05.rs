use regex::Regex;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let rules = sections.next()?;
    let updates = sections.next()?;

    let rule_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let rules: Vec<(u32, u32)> = rules
        .lines()
        .filter_map(|line| {
            let caps = rule_regex.captures(line)?;
            Some((caps[1].parse().unwrap(), caps[2].parse().unwrap()))
        })
        .collect();

    let res = updates
        .lines()
        .filter_map(|update| {
            let numbers: Vec<u32> = update.split(',').filter_map(|n| n.parse().ok()).collect();
            let is_valid = rules.iter().all(|&(before, after)| {
                if let (Some(pos1), Some(pos2)) = (
                    numbers.iter().position(|&x| x == before),
                    numbers.iter().position(|&x| x == after),
                ) {
                    pos1 < pos2
                } else {
                    true
                }
            });

            if is_valid {
                Some(numbers[numbers.len() / 2])
            } else {
                None
            }
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let rules = sections.next()?;
    let updates = sections.next()?;

    let rule_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let rules: Vec<(u32, u32)> = rules
        .lines()
        .filter_map(|line| {
            let caps = rule_regex.captures(line)?;
            Some((caps[1].parse().unwrap(), caps[2].parse().unwrap()))
        })
        .collect();

    let res = updates
        .lines()
        .filter_map(|update| {
            let mut numbers: Vec<u32> = update.split(',').filter_map(|n| n.parse().ok()).collect();
            let is_valid = rules.iter().all(|&(before, after)| {
                if let (Some(pos1), Some(pos2)) = (
                    numbers.iter().position(|&x| x == before),
                    numbers.iter().position(|&x| x == after),
                ) {
                    pos1 < pos2
                } else {
                    true
                }
            });

            if is_valid {
                None
            } else {
                // Sort the numbers according to the rules
                numbers.sort_by(|&a, &b| {
                    for &(before, after) in rules.iter() {
                        if a == before && b == after {
                            return std::cmp::Ordering::Less;
                        }
                        if a == after && b == before {
                            return std::cmp::Ordering::Greater;
                        }
                    }
                    b.cmp(&a) // Default to descending order for unrelated numbers
                });

                // Find the middle number
                let middle_idx = numbers.len() / 2;
                Some(numbers[middle_idx])
            }
        })
        .sum();

    Some(res)
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
