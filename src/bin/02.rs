advent_of_code::solution!(2);

fn distance(n1: u32, n2: u32) -> i64 {
    return i64::from(n2) - i64::from(n1);
}

fn solve(levels: &Vec<&str>, retry: bool) -> bool {
    let mut d = 0;

    for i in 1..levels.len() {
        let cur_distance = distance(
            levels[i - 1].parse::<u32>().unwrap(),
            levels[i].parse::<u32>().unwrap(),
        );
        if cur_distance * d < 0 || cur_distance.abs() > 3 || cur_distance.abs() < 1 {
            if retry == false {
                return false;
            } else {
                let mut m1 = levels.to_vec();
                m1.remove(i);
                let mut m2 = levels.to_vec();
                m2.remove(i - 1);

                if i == 2 {
                    let mut m3 = levels.to_vec();
                    m3.remove(i - 2);
                    return solve(&m1, false) || solve(&m2, false) || solve(&m3, false);
                } else {
                    return solve(&m1, false) || solve(&m2, false);
                }
            }
        }
        d += cur_distance;
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = 0;

    for line in input.lines() {
        let levels: Vec<&str> = line.split(' ').collect();

        if solve(&levels, false) == true {
            res += 1;
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = 0;

    for line in input.lines() {
        let levels: Vec<&str> = line.split(' ').collect();

        if solve(&levels, true) == true {
            res += 1;
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
