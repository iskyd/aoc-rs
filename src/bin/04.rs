advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut res = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'X' {
                if j + 3 < matrix[0].len() {
                    if matrix[i][j + 1] == 'M' && matrix[i][j + 2] == 'A' && matrix[i][j + 3] == 'S'
                    {
                        res += 1;
                    }
                }
                if i + 3 < matrix.len() {
                    if matrix[i + 1][j] == 'M' && matrix[i + 2][j] == 'A' && matrix[i + 3][j] == 'S'
                    {
                        res += 1;
                    }
                }
                if j + 3 < matrix[0].len() && i + 3 < matrix.len() {
                    if matrix[i + 1][j + 1] == 'M'
                        && matrix[i + 2][j + 2] == 'A'
                        && matrix[i + 3][j + 3] == 'S'
                    {
                        res += 1;
                    }
                }
                if j >= 3 && i + 3 < matrix.len() {
                    if matrix[i + 1][j - 1] == 'M'
                        && matrix[i + 2][j - 2] == 'A'
                        && matrix[i + 3][j - 3] == 'S'
                    {
                        res += 1;
                    }
                }
            } else if matrix[i][j] == 'S' {
                if j + 3 < matrix[0].len() {
                    if matrix[i][j + 1] == 'A' && matrix[i][j + 2] == 'M' && matrix[i][j + 3] == 'X'
                    {
                        res += 1;
                    }
                }
                if i + 3 < matrix.len() {
                    if matrix[i + 1][j] == 'A' && matrix[i + 2][j] == 'M' && matrix[i + 3][j] == 'X'
                    {
                        res += 1;
                    }
                }
                if j + 3 < matrix[0].len() && i + 3 < matrix.len() {
                    if matrix[i + 1][j + 1] == 'A'
                        && matrix[i + 2][j + 2] == 'M'
                        && matrix[i + 3][j + 3] == 'X'
                    {
                        res += 1;
                    }
                }
                if j >= 3 && i + 3 < matrix.len() {
                    if matrix[i + 1][j - 1] == 'A'
                        && matrix[i + 2][j - 2] == 'M'
                        && matrix[i + 3][j - 3] == 'X'
                    {
                        res += 1;
                    }
                }
            }
        }
    }

    Some(res)
}

fn check_chars(c1: char, c2: char, c3: char, c4: char) -> bool {
    return (c1 == 'S' || c1 == 'M')
        && (c2 == 'S' || c2 == 'M')
        && (c3 == 'S' || c3 == 'M')
        && (c4 == 'S' || c4 == 'M');
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut res = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            match matrix[i][j] {
                'A' => {
                    if (i >= 1)
                        && (j >= 1)
                        && (i + 1 < matrix.len())
                        && (j + 1 < matrix[0].len())
                        && check_chars(
                            matrix[i - 1][j - 1],
                            matrix[i - 1][j + 1],
                            matrix[i + 1][j - 1],
                            matrix[i + 1][j + 1],
                        )
                    {
                        if (matrix[i - 1][j - 1] == matrix[i - 1][j + 1]
                            && matrix[i + 1][j - 1] == matrix[i + 1][j + 1]
                            && matrix[i - 1][j - 1] != matrix[i + 1][j - 1])
                            || (matrix[i - 1][j - 1] == matrix[i + 1][j - 1]
                                && matrix[i - 1][j + 1] == matrix[i + 1][j + 1]
                                && matrix[i - 1][j - 1] != matrix[i - 1][j + 1])
                        {
                            res += 1;
                        }
                    }
                }
                _ => {}
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
