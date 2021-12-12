use shared::read_file;

fn main() {
    let filename = "src/input.txt";
    let bingo_board = read_file(filename);
    // commands.iter().for_each(|c| println!("{}", *c));

    let res = part1(&bingo_board);
    println!("PART 1 : {}", res);

    let res = part2(&bingo_board);
    println!("PART 2 : {}", res);
}

fn part1(bingo_board: &Vec<String>) -> u32 {
    let numbers = bingo_board[0]
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut bingo_board_clean = bingo_board.to_vec();
    bingo_board_clean.remove(0);
    bingo_board_clean.remove(0);

    let mut boards = create_boards(&bingo_board_clean);

    let mut res = 0;
    // for number in &numbers.as_slice()[..12] {
    for number in &numbers {
        // println!("number  {}", number);
        for board in &mut boards {
            res = board.mark_value(*number);
            if res > 0 {
                break;
            }
        }
        if res > 0 {
            break;
        }
    }

    // println!("board 1 {:?}", boards[0]);

    res
}

fn create_boards(bingo_board_clean: &Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    let mut tmp_board: Vec<String> = vec![];

    bingo_board_clean.iter().for_each(|line| {
        if line.len() > 0 {
            tmp_board.push(line.as_str().to_string());
        } else {
            let new_board = tmp_board.to_vec();
            boards.push(Board::new(new_board));
            tmp_board = vec![];
        }
    });
    let new_board = tmp_board.to_vec();
    boards.push(Board::new(new_board));

    boards
}

fn part2(bingo_board: &Vec<String>) -> u32 {
    0
}

#[derive(Debug)]
struct Case {
    value: u32,
    is_marked: bool,
}

impl Case {
    pub fn new(case_string: &str) -> Self {
        Self {
            value: case_string.parse::<u32>().unwrap(),
            is_marked: false,
        }
    }

    pub fn mark_value(self: &mut Self, number: u32) {
        if self.value == number {
            self.is_marked = true;
        }
    }
}

#[derive(Debug)]
struct Line {
    line: Vec<Case>,
}

impl Line {
    pub fn new(line_string: String) -> Self {
        Self {
            line: line_string
                .split(" ")
                .filter(|c| c.len() > 0)
                .map(|c| Case::new(c))
                .collect::<Vec<Case>>(),
        }
    }

    pub fn mark_value(self: &mut Self, number: u32) {
        self.line
            .iter_mut()
            .for_each(|case| case.mark_value(number));
    }

    pub fn is_complete(self: &Self) -> bool {
        let mut is_complete = true;
        self.line.iter().for_each(|case| {
            is_complete = is_complete && case.is_marked;
        });

        is_complete
    }

    pub fn is_marked(self: &Self, pos: usize) -> bool {
        self.line[pos].is_marked
    }

    pub fn sum_unmarked(self: &Self) -> u32 {
        let mut sum = 0;
        self.line.iter().for_each(|case| {
            if !case.is_marked {
                sum += case.value
            }
        });

        sum
    }
}

#[derive(Debug)]
struct Board {
    matrice: Vec<Line>,
}

impl Board {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            matrice: lines.iter().map(|l| Line::new((&l).to_string())).collect(),
        }
    }
    pub fn mark_value(self: &mut Self, number: u32) -> u32 {
        self.matrice
            .iter_mut()
            .for_each(|line| line.mark_value(number));

        let mut sum: u32 = 0;
        let res_line = self
            .matrice
            .iter()
            .map(|line| line.is_complete())
            .collect::<Vec<bool>>();

        let mut res_col: Vec<bool> = vec![];
        for col_pos in 0..self.matrice[0].line.len() {
            let mut is_complete = true;
            self.matrice.iter().for_each(|line| {
                is_complete = is_complete && line.is_marked(col_pos);
            });
            res_col.push(is_complete);
        }

        if res_line.contains(&true) || res_col.contains(&true) {
            sum = self.matrice.iter().map(|line| line.sum_unmarked()).sum();
        }

        sum * number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/demo.txt";
        let res = part1(&read_file(filename));

        assert_eq!(res, 4512);
    }

    #[test]
    fn should_pass_part1_demo_col() {
        let filename = "src/demo-col.txt";
        let res = part1(&read_file(filename));

        assert_eq!(res, 2590);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/demo.txt";
        let res = part2(&read_file(filename));

        assert_eq!(res, 0);
    }
}
