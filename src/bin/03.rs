advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut count: u64 = 0u64;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut max = 0u64;

        for i in 0..chars.len() - 1 {
            for j in i + 1..chars.len() {
                let tmp = format!("{}{}", chars[i], chars[j])
                    .parse::<u64>()
                    .unwrap_or(0);
                max = max.max(tmp);
            }
        }
        count += max;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count: u64 = 0u64;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let keep = 12;

        if chars.len() <= keep {
            let res_u64 = line.parse::<u64>().unwrap_or(0);
            count += res_u64;
            continue;
        }

        let mut stack: Vec<char> = Vec::new();
        let mut to_remove = chars.len() - keep;

        for &ch in &chars {
            while !stack.is_empty() && to_remove > 0 && stack[stack.len() - 1] < ch {
                stack.pop();
                to_remove -= 1;
            }
            stack.push(ch);
        }

        // In case all the removals are at the end.
        stack.truncate(keep);

        let res_str: String = stack.iter().collect();
        let res_u64 = res_str.parse::<u64>().unwrap_or(0);
        count += res_u64;
    }

    return Some(count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
