advent_of_code::solution!(2);

fn check_sequence(report: &[u32]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let ascending = report.windows(2).all(|w| w[0] <= w[1]);
    let descending = report.windows(2).all(|w| w[0] >= w[1]);

    (ascending || descending) &&
        report.windows(2).all(|w| {
            let diff = w[0].abs_diff(w[1]);
            diff >= 1 && diff <= 3
        })
}

fn check_with_dampener(report: &[u32]) -> bool {
    if check_sequence(report) {
        return true;
    }

    (0..report.len()).any(|i| {
        let mut modified = report.to_vec();
        modified.remove(i);
        check_sequence(&modified)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = input.lines()
        .filter(|line| {
            let report: Vec<u32> = line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            check_sequence(&report)
        })
        .count();
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input.lines()
        .filter(|line| {
            let report: Vec<u32> = line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            check_with_dampener(&report)
        })
        .count();
    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

