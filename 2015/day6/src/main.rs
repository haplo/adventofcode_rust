use bitvec::prelude::*;

const MAX_X: usize = 1000;
const MAX_Y: usize = 1000;

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    op: Operation,
    from: Coordinate,
    to: Coordinate,
}

impl Instruction {
    // Part 1: lights are on/off binary
    fn execute_part1(&self, grid: &mut BitBox) {
        let from_x = self.from.x;
        let to_x = self.to.x;
        for y in self.from.y..self.to.y + 1 {
            let i: usize = y * MAX_X + from_x;
            let j: usize = y * MAX_X + to_x + 1;
            match self.op {
                Operation::TurnOn => grid.as_mut_bitslice()[i..j].set_all(true),
                Operation::TurnOff => grid.as_mut_bitslice()[i..j].set_all(false),
                Operation::Toggle => grid.as_mut_bitslice()[i..j].for_each(|_, b| !b),
            }
        }
    }

    // Part 2: lights have integer brightness
    fn execute_part2(&self, grid: &mut Vec<u8>) {
        let from_x = self.from.x;
        let to_x = self.to.x;
        for y in self.from.y..self.to.y + 1 {
            let i: usize = y * MAX_X + from_x;
            let j: usize = y * MAX_X + to_x + 1;
            let slice = grid[i..j].iter_mut();
            match self.op {
                Operation::TurnOn => slice.for_each(|v| *v += 1),
                Operation::TurnOff => slice.for_each(|v| {
                    if *v > 0 {
                        *v -= 1;
                    }
                }),
                Operation::Toggle => slice.for_each(|v| *v += 2),
            }
        }
    }
}

fn parse_into_instruction(s: &str) -> Result<Instruction, &str> {
    let mut iter = s.split(" ").peekable();
    let op = match (iter.next(), iter.peek()) {
        (Some("turn"), Some(&"on")) => {
            iter.next();
            Operation::TurnOn
        }
        (Some("turn"), Some(&"off")) => {
            iter.next();
            Operation::TurnOff
        }
        (Some("toggle"), _) => Operation::Toggle,
        _ => return Err("Invalid operation"),
    };
    let from = parse_coordinates(iter.next().ok_or("Missing first coordinate")?)?;
    let inter = iter.next().ok_or("Missing through")?;
    if inter != "through" {
        return Err("Expected \"through\" between coordinates, got \"{}\"");
    }
    let to = parse_coordinates(iter.next().ok_or("Missing second coordinate")?)?;
    Ok(Instruction {
        op: op,
        from: from,
        to: to,
    })
}

fn parse_coordinate(s: &str) -> Result<usize, &str> {
    match s.parse::<usize>() {
        Ok(n) => Ok(n),
        Err(_) => Err("Bad coordinate, not an integer"),
    }
}

