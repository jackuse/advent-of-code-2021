use std::io::BufRead;

use shared::read_file;

fn main() {
    let filename = "day08/src/input.txt";
    let data = read_file(filename);

    let res = part1(&data);
    println!("PART 1 : {}", res);

    let res = part2(&data);
    println!("PART 2 : {}", res);
}

fn part1(data: &Vec<String>) -> i32 {
    let only_digits_string: Vec<String> = data
        .iter()
        .map(|l| l.split("|").map(|s| s.to_string()).collect::<Vec<String>>()[1].to_string())
        .collect();

    // println!("only_digits_string \n{:?}", only_digits_string);

    let digit_list: Vec<Vec<String>> = only_digits_string
        .iter()
        .map(|s| {
            s.split(" ")
                .map(|s| s.to_string())
                .filter(|s| (*s).len() > 0)
                .collect()
        })
        .collect();

    // println!("digit_list \n{:?}", digit_list);

    // 0 => 5
    // 1 => 2 *
    // 2 => 5
    // 3 => 5
    // 4 => 4 *
    // 5 => 5
    // 6 => 6
    // 7 => 3 *
    // 8 => 7 *
    // 9 => 6

    let easy_by_list: Vec<i32> = digit_list
        .iter()
        .map(|digits| {
            let easy_list: Vec<bool> = digits
                .iter()
                .map(|d| {
                    if d.len() == 2 || d.len() == 4 || d.len() == 3 || d.len() == 7 {
                        true
                    } else {
                        false
                    }
                })
                .collect();

            let total_easy = easy_list
                .iter()
                .fold(0, |acc, &x| if x { acc + 1 } else { acc });
            total_easy
        })
        .collect();

    easy_by_list.iter().sum::<i32>()
}
fn part2(data: &Vec<String>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/demo.txt";
        let res = part1(&read_file(filename));

        assert_eq!(res, 26);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/demo.txt";
        let res = part2(&read_file(filename));

        assert_eq!(res, 0);
    }
}
