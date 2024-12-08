use std::collections::HashMap;

advent_of_code::solution!(8);

fn show_map(matrix: Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            print!("{}", matrix[i][j]);
        }
        println!("");
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let mut hm: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] != '.' {
                match hm.get_mut(&matrix[i][j]) {
                    Some(e) => {
                        e.push((i, j));
                    }
                    None => {
                        hm.insert(matrix[i][j], vec![(i, j)]);
                    }
                }
            }
        }
    }

    let mut antinodes: HashMap<(usize, usize), bool> = HashMap::new();

    for (k, v) in hm.iter() {
        for (x1, y1) in v {
            let others: Vec<(usize, usize)> = v
                .iter()
                .filter(|(c1, c2)| c1 != x1 && c2 != y1)
                .cloned()
                .collect();

            // find all the antinodes
            for (x2, y2) in others {
                let d1: isize = x2 as isize - *x1 as isize;
                let d2: isize = y2 as isize - *y1 as isize;

                let a1x = *x1 as isize - d1;
                let a1y = *y1 as isize - d2;

                let a2x = x2 as isize + d1;
                let a2y = y2 as isize + d2;

                if a1x >= 0
                    && a1y >= 0
                    && a1x < matrix.len() as isize
                    && a1y < matrix[0].len() as isize
                {
                    antinodes.insert((a1x as usize, a1y as usize), true);
                }

                if a2x >= 0
                    && a2y >= 0
                    && a2x < matrix.len() as isize
                    && a2y < matrix[0].len() as isize
                {
                    antinodes.insert((a2x as usize, a2y as usize), true);
                }
            }
        }
    }

    //let mut m2 = matrix.clone();
    //for (k, _) in antinodes.iter() {
    //    m2[k.0][k.1] = '#';
    //}
    //
    //show_map(m2);

    Some(antinodes.len())
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
