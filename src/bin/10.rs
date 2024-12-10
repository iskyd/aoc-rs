use std::collections::HashMap;

advent_of_code::solution!(10);

fn show_map(matrix: Vec<Vec<u32>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            print!("{}", matrix[i][j]);
        }
        println!("");
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let trailheads: Vec<(usize, usize)> = matrix
        .iter()
        .enumerate()
        .flat_map(|(row_id, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, &col)| {
                if col == 0 {
                    Some((row_id, col_idx))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut res = 0;
    for (x, y) in trailheads {
        let mut stack: Vec<(usize, usize)> = vec![];
        stack.push((x, y));
        let mut existing: HashMap<(usize, usize), bool> = HashMap::new();
        while stack.len() > 0 {
            let (cur_x, cur_y) = stack.pop().unwrap();
            let cur_val = matrix[cur_x][cur_y];
            if cur_val == 9 {
                existing.insert((cur_x, cur_y), true);
                continue;
            }
            if cur_x > 0 && matrix[cur_x - 1][cur_y] == cur_val + 1 {
                stack.push((cur_x - 1, cur_y));
            }
            if cur_y > 0 && matrix[cur_x][cur_y - 1] == cur_val + 1 {
                stack.push((cur_x, cur_y - 1));
            }
            if cur_x + 1 < matrix.len() && matrix[cur_x + 1][cur_y] == cur_val + 1 {
                stack.push((cur_x + 1, cur_y));
            }
            if cur_y + 1 < matrix[0].len() && matrix[cur_x][cur_y + 1] == cur_val + 1 {
                stack.push((cur_x, cur_y + 1));
            }
        }
        res += existing.len() as u32;
        existing.clear();
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let trailheads: Vec<(usize, usize)> = matrix
        .iter()
        .enumerate()
        .flat_map(|(row_id, row)| {
            row.iter().enumerate().filter_map(move |(col_idx, &col)| {
                if col == 0 {
                    Some((row_id, col_idx))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut res = 0;
    for (x, y) in trailheads {
        let mut stack: Vec<(usize, usize)> = vec![];
        stack.push((x, y));
        while stack.len() > 0 {
            let (cur_x, cur_y) = stack.pop().unwrap();
            let cur_val = matrix[cur_x][cur_y];
            if cur_val == 9 {
                res += 1;
                continue;
            }
            if cur_x > 0 && matrix[cur_x - 1][cur_y] == cur_val + 1 {
                stack.push((cur_x - 1, cur_y));
            }
            if cur_y > 0 && matrix[cur_x][cur_y - 1] == cur_val + 1 {
                stack.push((cur_x, cur_y - 1));
            }
            if cur_x + 1 < matrix.len() && matrix[cur_x + 1][cur_y] == cur_val + 1 {
                stack.push((cur_x + 1, cur_y));
            }
            if cur_y + 1 < matrix[0].len() && matrix[cur_x][cur_y + 1] == cur_val + 1 {
                stack.push((cur_x, cur_y + 1));
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
