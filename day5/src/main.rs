use std::cmp::{max, min};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug)]
struct Floor {
    vents: HashMap<(usize, usize), usize>,
}

impl Floor {
    fn mark_line(&mut self, line: Line) {
        if line.start.0 == line.end.0 {
            let limits = (min(line.start.1, line.end.1), max(line.start.1, line.end.1));
            for y in limits.0..=limits.1 {
                *self.vents.entry((line.start.0, y)).or_insert(0) += 1;
            }
        } else if line.start.1 == line.end.1 {
            let limits = (min(line.start.0, line.end.0), max(line.start.0, line.end.0));
            for x in limits.0..=limits.1 {
                *self.vents.entry((x, line.start.1)).or_insert(0) += 1;
            }
        } else {
            let (min_x, max_x) = (min(line.start.0, line.end.0), max(line.start.0, line.end.0));
            let (min_y, _max_y) = (min(line.start.1, line.end.1), max(line.start.1, line.end.1));

            let factor_x: isize = if min_x == line.start.0 { 1 } else { -1 };
            let factor_y: isize = if min_y == line.start.1 { 1 } else { -1 };

            let diff = max_x - min_x;
            for d in 0..=diff {
                let x_coord = (line.start.0 as isize + (d as isize * factor_x)) as usize;
                let y_coord = (line.start.1 as isize + (d as isize * factor_y)) as usize;
                *self.vents.entry((x_coord, y_coord)).or_insert(0) += 1;
            }
        }
    }

    fn count_doubles(&self) -> usize {
        self.vents.values().filter(|v| **v >= 2).count()
    }
}

impl Line {
    fn from_line(line: &str) -> Self {
        let mut split = line.split(" -> ");
        let (mut start_split, mut end_split) = (
            split.next().unwrap().split(","),
            split.next().unwrap().split(","),
        );
        let start = (
            start_split.next().unwrap().parse().unwrap(),
            start_split.next().unwrap().parse().unwrap(),
        );
        let end = (
            end_split.next().unwrap().parse().unwrap(),
            end_split.next().unwrap().parse().unwrap(),
        );

        Line { start, end }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<Line> = input.lines().map(|l| Line::from_line(l)).collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[Line]) {
    let mut floor = Floor {
        vents: HashMap::new(),
    };

    let straight_lines = lines
        .iter()
        .filter(|l| l.start.0 == l.end.0 || l.start.1 == l.end.1)
        .copied();

    for line in straight_lines {
        floor.mark_line(line);
    }

    println!(
        "[1] After marking all horizontal/vertical lines, there are {} double-or-more coordinates.",
        floor.count_doubles()
    );
}

fn part2(lines: &[Line]) {
    let mut floor = Floor {
        vents: HashMap::new(),
    };

    for line in lines {
        floor.mark_line(*line);
    }

    println!(
        "[2] After marking all lines, there are {} double-or-more coordinates.",
        floor.count_doubles()
    );
}
