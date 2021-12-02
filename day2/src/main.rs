use std::str::FromStr;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Forward,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

#[derive(Clone, Copy)]
struct Movement {
    magnitude: usize,
    direction: Direction,
}

impl FromStr for Movement {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        let direction = split
            .next()
            .ok_or(String::from("No direction supplied"))?
            .parse()?;
        let magnitude = split
            .next()
            .ok_or(String::from("No magnitude supplied"))?
            .trim()
            .parse()
            .map_err(|_| String::from("Unable to parse magnitude"))?;

        Ok(Movement {
            magnitude,
            direction,
        })
    }
}

struct Position {
    vertical: isize,
    horizontal: isize,
    aim: isize,
}

impl Position {
    fn r#move(&mut self, movement: Movement) {
        let mag = movement.magnitude as isize;
        match movement.direction {
            Direction::Up => self.vertical -= mag,
            Direction::Down => self.vertical += mag,
            Direction::Forward => self.horizontal += mag,
        }
    }

    fn move2(&mut self, movement: Movement) {
        let mag = movement.magnitude as isize;
        match movement.direction {
            Direction::Up => self.aim -= mag,
            Direction::Down => self.aim += mag,
            Direction::Forward => {
                self.horizontal += mag;
                self.vertical += mag * self.aim;
            }
        }
    }
}

fn main() -> Result<(), String> {
    let input = include_str!("input.txt");
    let movements: Result<Vec<Movement>, String> = input.lines().map(|l| l.parse()).collect();
    let movements = movements?;
    let mut pos = Position {
        vertical: 0,
        horizontal: 0,
        aim: 0,
    };
    for movement in movements.clone() {
        pos.r#move(movement);
    }

    println!(
        "[1] Vertical Depth {} x Horizontal Position {} = {}",
        pos.vertical,
        pos.horizontal,
        pos.vertical * pos.horizontal
    );

    let mut pos = Position {
        vertical: 0,
        horizontal: 0,
        aim: 0,
    };

    for movement in movements {
        pos.move2(movement);
    }

    println!(
        "[2] Vertical Depth {} x Horizontal Position {} = {}",
        pos.vertical,
        pos.horizontal,
        pos.vertical * pos.horizontal
    );

    Ok(())
}
