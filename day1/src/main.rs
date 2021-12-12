use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let res = part_1_count_increase(read_file(filename));

    println!("Number of increase : {}", res);
}

fn part_1_count_increase(readings: Vec<String>) -> i32 {
    let number_list = readings
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut acc = 0;
    let mut last_val = number_list[0];
    for (pos, val) in number_list.into_iter().enumerate() {
        if pos == 0 {
            continue;
        }
        if val > last_val {
            acc += 1;
        }
        last_val = val
    }

    return acc;
}

fn read_file(filename: &str) -> Vec<String> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut res = vec![];

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  // println!("{}. {}", index + 1, line);
        res.push(line)
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn should_read_file() {
        let filename = "src/report.txt";
        let res_str = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        let res = res_str
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(read_file(filename), res);
    }

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/report.txt";
        let res = part_1_count_increase(read_file(filename));

        assert_eq!(res, 7);
    }
}
