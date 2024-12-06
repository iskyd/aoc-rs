use std::collections::HashMap;

advent_of_code::solution!(6);

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut guard_position: (usize, usize) = matrix
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter().enumerate().find_map(|(col_idx, &c)| {
                if c == '^' {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut direction = Direction::UP;

    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    visited.insert((guard_position.0, guard_position.1), true);

    loop {
        let next_cordinates: (usize, usize) = match direction {
            Direction::UP => {
                if guard_position.0 == 0 {
                    break;
                }
                (guard_position.0 - 1, guard_position.1)
            }
            Direction::DOWN => (guard_position.0 + 1, guard_position.1),
            Direction::LEFT => {
                if guard_position.1 == 0 {
                    break;
                }
                (guard_position.0, guard_position.1 - 1)
            }
            Direction::RIGHT => (guard_position.0, guard_position.1 + 1),
        };

        if next_cordinates.0 >= matrix.len() || next_cordinates.1 >= matrix[0].len() {
            break;
        }

        if matrix[next_cordinates.0][next_cordinates.1] == '#' {
            direction = match direction {
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
            }
        } else {
            guard_position.0 = next_cordinates.0;
            guard_position.1 = next_cordinates.1;
            match visited.get(&guard_position) {
                Some(_) => {}
                None => {
                    visited.insert(guard_position, true);
                }
            }
        }
    }

    Some(visited.len() as u32)
}

fn check_loops(mut guard_position: (usize, usize), matrix: Vec<Vec<char>>) -> bool {
    let mut direction = Direction::UP;

    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    visited.insert((guard_position.0, guard_position.1), true);
    let mut total_moves = 0; // used to check if a loop occurred. Reset when we visit a new position.

    loop {
        let next_cordinates: (usize, usize) = match direction {
            Direction::UP => {
                if guard_position.0 == 0 {
                    break;
                }
                (guard_position.0 - 1, guard_position.1)
            }
            Direction::DOWN => (guard_position.0 + 1, guard_position.1),
            Direction::LEFT => {
                if guard_position.1 == 0 {
                    break;
                }
                (guard_position.0, guard_position.1 - 1)
            }
            Direction::RIGHT => (guard_position.0, guard_position.1 + 1),
        };

        if next_cordinates.0 >= matrix.len() || next_cordinates.1 >= matrix[0].len() {
            break;
        }

        if matrix[next_cordinates.0][next_cordinates.1] == '#' {
            direction = match direction {
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
            }
        } else {
            guard_position.0 = next_cordinates.0;
            guard_position.1 = next_cordinates.1;
            match visited.get(&guard_position) {
                Some(_) => {
                    total_moves += 1;
                }
                None => {
                    total_moves = 0;
                    visited.insert(guard_position, true);
                }
            }
        }

        if total_moves >= matrix.len() {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let guard_position: (usize, usize) = matrix
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter().enumerate().find_map(|(col_idx, &c)| {
                if c == '^' {
                    Some((row_idx, col_idx))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut res = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if i == guard_position.0 && j == guard_position.1 {
                continue;
            }

            let mut modified_matrix = matrix.clone();
            modified_matrix[i][j] = '#';

            if check_loops(guard_position, modified_matrix) == true {
                res += 1;
            }
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
