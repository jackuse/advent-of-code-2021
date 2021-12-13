use shared::read_file;

fn main() {
    let filename = "day06/src/input.txt";
    let fish = read_file(filename);

    let res = part1(&fish);
    println!("PART 1 : {}", res);

    let res = part2(&fish);
    println!("PART 2 : {}", res);
}

fn part1(fish: &Vec<String>) -> usize {
    let mut fish_list = to_fish_list(&fish);
    let max_day: u8 = 80;

    // println!("Initial state  : {:?}", fish_list);

    for day in 1..=max_day {
        fish_list = sim_day(&fish_list);
        // println!("After  {:>2} days : {:?}", day, fish_list);
    }

    fish_list.len()
}

fn part2(fish: &Vec<String>) -> u64 {
    let fish_list = to_fish_list(&fish);
    let max_day: u16 = 256;

    let mut fish_inventory_by_astro_sign: Vec<u64> = Vec::with_capacity(9);
    for _ in 0..9 {
        fish_inventory_by_astro_sign.push(0);
    }

    fish_list.iter().for_each(|&fish| {
        fish_inventory_by_astro_sign[fish as usize] += 1;
    });

    // println!("Initial state  : {:?}", fish_inventory_by_astro_sign);

    for day in 0..max_day {
        let astro_sign_new_born: usize = ((day + 7) % 9).into();
        let parent_fish: usize = (day % 9).into();
        fish_inventory_by_astro_sign[astro_sign_new_born] +=
            fish_inventory_by_astro_sign[parent_fish];
        // println!("After  {:>3} days : {:?}", day, fish_inventory_by_astro_sign);
    }

    fish_inventory_by_astro_sign.into_iter().sum()
}

fn to_fish_list(fish: &Vec<String>) -> Vec<u8> {
    fish[0]
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}

fn sim_day(fish_list: &Vec<u8>) -> Vec<u8> {
    let mut new_fish: Vec<u8> = vec![];
    let mut fish_list_updated = fish_list
        .iter()
        .map(|&fish| {
            let updated_fish;
            if fish == 0 {
                new_fish.push(8);
                updated_fish = 6;
            } else {
                updated_fish = fish - 1;
            }
            updated_fish
        })
        .collect::<Vec<u8>>();
    fish_list_updated.append(&mut new_fish);
    fish_list_updated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/demo.txt";
        let res = part1(&read_file(filename));

        assert_eq!(res, 5934);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/demo.txt";
        let res = part2(&read_file(filename));

        assert_eq!(res, 26984457539);
    }
}
