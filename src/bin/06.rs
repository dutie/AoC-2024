use std::{collections::HashSet, env::current_dir, io, usize};
advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn delta(&self) -> Position {
        match self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Right => Position { x: 1, y: 0 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
        }
    }
}

struct Map {
    grid: Vec<Vec<char>>,
    height: i32,
    width: i32,
}

impl Map {
    fn to_string_with_path(&self, path: &[(Position, Direction)]) -> String {
        let mut output = Vec::new();

        for y in 0..self.height {
            let mut row = String::new();
            for x in 0..self.width {
                let pos = Position { x, y };

                if let Some(idx) = path.iter().position(|(p, _)| *p == pos) {
                    let (_, dir) = path[idx];
                    let symbol = match dir {
                        Direction::Up => '^',
                        Direction::Right => '>',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                    };
                    row.push(symbol);
                } else {
                    row.push(self.grid[y as usize][x as usize]);
                }
            }
            output.push(row);
        }

        output.join("\n")
    }
    fn new(input: &str) -> (Self, Position, Direction) {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        // Get start pos
        let mut start_pos = Position { x: 0, y: 0 };
        let mut start_dir = Direction::Up;

        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                match cell {
                    '^' => {
                        start_pos = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        start_dir = Direction::Up;
                    }
                    '>' => {
                        start_pos = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        start_dir = Direction::Right;
                    }
                    'v' => {
                        start_pos = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        start_dir = Direction::Down;
                    }
                    '<' => {
                        start_pos = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        start_dir = Direction::Left;
                    }
                    _ => continue,
                }
            }
        }

        (
            Self {
                grid,
                height,
                width,
            },
            start_pos,
            start_dir,
        )
    }

    fn is_blocked(&self, pos: &Position) -> bool {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width || pos.y >= self.height {
            return true;
        }
        self.grid[pos.y as usize][pos.x as usize] == '#'
            || self.grid[pos.y as usize][pos.x as usize] == 'O'
    }

    fn is_out_of_bounds(&self, pos: &Position) -> bool {
        pos.x < 0 || pos.y < 0 || pos.x >= self.width || pos.y >= self.height
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let (map, mut current_pos, mut current_dir) = Map::new(input);
    let mut visited = HashSet::new();
    visited.insert(current_pos);

    loop {
        let delta = current_dir.delta();
        let next_pos = Position {
            x: current_pos.x + delta.x,
            y: current_pos.y + delta.y,
        };

        if map.is_out_of_bounds(&next_pos) {
            break;
        }
        if map.is_blocked(&next_pos) {
            current_dir = current_dir.turn_right();
        } else {
            current_pos = next_pos;
            visited.insert(current_pos);
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut map, start_pos, start_dir) = Map::new(input);
    let original_path = get_original_path(&map, start_pos, start_dir);
    let mut possible_positions = 0;
    let mut visited_obstacles = Vec::new();

    for pos in original_path {
        if pos == start_pos
            || map.grid[pos.y as usize][pos.x as usize] == '#'
            || map.grid[pos.y as usize][pos.x as usize] == 'O'
        {
            continue;
        }
        let tup = (pos.x, pos.y);
        if visited_obstacles.contains(&tup) {
            continue;
        }
        map.grid[pos.y as usize][pos.x as usize] = 'O';
        visited_obstacles.push(tup);

        if has_cycle(&map, start_pos, start_dir) {
            possible_positions += 1;
        }
        map.grid[pos.y as usize][pos.x as usize] = '.';
    }

    Some(possible_positions)
}

fn has_cycle_simple(map: &Map, start_pos: Position, start_dir: Direction) -> bool {
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;
    let mut visited = Vec::new();

    visited.push((current_pos, current_dir));

    loop {
        if let Some((new_pos, new_dir)) = make_move(map, current_pos, current_dir) {
            current_pos = new_pos;
            current_dir = new_dir;

            let current_state = (current_pos, current_dir);
            if let Some(start_idx) = visited.iter().position(|&state| state == current_state) {
                // Cycle Found:
                let cycle_length = visited.len() - start_idx;
                if cycle_length >= 2 {
                    let positions_in_cycle: HashSet<_> =
                        visited[start_idx..].iter().map(|(pos, _)| *pos).collect();
                    if positions_in_cycle.len() > 1 {
                        return true;
                    }
                }
            }

            visited.push(current_state);

            // Inf prevention
            if visited.len() > 10000 {
                return false;
            }
        } else {
            return false;
        }
    }
}

fn has_cycle(map: &Map, start_pos: Position, start_dir: Direction) -> bool {
    let mut tortoise_pos = start_pos;
    let mut tortoise_dir = start_dir;
    let mut hare_pos = start_pos;
    let mut hare_dir = start_dir;
    let mut tortoise_visited_len = 0;

    loop {
        // Slow move
        if let Some((new_pos, new_dir)) = make_move(map, tortoise_pos, tortoise_dir) {
            tortoise_pos = new_pos;
            tortoise_dir = new_dir;
            tortoise_visited_len += 1;
        } else {
            return false;
        }

        // Fast move
        for _ in 0..2 {
            if let Some((new_pos, new_dir)) = make_move(map, hare_pos, hare_dir) {
                hare_pos = new_pos;
                hare_dir = new_dir;
            } else {
                return false;
            }
        }

        // slow meets fast
        if tortoise_pos == hare_pos && tortoise_dir == hare_dir {
            return tortoise_visited_len >= 4;
        }
    }
}
fn make_move(map: &Map, pos: Position, dir: Direction) -> Option<(Position, Direction)> {
    let delta = dir.delta();
    let next_pos = Position {
        x: pos.x + delta.x,
        y: pos.y + delta.y,
    };

    if map.is_out_of_bounds(&next_pos) {
        return None;
    }

    if map.is_blocked(&next_pos) {
        // Turn right
        Some((pos, dir.turn_right()))
    } else {
        // Move forward
        Some((next_pos, dir))
    }
}

fn get_original_path(map: &Map, start_pos: Position, start_dir: Direction) -> Vec<Position> {
    let mut path = Vec::new();
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;

    loop {
        path.push(current_pos);

        if let Some((new_pos, new_dir)) = make_move(map, current_pos, current_dir) {
            current_pos = new_pos;
            current_dir = new_dir;
        } else {
            break;
        }
    }

    path
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
        assert_eq!(result, Some(5));
    }
}
