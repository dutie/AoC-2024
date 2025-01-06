advent_of_code::solution!(10);
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

struct Grid {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let data: Vec<Vec<u8>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        
        let height = data.len();
        let width = data.first().map_or(0, |row| row.len());
        
        Grid {
            data,
            width,
            height,
        }
    }
    
    fn find_trailheads(&self) -> Vec<Position> {
        let mut trailheads = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.data[y][x] == 0 {
                    trailheads.push(Position { x, y });
                }
            }
        }
        trailheads
    }
    
    fn height_at(&self, pos: &Position) -> u8 {
        self.data[pos.y][pos.x]
    }
    
    fn get_neighbors(&self, pos: &Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let x = pos.x;
        let y = pos.y;
        
        if y > 0 {
            neighbors.push(Position { x, y: y - 1 });
        }
        if x > 0 {
            neighbors.push(Position { x: x - 1, y });
        }
        if y + 1 < self.height {
            neighbors.push(Position { x, y: y + 1 });
        }
        if x + 1 < self.width {
            neighbors.push(Position { x: x + 1, y });
        }
        
        neighbors
    }
}

fn find_paths(grid: &Grid, start: Position) -> HashSet<Position> {
    let mut reachable_nines = HashSet::new();
    let mut visited = HashSet::new();
    let mut stack = vec![(start, grid.height_at(&start))];
    visited.insert(start);
    
    while let Some((pos, height)) = stack.pop() {
        if grid.height_at(&pos) == 9 {
            reachable_nines.insert(pos);
            continue;
        }
        
        for neighbor in grid.get_neighbors(&pos) {
            if !visited.contains(&neighbor) && grid.height_at(&neighbor) == height + 1 {
                visited.insert(neighbor);
                stack.push((neighbor, grid.height_at(&neighbor)));
            }
        }
    }
    
    reachable_nines
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    let trailheads = grid.find_trailheads();
    
    let total_score: usize = trailheads
        .into_iter()
        .map(|start| find_paths(&grid, start).len())
        .sum();
    
    Some(total_score as u32)
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
