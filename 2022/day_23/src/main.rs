use aoc::grid::*;
use aoc::*;

static DAY: &'static str = "23";

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let input = load_input(format!("./2022/day_{DAY}/fixtures/input.txt"));

    let start = Instant::now();
    let result1 = part_one(&input);
    let end = Instant::now();
    println!("Elapsed time: {:?}", end - start);
    println!("Answer day {DAY} part one: {result1} in {:?}", end - start);

    let start = Instant::now();
    let result2 = part_two(&input);
    let end = Instant::now();
    println!("Answer day {DAY} part two: {result2} in {:?}", end - start);
}

fn part_one(input: &String) -> isize {
    let mut grid = create_hash_set_from_input(&input, |c| c == '#');

    let nw = Coordinate { x: -1, y: -1 };
    let n = Coordinate { x: 0, y: -1 };
    let ne = Coordinate { x: 1, y: -1 };
    let e = Coordinate { x: 1, y: 0 };
    let se = Coordinate { x: 1, y: 1 };
    let s = Coordinate { x: 0, y: 1 };
    let sw = Coordinate { x: -1, y: 1 };
    let w = Coordinate { x: -1, y: 0 };

    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    print_hash_set_grid(&grid);

    for _ in 0..10 {
        let mut proposes: HashMap<Coordinate, Vec<Coordinate>> = HashMap::new();
        let mut next_grid: HashSet<Coordinate> = HashSet::new();

        // first half of a round: each Elf considers the eight positions adjacent to themself
        'outer: for elf in grid.iter() {
            if elf.extended_neighbors().iter().all(|n| !grid.contains(n)) {
                next_grid.insert(*elf);
            } else {
                for direction in directions.iter() {
                    match direction {
                        Direction::North => {
                            if !grid.contains(&(*elf + nw))
                                && !grid.contains(&(*elf + n))
                                && !grid.contains(&(*elf + ne))
                            {
                                proposes.entry(*elf + n).or_default();
                                proposes.get_mut(&(*elf + n)).unwrap().push(*elf);
                                continue 'outer;
                            }
                        }
                        Direction::South => {
                            if !grid.contains(&(*elf + sw))
                                && !grid.contains(&(*elf + s))
                                && !grid.contains(&(*elf + se))
                            {
                                proposes.entry(*elf + s).or_default();
                                proposes.get_mut(&(*elf + s)).unwrap().push(*elf);
                                continue 'outer;
                            }
                        }
                        Direction::West => {
                            if !grid.contains(&(*elf + sw))
                                && !grid.contains(&(*elf + w))
                                && !grid.contains(&(*elf + nw))
                            {
                                proposes.entry(*elf + w).or_default();
                                proposes.get_mut(&(*elf + w)).unwrap().push(*elf);
                                continue 'outer;
                            }
                        }
                        Direction::East => {
                            if !grid.contains(&(*elf + ne))
                                && !grid.contains(&(*elf + e))
                                && !grid.contains(&(*elf + se))
                            {
                                proposes.entry(*elf + e).or_default();
                                proposes.get_mut(&(*elf + e)).unwrap().push(*elf);
                                continue 'outer;
                            }
                        }
                    }
                }
            }
            next_grid.insert(*elf);
        }

        // second half of the round: simultaneously, each Elf moves to their proposed destination tile
        // if they were the only Elf to propose moving to that position.
        // If two or more Elves propose moving to the same position, none of those Elves move.
        for (new, old) in proposes.iter() {
            if old.len() == 1 {
                next_grid.insert(*new);
            } else {
                for elf in old {
                    next_grid.insert(*elf);
                }
            }
        }

        let last_direction = directions.remove(0);
        directions.push(last_direction);
        grid = next_grid;
    }
    
    //print_hash_set_grid(&grid);

    let min_x = grid.iter().map(|c| c.x).min().unwrap();
    let max_x = grid.iter().map(|c| c.x).max().unwrap();
    let min_y = grid.iter().map(|c| c.y).min().unwrap();
    let max_y = grid.iter().map(|c| c.y).max().unwrap();

    return (max_x - min_x + 1) * (max_y - min_y + 1) - grid.len() as isize;
}

fn part_two(_input: &String) -> isize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(110, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(4070, part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(20, part_two(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(0, part_two(&input));
    }
}
