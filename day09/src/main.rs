extern crate simple_matrix;

use shared::read_file;
use simple_matrix::Matrix;

fn main() {
    let filename = "day09/src/input.txt";
    let data = read_file(filename);

    let res = part1(&data);
    println!("PART 1 : {}", res);

    let res = part2(&data);
    println!("PART 2 : {}", res);
}

fn part1(data: &Vec<String>) -> u32 {
    let nb_row = data.len();
    let nb_col = data[0].len();
    let mut matrix: Matrix<u32> = Matrix::new(nb_row, nb_col);

    // println!("{:?}", data);

    // init matrix
    let mut x = 0;
    let mut y = 0;
    data.iter()
        .map(|d| split_number(d.to_string()))
        .for_each(|line| {
            y = 0;
            line.iter().for_each(|&val| {
                matrix.set(x, y, val);
                y += 1;
            });
            x += 1;
        });

    let mut lowest_point: Vec<u32> = vec![];
    for x in 0..nb_row {
        for y in 0..nb_col {
            let cur = matrix.get(x, y).unwrap();

            let mut is_lowest = true;
            let min_x = if x > 0 { x - 1 } else { 0 };
            let min_y = if y > 0 { y - 1 } else { 0 };
            let max_x = if x < nb_row - 1 { x + 1 } else { nb_row - 1 };
            let max_y = if y < nb_col - 1 { y + 1 } else { nb_col - 1 };
            for a in min_x..=max_x {
                for b in min_y..=max_y {
                    if matrix.get(a, b).unwrap() < cur {
                        is_lowest = false;
                    }
                }
            }
            if is_lowest {
                lowest_point.push(*cur);
            }
        }
    }

    // println!("lowest_point {:?}", lowest_point);
    // println!("matrix {:?}", matrix);

    lowest_point.iter().sum::<u32>() + (lowest_point.len() as u32)
}

fn split_number(string: String) -> Vec<u32> {
    string
        .split("")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn part2(data: &Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/demo.txt";
        let res = part1(&read_file(filename));

        assert_eq!(res, 15);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/demo.txt";
        let res = part2(&read_file(filename));

        assert_eq!(res, 0);
    }
}
