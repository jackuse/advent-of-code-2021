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

fn split_bit<'a>(bit_number_string: &'a String) -> Vec<&'a str> {
    bit_number_string
        .split("")
        .filter(|a| *a == "0" || *a == "1")
        .collect::<Vec<&'a str>>()
}

fn part1(diag_report: &Vec<String>) -> i32 {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    let number_size = split_bit(&diag_report[0]).len();
    let mut acc_one: Vec<i32> = Vec::with_capacity(number_size);
    for _ in 0..number_size {
        acc_one.push(0);
    }

    diag_report.iter().for_each(|l| {
        let bit_list = split_bit(l);

        for (pos, val) in bit_list.into_iter().enumerate() {
            if val == "1" {
                acc_one[pos] += 1;
            }
        }
    });

    let majority = diag_report.len() as i32 / 2;
    let gamma_rate_string = acc_one
        .iter()
        .map(|count| if *count > majority { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");
    let epsilon_rate_string = acc_one
        .iter()
        .map(|count| if *count > majority { "0" } else { "1" })
        .collect::<Vec<&str>>()
        .join("");

    gamma_rate = i32::from_str_radix(gamma_rate_string.as_str(), 2).unwrap();
    epsilon_rate = i32::from_str_radix(epsilon_rate_string.as_str(), 2).unwrap();

    // acc_one.iter().for_each(|c| println!("{}", *c));

    gamma_rate * epsilon_rate
}

fn part2(diag_report: &Vec<String>) -> i32 {
    0
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

        assert_eq!(res, 0);
    }
}
