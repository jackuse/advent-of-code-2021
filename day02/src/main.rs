use shared::read_file;

fn main() {
    let filename = "src/input.txt";
    let commands = read_file(filename);

    // commands.iter().for_each(|c| println!("{}", *c));

    let res = part1(&commands);
    println!("PART 1 : {}", res);

    let res = part2(&commands);
    println!("PART 2 : {}", res);
}

fn part1(commands: &Vec<String>) -> i32 {
    let mut h_pos = 0;
    let mut depth_pos = 0;

    commands.iter().for_each(|command| {
        let tmp = command.split(" ").collect::<Vec<&str>>();
        let action = tmp[0];
        let power = tmp[1].parse::<i32>().unwrap();

        if action == "forward" {
            h_pos += power;
        }
        if action == "down" {
            depth_pos += power;
        }
        if action == "up" {
            depth_pos -= power;
        }
    });

    h_pos * depth_pos
}

fn part2(commands: &Vec<String>) -> i32 {
    let mut h_pos = 0;
    let mut depth_pos = 0;
    let mut aim = 0;

    commands.iter().for_each(|command| {
        let tmp = command.split(" ").collect::<Vec<&str>>();
        let action = tmp[0];
        let power = tmp[1].parse::<i32>().unwrap();

        if action == "forward" {
            h_pos += power;
            depth_pos += aim * power;
        }
        if action == "down" {
            aim += power;
        }
        if action == "up" {
            aim -= power;
        }
    });

    h_pos * depth_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/demo.txt";
        let res = part1(&read_file(filename));

        assert_eq!(res, 150);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/demo.txt";
        let res = part2(&read_file(filename));

        assert_eq!(res, 900);
    }
}
