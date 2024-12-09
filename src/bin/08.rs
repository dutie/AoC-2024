use itertools::Itertools;
use std::collections::{HashMap, HashSet};
advent_of_code::solution!(8);

pub fn elle_ma(input: &str) -> (HashMap<char, Vec<(isize, isize)>>, (isize, isize)) {
    let mut frequencies = HashMap::new();
    let mut max_y = 0;
    let mut max_x = 0;

    for (y, line) in input.lines().enumerate() {
        max_y = max_y.max(y);
        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x);
            if c != '.' {
                frequencies
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((x as isize, y as isize));
            }
        }
    }

    (frequencies, (max_x as isize, max_y as isize))
}

pub fn contains(point: (isize, isize), width: isize, height: isize) -> bool {
    point.0 >= 0 && point.0 <= width && point.1 >= 0 && point.1 <= height
}

pub fn part_one(input: &str) -> Option<u32> {
    let (frequencies, (max_x, max_y)) = elle_ma(input);
    let mut antinodes = HashSet::new();

    for (_freq, points) in frequencies {
        for (first, second) in points.iter().tuple_combinations() {
            let ffirst = (
                first.0 - (second.0 - first.0),
                first.1 - (second.1 - first.1),
            );
            if contains(ffirst, max_x, max_y) {
                antinodes.insert(ffirst);
            }
            let ssecond = (
                second.0 - (first.0 - second.0),
                second.1 - (first.1 - second.1),
            );
            if contains(ssecond, max_x, max_y) {
                antinodes.insert(ssecond);
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (frequencies, (max_x, max_y)) = elle_ma(input);
    let mut antinodes = HashSet::new();

    for (_, points) in frequencies {
        for (first, second) in points.iter().tuple_combinations() {
            let mut curr_first = *first;
            let mut curr_second = *second;

            let forward_vector = (second.0 - first.0, second.1 - first.1);
            let backward_vector = (first.0 - second.0, first.1 - second.1);

            while contains(curr_first, max_x, max_y) {
                antinodes.insert(curr_first);
                curr_first = (
                    curr_first.0 + forward_vector.0,
                    curr_first.1 + forward_vector.1,
                );
            }

            while contains(curr_second, max_x, max_y) {
                antinodes.insert(curr_second);
                curr_second = (
                    curr_second.0 + backward_vector.0,
                    curr_second.1 + backward_vector.1,
                );
            }
        }
    }

    Some(antinodes.len() as u32)
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
