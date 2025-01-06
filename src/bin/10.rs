advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position(usize);

struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut data = Vec::with_capacity(input.len() / 2);
        let mut width = 0;
        let mut height = 0;
        
        for line in input.lines() {
            if line.is_empty() { continue; }
            if width == 0 { width = line.len(); }
            height += 1;
            data.extend(line.bytes().map(|b| b - b'0'));
        }
        
        Grid {
            data,
            width,
            height,
        }
    }
    
    #[inline(always)]
    fn idx_to_pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }
    
    fn find_trailheads(&self) -> Vec<Position> {
        self.data.iter()
            .enumerate()
            .filter(|&(_, &height)| height == 0)
            .map(|(idx, _)| Position(idx))
            .collect()
    }
    
    #[inline(always)]
    fn height_at(&self, pos: Position) -> u8 {
        self.data[pos.0]
    }
    
    #[inline(always)]
    fn get_neighbors(&self, pos: Position) -> impl Iterator<Item = Position> + '_ {
        let (x, y) = self.idx_to_pos(pos.0);
        let width = self.width;
        let idx = pos.0;
        
        [
            if y > 0 { Some(idx - width) } else { None },
            if x > 0 { Some(idx - 1) } else { None },
            if y + 1 < self.height { Some(idx + width) } else { None },
            if x + 1 < self.width { Some(idx + 1) } else { None }
        ].into_iter()
        .flatten()
        .map(Position)
    }
}

fn find_paths(grid: &Grid, start: Position) -> usize {
    let mut reachable_nines = 0;
    let mut visited = vec![false; grid.data.len()];
    let mut stack = Vec::with_capacity(grid.width.max(grid.height));
    
    visited[start.0] = true;
    stack.push((start, grid.height_at(start)));
    
    while let Some((pos, height)) = stack.pop() {
        if grid.height_at(pos) == 9 {
            reachable_nines += 1;
            continue;
        }
        
        for neighbor in grid.get_neighbors(pos) {
            if !visited[neighbor.0] && grid.height_at(neighbor) == height + 1 {
                visited[neighbor.0] = true;
                stack.push((neighbor, grid.height_at(neighbor)));
            }
        }
    }
    
    reachable_nines
}

fn find_ratings(grid: &Grid, start: Position) -> usize {
    let mut reachable_nines = 0;
    let mut stack = Vec::with_capacity(grid.width.max(grid.height));
    
    stack.push((start, grid.height_at(start)));
    
    while let Some((pos, height)) = stack.pop() {
        if grid.height_at(pos) == 9 {
            reachable_nines += 1;
            continue;
        }
        
        for neighbor in grid.get_neighbors(pos) {
            if grid.height_at(neighbor) == height + 1 {
                stack.push((neighbor, grid.height_at(neighbor)));
            }
        }
    }
    
    reachable_nines
}
pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    
    let total_score = grid.find_trailheads()
        .into_iter()
        .map(|start| find_paths(&grid, start))
        .sum::<usize>();
    
    Some(total_score as u32)
}
pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);
    
    let total_score = grid.find_trailheads()
        .into_iter()
        .map(|start| find_ratings(&grid, start))
        .sum::<usize>();
    
    Some(total_score as u32)
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
