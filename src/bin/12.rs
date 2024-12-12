use std::collections::HashMap;

advent_of_code::solution!(12);

fn calculate_area_and_perimeter(
    visited: &mut HashMap<(usize, usize), bool>,
    matrix: &Vec<Vec<char>>,
    i: usize,
    j: usize,
) -> (u64, u64) {
    let mut area = 1;
    let mut perimeter = 4;
    visited.insert((i, j), true);

    if i > 0 && matrix[i - 1][j] == matrix[i][j] {
        perimeter -= 1;
    }

    if i > 0 && visited.contains_key(&(i - 1, j)) == false && matrix[i - 1][j] == matrix[i][j] {
        let (n_area, n_perimeter) = calculate_area_and_perimeter(visited, matrix, i - 1, j);
        area += n_area;
        perimeter += n_perimeter;
    }

    if i + 1 < matrix.len() && matrix[i + 1][j] == matrix[i][j] {
        perimeter -= 1;
    }

    if i + 1 < matrix.len()
        && visited.contains_key(&(i + 1, j)) == false
        && matrix[i + 1][j] == matrix[i][j]
    {
        let (n_area, n_perimeter) = calculate_area_and_perimeter(visited, matrix, i + 1, j);
        area += n_area;
        perimeter += n_perimeter;
    }

    if j > 0 && matrix[i][j - 1] == matrix[i][j] {
        perimeter -= 1;
    }

    if j > 0 && visited.contains_key(&(i, j - 1)) == false && matrix[i][j - 1] == matrix[i][j] {
        let (n_area, n_perimeter) = calculate_area_and_perimeter(visited, matrix, i, j - 1);
        area += n_area;
        perimeter += n_perimeter;
    }

    if j + 1 < matrix[0].len() && matrix[i][j + 1] == matrix[i][j] {
        perimeter -= 1;
    }

    if j + 1 < matrix[0].len()
        && visited.contains_key(&(i, j + 1)) == false
        && matrix[i][j + 1] == matrix[i][j]
    {
        let (n_area, n_perimeter) = calculate_area_and_perimeter(visited, matrix, i, j + 1);
        area += n_area;
        perimeter += n_perimeter;
    }

    (area, perimeter)
}

fn matrix_val(matrix: &Vec<Vec<char>>, i: isize, j: isize) -> char {
    if i < 0 || j < 0 || i >= matrix.len() as isize || j >= matrix[0].len() as isize {
        return ' ';
    }

    matrix[i as usize][j as usize]
}

fn calculate_area_and_corners(
    visited: &mut HashMap<(usize, usize), bool>,
    matrix: &Vec<Vec<char>>,
    i: usize,
    j: usize,
) -> (u64, u64) {
    let mut area = 1;
    let mut corners = 0;
    visited.insert((i, j), true);

    let c = matrix[i][j];
    let c_up = matrix_val(matrix, i as isize - 1, j as isize);
    let c_down = matrix_val(matrix, i as isize + 1, j as isize);
    let c_left = matrix_val(matrix, i as isize, j as isize - 1);
    let c_right = matrix_val(matrix, i as isize, j as isize + 1);

    if c != c_up && c != c_right {
        corners += 1;
    }
    if c != c_right && c != c_down {
        corners += 1;
    }
    if c != c_down && c != c_left {
        corners += 1;
    }
    if c != c_left && c != c_up {
        corners += 1;
    }

    if matrix_val(matrix, i as isize - 1, j as isize - 1) != c && c == c_up && c == c_left {
        corners += 1;
    }
    if matrix_val(matrix, i as isize - 1, j as isize + 1) != c && c == c_up && c == c_right {
        corners += 1;
    }
    if matrix_val(matrix, i as isize + 1, j as isize + 1) != c && c == c_down && c == c_right {
        corners += 1;
    }
    if matrix_val(matrix, i as isize + 1, j as isize - 1) != c && c == c_down && c == c_left {
        corners += 1;
    }

    if i > 0 && visited.contains_key(&(i - 1, j)) == false && matrix[i - 1][j] == matrix[i][j] {
        let (n_area, n_corners) = calculate_area_and_corners(visited, matrix, i - 1, j);
        area += n_area;
        corners += n_corners;
    }

    if i + 1 < matrix.len()
        && visited.contains_key(&(i + 1, j)) == false
        && matrix[i + 1][j] == matrix[i][j]
    {
        let (n_area, n_corners) = calculate_area_and_corners(visited, matrix, i + 1, j);
        area += n_area;
        corners += n_corners;
    }

    if j > 0 && visited.contains_key(&(i, j - 1)) == false && matrix[i][j - 1] == matrix[i][j] {
        let (n_area, n_corners) = calculate_area_and_corners(visited, matrix, i, j - 1);
        area += n_area;
        corners += n_corners;
    }

    if j + 1 < matrix[0].len()
        && visited.contains_key(&(i, j + 1)) == false
        && matrix[i][j + 1] == matrix[i][j]
    {
        let (n_area, n_corners) = calculate_area_and_corners(visited, matrix, i, j + 1);
        area += n_area;
        corners += n_corners;
    }

    (area, corners)
}

pub fn part_one(input: &str) -> Option<u64> {
    let matrix: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut price = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if visited.contains_key(&(i, j)) {
                continue;
            }

            let (area, perimeter) = calculate_area_and_perimeter(&mut visited, &matrix, i, j);
            price += area * perimeter;
        }
    }

    Some(price)
}

pub fn part_two(input: &str) -> Option<u64> {
    let matrix: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut price = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if visited.contains_key(&(i, j)) {
                continue;
            }

            let (area, corners) = calculate_area_and_corners(&mut visited, &matrix, i, j);
            price += area * corners;
        }
    }

    Some(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
