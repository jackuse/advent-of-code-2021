use shared::read_file;

fn main() {
    let filename = "src/input.txt";
    let number_list = convert_string_to_number(read_file(filename));

    // number_list.iter().for_each(|w| println!("{}", *w));

    let res = part_1_count_increase(&number_list);
    println!("PART 1 : Number of increase : {}", res);

    let res = part_2(&number_list);
    println!("PART 2 : Number of increase : {}", res);
}

fn part_1_count_increase(number_list: &Vec<i32>) -> i32 {
    let mut acc = 0;
    let mut last_val = number_list[0];
    for (pos, val) in number_list.into_iter().enumerate() {
        if pos == 0 {
            continue;
        }
        if *val > last_val {
            acc += 1;
        }
        last_val = *val
    }

    return acc;
}

fn part_2(number_list: &Vec<i32>) -> i32 {
    let mut window_list: Vec<i32> = vec![];
    for (pos, val) in number_list.into_iter().enumerate() {
        window_list.push(*val);
        if pos as i32 - 1 >= 0 {
            window_list[pos - 1] = val + window_list[pos - 1];
        }
        if pos as i32 - 2 >= 0 {
            window_list[pos - 2] = val + window_list[pos - 2];
        }
    }

    // remove last 2 incomplete values
    window_list.pop();
    window_list.pop();

    // println!("window");
    // window_list.iter().for_each(|w| println!("{}", *w));

    part_1_count_increase(&window_list)
}

fn convert_string_to_number(vector: Vec<String>) -> Vec<i32> {
    vector
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/report.txt";
        let number_list = convert_string_to_number(read_file(filename));
        let res = part_1_count_increase(&number_list);

        assert_eq!(res, 7);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/report.txt";
        let number_list = convert_string_to_number(read_file(filename));
        let res = part_2(&number_list);

        assert_eq!(res, 5);
    }
}
