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

    init_matrix(&mut matrix, &data);

    let lowest_point = find_lowest_point(&matrix);
    // println!("lowest_point {:?}", lowest_point);
    // println!("matrix {:?}", matrix);

    lowest_point.iter().map(|point| point.value).sum::<u32>() + (lowest_point.len() as u32)
}

fn init_matrix(matrix: &mut Matrix<u32>, data: &Vec<String>) {
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
        })
}

fn split_number(string: String) -> Vec<u32> {
    string
        .split("")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn find_lowest_point(matrix: &Matrix<u32>) -> Vec<Point> {
    let mut lowest_point: Vec<Point> = vec![];
    for x in 0..matrix.rows() {
        for y in 0..matrix.cols() {
            let cur = matrix.get(x, y).unwrap();

            let mut is_lowest = true;
            let min_x = if x > 0 { x - 1 } else { 0 };
            let min_y = if y > 0 { y - 1 } else { 0 };
            let max_x = if x < matrix.rows() - 1 {
                x + 1
            } else {
                matrix.rows() - 1
            };
            let max_y = if y < matrix.cols() - 1 {
                y + 1
            } else {
                matrix.cols() - 1
            };
            for a in min_x..=max_x {
                for b in min_y..=max_y {
                    if matrix.get(a, b).unwrap() < cur {
                        is_lowest = false;
                    }
                }
            }
            if is_lowest {
                lowest_point.push(Point {
                    value: *cur,
                    row: x,
                    col: y,
                });
            }
        }
    }

    lowest_point
}

#[derive(Debug, Default)]
struct Point {
    value: u32,
    row: usize,
    col: usize,
}

fn part2(data: &Vec<String>) -> u64 {
    let nb_row = data.len();
    let nb_col = data[0].len();
    let mut matrix: Matrix<u32> = Matrix::new(nb_row, nb_col);

    init_matrix(&mut matrix, &data);

    let lowest_point = find_lowest_point(&matrix);
    println!("lowest_point {:?}", lowest_point[3]);

    let mut bassin: Vec<u32> = vec![];

    // let mut visited: Vec<(usize, usize)> = vec![];
    // visited.push((lowest_point[3].row, lowest_point[3].col));
    // let size = bassin_size(&matrix, &lowest_point[3], &mut visited, 1);
    // println!("size {}", size);

    lowest_point.iter().for_each(|point| {
        let mut visited: Vec<(usize, usize)> = vec![];
        visited.push((point.row, point.col));
        let size = bassin_size(&matrix, &point, &mut visited, 1);

        bassin.push(size);
    });

    bassin.sort();
    bassin.reverse();

    println!("{:?}", bassin);
    bassin[0..3].iter().fold::<u64, _>(1, |acc, &x| acc * x as u64)
}

fn bassin_size(
    matrix: &Matrix<u32>,
    point: &Point,
    visited: &mut Vec<(usize, usize)>,
    size: u32,
) -> u32 {
    let mut res = 1;
    let to_visit_list = vec![
        Point {
            value: 0,
            row: if point.row != 0 { point.row - 1 } else { 0 },
            col: point.col,
        },
        Point {
            value: 0,
            row: point.row,
            col: if point.col != 0 { point.col - 1 } else { 0 },
        },
        Point {
            value: 0,
            row: point.row,
            col: if point.col + 1 >= matrix.cols() {
                point.col
            } else {
                point.col + 1
            },
        },
        Point {
            value: 0,
            row: if point.row + 1 >= matrix.rows() {
                point.row
            } else {
                point.row + 1
            },
            col: point.col,
        },
    ];

    // println!(
    //     "to_visit_list {:?}",
    //     to_visit_list
    // );

    to_visit_list.iter().for_each(|to_visit_point| {
        let pt = *matrix.get(to_visit_point.row, to_visit_point.col).unwrap();
        let is_visited = visited
            .iter()
            .any(|t| t.0 == to_visit_point.row && t.1 == to_visit_point.col);
        visited.push((to_visit_point.row, to_visit_point.col));
        if pt != 9
            && !is_visited
            && !(to_visit_point.row == point.row && to_visit_point.col == point.col)
        {
            // println!(
            //     "call ({},{})  {}",
            //     to_visit_point.row, to_visit_point.col, size
            // );
            res += bassin_size(
                matrix,
                &Point {
                    row: to_visit_point.row,
                    col: to_visit_point.col,
                    value: pt,
                },
                visited,
                size + 1,
            );
        }
    });

    // println!(
    //     "{:width$}point ({},{}) ",
    //     "",
    //     point.row,
    //     point.col,
    //     width = (size as usize)
    // );
    // println!("end res {:?}", res);

    res
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

        assert_eq!(res, 1134);
    }
}
