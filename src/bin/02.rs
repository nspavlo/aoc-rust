advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0u64;

    for line in input.lines() {
        for ids in line.split(',') {
            let nums: Vec<u64> = ids.split('-').filter_map(|s| s.parse().ok()).collect();
            let mut matches = Vec::<String>::new();

            if let [first, second] = nums.as_slice() {
                for num in *first..=*second {
                    let str = num.to_string();
                    let (part1, part2) = str.split_at(str.len() / 2);

                    if part1 == part2 {
                        matches.push(str);
                        count += num;
                    }
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count = 0u64;

    for line in input.lines() {
        for ids in line.split(',') {
            let nums: Vec<u64> = ids.split('-').filter_map(|s| s.parse().ok()).collect();

            if let [first, second] = nums.as_slice() {
                for num in *first..=*second {
                    let str = num.to_string();
                    let length = str.len();

                    for chunk_size in 1..length {
                        if length % chunk_size == 0 {
                            let pattern = &str[0..chunk_size];
                            let expected = pattern.repeat(length / chunk_size);

                            if str == expected {
                                count += num;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
