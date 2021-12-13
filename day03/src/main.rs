use shared::read_file;

fn main() {
    let filename = "src/input.txt";
    let diag_report = read_file(filename);

    // commands.iter().for_each(|c| println!("{}", *c));

    let res = part1(&diag_report);
    println!("PART 1 : {}", res);

    let res = part2(&diag_report);
    println!("PART 2 : {}", res);
}


fn part1(diag_report: &Vec<String>) -> i32 {
    let list_most_one = list_most_one(&diag_report);

    let majority = diag_report.len() as i32 / 2;
    let gamma_rate_string = list_most_one
        .iter()
        .map(|count| if *count > majority { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");
    let epsilon_rate_string = list_most_one
        .iter()
        .map(|count| if *count > majority { "0" } else { "1" })
        .collect::<Vec<&str>>()
        .join("");

    let gamma_rate = i32::from_str_radix(gamma_rate_string.as_str(), 2).unwrap();
    let epsilon_rate = i32::from_str_radix(epsilon_rate_string.as_str(), 2).unwrap();

    gamma_rate * epsilon_rate
}


fn part2(diag_report: &Vec<String>) -> i32 {
    let oxygen_rate = extract_rate(diag_report, true, None);
    let co2_rate = extract_rate(diag_report, false, None);

    oxygen_rate * co2_rate
}

fn list_most_one(binary_number_list: &Vec<String>) -> Vec<i32> {
    let number_size = split_bit(&binary_number_list[0]).len();
    let mut acc_one: Vec<i32> = Vec::with_capacity(number_size);
    for _ in 0..number_size {
        acc_one.push(0);
    }

    binary_number_list.iter().for_each(|l| {
        let bit_list = split_bit(l);

        for (pos, val) in bit_list.into_iter().enumerate() {
            if val == "1" {
                acc_one[pos] += 1;
            }
        }
    });

    acc_one
}

fn split_bit<'a>(bit_number_string: &'a String) -> Vec<&'a str> {
    bit_number_string
        .split("")
        .filter(|a| *a == "0" || *a == "1")
        .collect::<Vec<&'a str>>()
}

fn filter_report(diag_report: &Vec<String>, pos: usize, char: &str) -> Vec<String> {
    diag_report
        .to_vec()
        .into_iter()
        .filter(|line| line.chars().nth(pos).unwrap().to_string().as_str() == char)
        .collect::<Vec<String>>()
}

fn extract_rate(diag_report: &Vec<String>, isMost: bool, position: Option<usize>) -> i32 {
    if diag_report.len() == 1 {
        // println!("{}", diag_report[0]);
        return i32::from_str_radix(diag_report[0].as_str(), 2).unwrap();
    }

    let mut pos = position.unwrap_or(0);
    let majority = diag_report.len() as f32 / 2f32;
    let list_most_one = list_most_one(&diag_report);
    let mut new_diag_report = vec![];

    if isMost {
        if list_most_one[pos] as f32 >= majority {
            new_diag_report = filter_report(diag_report, pos, "1");
        } else {
            new_diag_report = filter_report(diag_report, pos, "0");
        }
    } else {
        if (list_most_one[pos] as f32) < majority {
            new_diag_report = filter_report(diag_report, pos, "1");
        } else {
            new_diag_report = filter_report(diag_report, pos, "0");
        }
    }
    pos += 1;
    extract_rate(&new_diag_report, isMost, Some(pos))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/demo.txt";
        let res = part1(&read_file(filename));

        assert_eq!(res, 198);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/demo.txt";
        let res = part2(&read_file(filename));

        assert_eq!(res, 230);
    }
}
