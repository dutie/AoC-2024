advent_of_code::solution!(4);

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),  // right
    (1, 0),  // down
    (1, 1),  // diagonal down-right
    (-1, 1), // diagonal up-right
    (0, -1), // left
    (-1, 0), // up
    (-1, -1),// diagonal up-left
    (1, -1), // diagonal down-left
];

fn count_xmas_from_position(row: i32, col: i32, grid: &[Vec<char>], target: &[char]) -> u32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    let mut count = 0;
    for &(dr, dc) in &DIRECTIONS {
        let mut valid = true;
        let mut r = row;
        let mut c = col;
        
        for &ch in target.iter().skip(1) {  // skip first char as we already matched 'X'
            r += dr;
            c += dc;
            
            if r < 0 || r >= rows || c < 0 || c >= cols || 
               grid[r as usize][c as usize] != ch {
                valid = false;
                break;
            }
        }
        
        if valid {
            count += 1;
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let target = ['X', 'M', 'A', 'S'];
    let mut count = 0;
    
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == target[0] {
                count += count_xmas_from_position(row as i32, col as i32, &grid, &target);
            }
        }
    }
    
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let mut count = 0;

    for row in 1..grid.len()-1 {
        for col in 1..grid[0].len()-1 {
            if grid[row][col] == 'A' {
                let diag_down = [grid[row-1][col-1], grid[row+1][col+1]];
                let diag_up   = [grid[row-1][col+1], grid[row+1][col-1]];

//                if diag_down == "MS" || diag_down == "SM" ||
//                  diag_up == "MS" || diag_up == "SM" {
//                    count += 1;
//                }
//                let pattern: String = [diag_down[0], diag_down[1], diag_up[0], diag_up[1]]
//                    .iter()
//                    .collect();
                
//                if matches!(pattern.as_str(), "MMSS" | "MSMS" | "SMSM" | "SSMM") {
//                    count += 1;
//                }
                let is_mas = |chars: &[char]| {
                    chars == ['M', 'S'] || chars == ['S', 'M']
                };
                
                if is_mas(&diag_down) && is_mas(&diag_up) {
                    count += 1;
                }                
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
