#[derive(Debug, Clone)]
struct Board {
    numbers: [[(usize, bool); 5]; 5],
}

impl Board {
    fn from_lines(lines: &[String]) -> Self {
        let mut numbers = [[(0, false); 5]; 5];
        for (row, line) in lines.iter().enumerate() {
            for (column, nr) in line
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .enumerate()
            {
                numbers[row][column] = (nr, false);
            }
        }

        Board { numbers }
    }

    fn mark(&mut self, nr: usize) {
        for row in self.numbers.iter_mut() {
            for mut column in row.iter_mut() {
                if column.0 == nr {
                    column.1 = true;
                }
            }
        }
    }

    fn is_winning(&self) -> bool {
        for idx in 0..=4 {
            if self.numbers[idx].iter().all(|(_nr, marked)| *marked) {
                return true;
            }

            if self.numbers.iter().all(|row| row[idx].1) {
                return true;
            }
        }

        return false;
    }

    fn print_winning_board(&self, last_number: usize) {
        let sum: usize = self
            .numbers
            .iter()
            .flat_map(|row| row)
            .filter(|(_nr, marked)| !marked)
            .map(|(nr, _marked)| nr)
            .sum();
        println!(
            "The product of the sum of non-marked numbers with the last drawn number is: {}",
            sum * last_number
        );
    }
}

fn main() {
    let input = include_str!("input.txt");

    let drawn_numbers = input.lines().next().unwrap();
    let drawn_numbers: Vec<usize> = drawn_numbers
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<String> = input.lines().skip(1).map(|l| l.to_owned()).collect();
    let boards: Vec<Board> = boards
        .chunks(6)
        .map(|b| Board::from_lines(&b[1..]))
        .collect();

    println!("--- part 1 ---");
    part1(boards.clone(), drawn_numbers.clone());
    println!("--- part 2 ---");
    part2(boards, drawn_numbers);
}

fn part1(mut boards: Vec<Board>, mut drawn_numbers: Vec<usize>) {
    let (last_draw, winning_board) = loop {
        let nr = drawn_numbers.remove(0);

        for board in boards.iter_mut() {
            board.mark(nr);
        }
        if let Some(board) = boards.iter().find(|b| b.is_winning()) {
            break (nr, board);
        }
    };

    winning_board.print_winning_board(last_draw);
}

fn part2(mut boards: Vec<Board>, mut drawn_numbers: Vec<usize>) {
    let (last_draw, winning_board) = loop {
        let nr = drawn_numbers.remove(0);

        for board in boards.iter_mut() {
            board.mark(nr);
        }
        if boards.len() == 1 && boards[0].is_winning() {
            break (nr, &boards[0]);
        }
        boards.retain(|b| !b.is_winning());
    };
    winning_board.print_winning_board(last_draw);
}
