advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut output = Vec::new();
    for line in input.lines() {
        let (result, numbers) = line.split_once(": ").unwrap();
        output.push((
            result.parse().unwrap(),
            numbers.split(' ').map(|s| s.parse().unwrap()).collect(),
        ));
    }
    output
}

pub fn part_one(input: &str) -> Option<u64> {
    let input_parsed = parse_input(input);
    let mut sum = 0;

    for (result, numbers) in input_parsed {
        let n = numbers.len() - 1;
        for mask in 0..(1 << n) {
            let mut total = numbers[0];

            for i in 0..n {
                let next = numbers[i + 1];
                if total+next > result {
                    break;
                }

                if (mask >> i) & 1 == 1 {
                    total *= next;
                } else {
                    total += next;
                }
            }

            if total == result {
                sum += result;
                break;
            }
        }
    }

    Some(sum)
}
#[inline]
pub fn conc(a: u64, b: u64) -> u64{
    a as u64 * 10u64.pow(b.ilog10() + 1) + b as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    let input_parsed = parse_input(input);
    let mut sum = 0;

    for (result, numbers) in input_parsed {
        let n = numbers.len() - 1;

        for mask in 0..(3_u64.pow(n as u32)) {
            let mut total = numbers[0];

            for i in 0..n {
                if total > result {
                    break;
                }
                let next = numbers[i + 1];

                total = match (mask / 3_u64.pow(i as u32)) % 3 {
                    0 => total + next,
                    1 => total * next,
                    2 => conc(total, next),
                    _ => unreachable!(),
                }

            }

            if total == result {
                sum += result;
                break;
            }
        }
    }

    Some(sum)
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
