use std::collections::HashMap;
use std::collections::VecDeque;

advent_of_code::solution!(9);

#[derive(Clone, Copy)]
enum DiskSpace {
    Data(u32),
    Free,
}

fn calculate_checksum(disk_map: Vec<DiskSpace>) -> usize {
    let mut checksum = 0;
    for i in 0..disk_map.len() {
        match disk_map[i] {
            DiskSpace::Free => {
                continue;
            }
            DiskSpace::Data(d) => {
                checksum += i * d as usize;
            }
        }
    }

    checksum
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk_map: Vec<DiskSpace> = vec![];
    let mut file_id = 0;
    for (i, c) in input.chars().enumerate() {
        if c.is_numeric() == false {
            break;
        }
        if i % 2 == 0 {
            let total_data = c.to_digit(10).unwrap() as usize;
            let data_map: Vec<DiskSpace> = vec![DiskSpace::Data(file_id); total_data];
            disk_map.extend(data_map);
            file_id += 1;
        } else {
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
            DiskSpace::Data(_) => {
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

    Some(calculate_checksum(disk_map))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut disk_map: Vec<DiskSpace> = vec![];
    let mut files_map: HashMap<u32, (usize, usize)> = HashMap::new();
    let mut free_map: Vec<(usize, usize)> = vec![];
    let mut file_id = 0;
    for (i, c) in input.chars().enumerate() {
        if c.is_numeric() == false {
            break;
        }
        if i % 2 == 0 {
            let total_data = c.to_digit(10).unwrap() as usize;
            let data_map: Vec<DiskSpace> = vec![DiskSpace::Data(file_id); total_data];
            files_map.insert(file_id, (disk_map.len(), disk_map.len() + total_data));
            disk_map.extend(data_map);
            file_id += 1;
        } else {
            let total_free_space = c.to_digit(10).unwrap() as usize;
            free_map.push((disk_map.len(), disk_map.len() + total_free_space));
            disk_map.extend(vec![DiskSpace::Free; total_free_space]);
        }
    }

    let mut j = disk_map.len() - 1;
    while j > 0 || free_map.len() > 0 {
        let last_space = disk_map[j];

        match last_space {
            DiskSpace::Free => {
                j -= 1;
            }
            DiskSpace::Data(d) => {
                let file_info = files_map.get(&d).unwrap();
                let file_length = file_info.1 - file_info.0;
                for z in 0..free_map.len() {
                    let f = free_map[z];
                    if f.1 - f.0 >= file_length && j > f.0 {
                        // move here
                        for k in 0..file_length {
                            disk_map[f.0 + k] = DiskSpace::Data(d);
                        }
                        for k in 0..file_length {
                            disk_map[j - k] = DiskSpace::Free;
                        }

                        if f.1 - f.1 == file_length {
                            free_map.remove(z);
                        } else {
                            free_map[z] = (f.0 + file_length, f.1);
                        }

                        break;
                    }
                }

                if j < file_length {
                    break;
                }
                j -= file_length;
            }
        }
    }

    Some(calculate_checksum(disk_map))
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
        assert_eq!(result, Some(2858));
    }
}
