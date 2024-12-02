advent_of_code::solution!(2);

fn distance(n1: u32, n2: u32) -> i64 {
    return i64::from(n2) - i64::from(n1);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = 0;

    for line in input.lines() {
        let levels: Vec<&str> = line.split(' ').collect();

        if solve(&levels) == true {
            res += 1;
        }
    }

    Some(res)
}

fn solve(levels: &Vec<&str>) -> bool {
    let mut d = 0;

    for i in 1..levels.len() {
        let cur_distance = distance(
            levels[i - 1].parse::<u32>().unwrap(),
            levels[i].parse::<u32>().unwrap(),
        );
        if cur_distance * d < 0 || cur_distance.abs() > 3 || cur_distance.abs() < 1 {
            return false;
        }
        d += cur_distance;
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = 0;

    for line in input.lines() {
        let levels: Vec<&str> = line.split(' ').collect();

        if solve(&levels) == true {
            res += 1;
        } else {
            for i in 0..levels.len() {
                let mut modified = levels.to_vec();
                modified.remove(i);
                if solve(&modified) == true {
                    res += 1;
                    break;
                }
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
