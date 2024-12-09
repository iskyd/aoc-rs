advent_of_code::solution!(9);

#[derive(Clone, Copy)]
enum DiskSpace {
    Data(u32),
    Free,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk_map: Vec<DiskSpace> = vec![];
    let mut file_id = 0;
    for (i, c) in input.chars().enumerate() {
        if c.is_numeric() == false {
            break;
        }
        if i % 2 == 0 {
            // data
            let total_data = c.to_digit(10).unwrap() as usize;
            let data_map: Vec<DiskSpace> = vec![DiskSpace::Data(file_id); total_data];
            disk_map.extend(data_map);
            file_id += 1;
        } else {
            // free space
            let total_free_space = c.to_digit(10).unwrap() as usize;
            let free_map: Vec<DiskSpace> = vec![DiskSpace::Free; total_free_space];
            disk_map.extend(free_map);
        }
    }

    let mut i = 0;
    let mut j = disk_map.len() - 1;
    while i < j {
        let first_space = disk_map[i];
        let last_space = disk_map[j];

        match first_space {
            DiskSpace::Data(file_id) => {
                i += 1;
            }
            DiskSpace::Free => match last_space {
                DiskSpace::Data(_) => {
                    disk_map[i] = disk_map[j];
                    disk_map[j] = DiskSpace::Free;
                    i += 1;
                    j -= 1;
                }
                DiskSpace::Free => {
                    j -= 1;
                }
            },
        }
    }

    let mut checksum = 0;
    for i in 0..disk_map.len() {
        match disk_map[i] {
            DiskSpace::Free => {
                break;
            }
            DiskSpace::Data(d) => {
                checksum += i * d as usize;
            }
        }
    }

    Some(checksum)
}

//pub fn part_one(input: &str) -> Option<usize> {
//    let mut disk_map: Vec<char> = vec![];
//    let mut file_id = 0;
//    for (i, c) in input.chars().enumerate() {
//        if c.is_numeric() == false {
//            break;
//        }
//        if i % 2 == 0 {
//            // data
//            let total_data = c.to_digit(10).unwrap() as usize;
//            let data_map: Vec<char> = vec![char::from_digit(file_id, 10).unwrap(); total_data];
//            disk_map.extend(&data_map);
//            file_id += 1;
//        } else {
//            // free space
//            let total_free_space = c.to_digit(10).unwrap() as usize;
//            let free_map: Vec<char> = vec!['.'; total_free_space];
//            disk_map.extend(free_map);
//        }
//    }
//
//    let mut i = 0;
//    let mut j = disk_map.len() - 1;
//    while i < j {
//        if disk_map[i] == '.' && disk_map[j].is_numeric() == true {
//            disk_map[i] = disk_map[j];
//            disk_map[j] = '.';
//            i += 1;
//            j -= 1;
//        } else if disk_map[i] == '.' && disk_map[j] == '.' {
//            j -= 1;
//        } else if disk_map[i].is_numeric() {
//            i += 1;
//        } else {
//        }
//    }
//
//    let mut checksum = 0;
//    i = 0;
//    for i in 0..disk_map.len() {
//        if disk_map[i] == '.' {
//            break;
//        }
//
//        checksum += i * disk_map[i].to_digit(10).unwrap() as usize;
//    }
//
//    Some(checksum)
//}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
