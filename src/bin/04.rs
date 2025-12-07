advent_of_code::solution!(4);

const ROLL: char = '@';
const REMOVED_ROLL: char = 'x';
const EMPTY_SPACE: char = '.';

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0u64;

    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let current = grid[i][j];

            if current != ROLL {
                continue;
            }

            let mut neighbor_count = 0;
            let directions = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            for (di, dj) in directions {
                let x = (i as isize) + di;
                let y = (j as isize) + dj;

                let rows = grid.len() as isize;
                let cols = grid[i].len() as isize;

                if x >= 0 && x < rows && y >= 0 && y < cols {
                    let nx = x as usize;
                    let ny = y as usize;

                    let neighbor_value = grid[nx][ny];
                    if neighbor_value == ROLL {
                        neighbor_count += 1;
                    }
                }
            }

            if neighbor_count < 4 {
                count += 1;
            }
        }
    }

    return Some(count);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count = 0u64;

    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    loop {
        for row in &mut grid {
            for cell in row.iter_mut() {
                if *cell == REMOVED_ROLL {
                    *cell = EMPTY_SPACE;
                }
            }
        }

        let mut removed_this_round = 0u64;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let current = grid[i][j];

                if current != ROLL {
                    continue;
                }

                let mut neighbor_count = 0;
                let directions = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];
                for (di, dj) in directions {
                    let x = (i as isize) + di;
                    let y = (j as isize) + dj;

                    let rows = grid.len() as isize;
                    let cols = grid[i].len() as isize;

                    if x >= 0 && x < rows && y >= 0 && y < cols {
                        let nx = x as usize;
                        let ny = y as usize;

                        let neighbor_value = grid[nx][ny];
                        if neighbor_value == ROLL || neighbor_value == REMOVED_ROLL {
                            neighbor_count += 1;
                        }
                    }
                }

                if neighbor_count < 4 {
                    grid[i][j] = REMOVED_ROLL;
                    removed_this_round += 1;
                    count += 1;
                    continue;
                }
            }
        }

        if removed_this_round == 0 {
            break;
        }
    }

    return Some(count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
