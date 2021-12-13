use shared::read_file;

fn main() {
    let filename = "day07/src/input.txt";
    let data = read_file(filename);

    let res = part1(&data);
    println!("PART 1 : {}", res);

    let res = part2(&data);
    println!("PART 2 : {}", res);
}

fn part1(data: &Vec<String>) -> u32 {
    let position = extract_one_line_list(&data);
    // todo make a ford-flukerson

    // brut force
    let max_pos = *position.iter().max().unwrap() + 1;
    let mut total_fuel_by_position = vec![0; max_pos as usize];
    // println!("{:?}", total_fuel_by_position);

    for pos in 0..max_pos {
        let mut total_fuel = 0;
        position
            .iter()
            .for_each(|&p| total_fuel += (p as i32 - pos as i32).abs() as u32);
        total_fuel_by_position[pos as usize] = total_fuel;
    }

    // println!("{:?}", total_fuel_by_position);
    let min = *total_fuel_by_position.iter().min().unwrap();
    // let position = total_fuel_by_position.iter().position(|&f| f == min).unwrap();

    min
}
fn part2(data: &Vec<String>) -> usize {
    0
}

fn extract_one_line_list(one_line_list: &Vec<String>) -> Vec<u32> {
    one_line_list[0]
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/demo.txt";
        let res = part1(&read_file(filename));

        assert_eq!(res, 0);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/demo.txt";
        let res = part2(&read_file(filename));

        assert_eq!(res, 0);
    }
}
