advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut res: u32 = 0;
    while i < chars.len() {
        if i + 4 >= chars.len() {
            break;
        }
        let c = String::from_iter(&chars[i..i + 4]);
        match c.as_str() {
            "mul(" => {
                //
                let mut start = i + 4;
                let mut j = start;
                let mut n1: u32 = 0;
                let mut n2: u32 = 0;
                loop {
                    if j > chars.len() {
                        i = j;
                        break;
                    }

                    if chars[j].is_digit(10) {
                        j += 1;
                    } else if chars[j] == ',' {
                        // convert the first part in digit it is n1
                        n1 = String::from_iter(&chars[start..j]).parse::<u32>().unwrap();
                        start = j + 1;
                        j += 1;
                    } else if chars[j] == ')' {
                        n2 = String::from_iter(&chars[start..j]).parse::<u32>().unwrap();
                        res += n1 * n2;
                        i = j + 1;
                        break;
                    } else {
                        i = j;
                        break;
                    }
                }
            }
            _ => {
                i += 1;
            }
        }
    }
    Some(res)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