// parse a string slice (e.g. "12,34") into a Coordinate struct
fn parse_coordinates(s: &str) -> Result<Coordinate, &str> {
    let mut split = s.splitn(2, ",");
    let x = match split.next() {
        Some(coord) => parse_coordinate(coord)?,
        None => return Err("Bad coordinate, missing first"),
    };
    let y = match split.next() {
        Some(coord) => parse_coordinate(coord)?,
        None => return Err("Bad coordinate, missing second"),
    };
    if x > MAX_X || y > MAX_Y {
        return Err("Coordinate larger than maximum value");
    }
    Ok(Coordinate { x: x, y: y })
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut grid_part1 = bitbox![0; MAX_X*MAX_Y];
    let mut grid_part2: Vec<u8> = vec![0; MAX_X * MAX_Y];
    for (i, line) in input.lines().enumerate() {
        let instruction =
            parse_into_instruction(line).expect(&format!("Line {} has bad format", i));
        instruction.execute_part1(&mut grid_part1);
        instruction.execute_part2(&mut grid_part2);
    }
    let turned_on: usize = grid_part1.count_ones();
    let total_brightness: u32 = grid_part2.iter().fold(0 as u32, |acc, n| acc + (*n as u32));
    println!("{} lights are lit following Part 1 instructions", turned_on);
    println!(
        "Total brightness is {} following Part 2 instructions",
        total_brightness
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_execute_part1() {
        let mut grid = bitbox![0; MAX_X*MAX_Y];
        Instruction {
            op: Operation::TurnOff,
            from: Coordinate { x: 0, y: 0 },
            to: Coordinate { x: 10, y: 10 },
        }
        .execute_part1(&mut grid);
        assert_eq!(grid.count_ones(), 0);
        assert_eq!(grid.count_zeros(), 1_000_000);
        Instruction {
            op: Operation::TurnOn,
            from: Coordinate { x: 50, y: 100 },
            to: Coordinate { x: 550, y: 600 },
        }
        .execute_part1(&mut grid);
        assert_eq!(grid.count_ones(), 251_001);
        assert_eq!(grid.count_zeros(), 748_999);
        Instruction {
            op: Operation::Toggle,
            from: Coordinate { x: 0, y: 0 },
            to: Coordinate { x: 999, y: 999 },
        }
        .execute_part1(&mut grid);
        assert_eq!(grid.count_ones(), 748_999);
        assert_eq!(grid.count_zeros(), 251_001);
    }

    #[test]
    fn test_instruction_execute_part2() {
        let mut grid: Vec<u8> = vec![0; MAX_X * MAX_Y];
        Instruction {
            op: Operation::TurnOff,
            from: Coordinate { x: 0, y: 0 },
            to: Coordinate { x: 10, y: 10 },
        }
        .execute_part2(&mut grid);
        assert_eq!(grid.iter().fold(0 as u32, |acc, n| acc + (*n as u32)), 0);
        Instruction {
            op: Operation::TurnOn,
            from: Coordinate { x: 50, y: 100 },
            to: Coordinate { x: 550, y: 600 },
        }
        .execute_part2(&mut grid);
        assert_eq!(
            grid.iter().fold(0 as u32, |acc, n| acc + (*n as u32)),
            251001
        );
        Instruction {
            op: Operation::Toggle,
            from: Coordinate { x: 0, y: 0 },
            to: Coordinate { x: 499, y: 499 },
        }
        .execute_part2(&mut grid);
        assert_eq!(
            grid.iter().fold(0 as u32, |acc, n| acc + (*n as u32)),
            751001
        );
    }

    #[test]
    fn test_parse_coordinates() {
        assert_eq!(
            parse_coordinates(&"0,0").unwrap(),
            Coordinate { x: 0, y: 0 }
        );
        assert_eq!(
            parse_coordinates(&"123,456").unwrap(),
            Coordinate { x: 123, y: 456 }
        );
        assert!(parse_coordinates(&"123456789,987654321").is_err());
        assert!(parse_coordinates(&"12,ab").is_err());
        assert!(parse_coordinates(&"ab,12").is_err());
        assert!(parse_coordinates(&"abc,def").is_err());
        assert!(parse_coordinates(&"").is_err());
    }

    #[test]
    fn test_parse_into_instructions() {
        assert_eq!(
            parse_into_instruction(&"turn on 0,0 through 999,999").unwrap(),
            Instruction {
                op: Operation::TurnOn,
                from: Coordinate { x: 0, y: 0 },
                to: Coordinate { x: 999, y: 999 }
            }
        );
        assert_eq!(
            parse_into_instruction(&"turn off 42,123 through 27,456").unwrap(),
            Instruction {
                op: Operation::TurnOff,
                from: Coordinate { x: 42, y: 123 },
                to: Coordinate { x: 27, y: 456 }
            }
        );
        assert_eq!(
            parse_into_instruction(&"toggle 2,4 through 6,8").unwrap(),
            Instruction {
                op: Operation::Toggle,
                from: Coordinate { x: 2, y: 4 },
                to: Coordinate { x: 6, y: 8 }
            }
        );
        assert!(parse_into_instruction(&"turn 2,4 through 6,8").is_err());
        assert!(parse_into_instruction(&"random junk").is_err());
        assert!(parse_into_instruction(&"").is_err());
    }
}
