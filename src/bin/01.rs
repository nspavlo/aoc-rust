advent_of_code::solution!(1);

const DIAL_SIZE: usize = 100;
const STARTING_POSITION: usize = 50;

fn parse_instruction(line: &str) -> Option<(char, usize)> {
    let direction = line.chars().next()?;
    let steps = line.get(1..)?.parse().ok()?;
    Some((direction, steps))
}

fn rotate_dial(position: usize, steps: usize, clockwise: bool) -> usize {
    if clockwise {
        (position + steps) % DIAL_SIZE
    } else {
        (position + DIAL_SIZE - (steps % DIAL_SIZE)) % DIAL_SIZE
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = STARTING_POSITION;
    let mut count = 0u64;

    for line in input.lines() {
        let (direction, steps) = parse_instruction(line)?;
        position = rotate_dial(position, steps, direction == 'R');

        if position == 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut position = STARTING_POSITION;
    let mut count = 0u64;

    for line in input.lines() {
        let (direction, steps) = parse_instruction(line)?;

        if direction == 'L' {
            if steps > position {
                count += ((steps - position - 1) / DIAL_SIZE + 1) as u64;
            }
        } else {
            count += ((position + steps) / DIAL_SIZE) as u64;
        }

        position = rotate_dial(position, steps, direction == 'R');
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
