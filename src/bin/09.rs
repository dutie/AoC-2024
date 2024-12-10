advent_of_code::solution!(9);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let mut numbers: Vec<usize> = input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
        .collect();

    let mut front = 0;
    let mut back = numbers.len() - 1;
    let mut current_position = 0;
    let mut checksum = 0;

    while front <= back {
        let file_size = numbers[front];
        for i in 0..file_size {
            checksum += (current_position + i) * (front / 2);
        }
        current_position += file_size;
        front += 1;

        if front >= back {
            break;
        }
         // Process free space and move files from the end if possible
        let free_space = numbers[front];
        for _ in 0..free_space {
            if numbers[back] == 0 {
                back -= 2;
                if back < front {
                    break;
                }
            }

            // Move one block from the end file
            checksum += current_position * (back / 2);
            numbers[back] -= 1;
            current_position += 1;
        }

        current_position += numbers[front] - free_space;  // Adjust for any unused free space
        front += 1;
    }

    Some(checksum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut numbers: Vec<usize> = input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
        .collect();

    let mut front = 0;
    let mut back = numbers.len() - 1;
    let mut current_position = 0;
    let mut checksum = 0;

    while front <= back {
        let file_size = numbers[front];
        for i in 0..file_size {
            checksum += (current_position + i) * (front / 2);
        }
        current_position += file_size;
        front += 1;

        if front >= back {
            break;
        }
        // GARBAGE:
        let free_space = numbers[front];
        if free_space > 0 {
            // Look for files from the end that can fit in this free space
            while back > front && free_space >= numbers[back] && numbers[back] > 0 {
                // Move the entire file
                let file_size = numbers[back];
                for i in 0..file_size {
                    checksum += (current_position + i) * (back / 2);
                }
                current_position += file_size;

                // Mark this file as moved
                numbers[back] = 0;
                back -= 2;
            }
        }

        // Move past the free space section
        current_position += free_space;
        front += 1;
    }

    Some(checksum as u64)
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
