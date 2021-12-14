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

    // 0 => 6 +
    // 1 => 2 *
    // 2 => 5
    // 3 => 5
    // 4 => 4 *
    // 5 => 5
    // 6 => 6 +
    // 7 => 3 *
    // 8 => 7 *
    // 9 => 6 +

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

fn part2(data: &Vec<String>) -> usize {
    let data_and_needle_list: Vec<Vec<Vec<String>>> = data
        .iter()
        .map(|line| {
            line.split("|")
                .map(|part| part.to_string())
                .map(|part| {
                    part.split(" ")
                        .map(|s| s.to_string())
                        .filter(|s| (*s).len() > 0) // remove empty string
                        .map(|s| sort(&s)) // sort A-Z
                        .collect::<Vec<String>>()
                })
                .collect()
        })
        .collect();

    let number_found_list: Vec<String> = data_and_needle_list.iter().map(|data_and_needle| {
        let mut finder = Finder::new();

        finder.init(&data_and_needle[0]);
        finder.deduce_other(&data_and_needle[0]);
        finder.decode_number(&data_and_needle[1])

    }).collect();

    number_found_list.iter().map(|number| number.parse::<usize>().unwrap()).sum()
}

fn _part2_old(data: &Vec<String>) -> usize {
    let data_and_needle: Vec<Vec<Vec<String>>> = data
        .iter()
        .map(|line| {
            line.split("|")
                .map(|part| part.to_string())
                .map(|part| {
                    part.split(" ")
                        .map(|s| s.to_string())
                        .filter(|s| (*s).len() > 0) // remove empty string
                        .map(|s| sort(&s)) // sort A-Z
                        .collect::<Vec<String>>()
                })
                .collect()
        })
        .collect();

    // println!("data_and_needle \n{:?}", &data_and_needle);

    let data_clean = &data_and_needle[0][0];
    // println!("data_clean \n{:?}", data_clean);

    //  aaaa
    // b    c
    // b    c
    //  dddd
    // e    f
    // e    f
    //  gggg
    //
    // Depuis les unique 1 4 7 8
    // manque grp 6 9 0  (t6) et grp 2 3 5 (t5)
    //
    // t2 => cf
    // t6 sans c ou f => 6
    // ========= 1 4 7 8 6 RAF 9 0 2 3 5
    // t6 avec 4 => 0 => 0 et 9 trouvé
    // ========= 1 4 7 8 6 9 0 RAF 2 3 5
    // t5 avec 1 => 3
    // 9 sans 6 => C trouvé
    // t5 sans C => 5 => 2 et  trouvé

    let mut memory = Finder::new();

    // Init with know value
    data_clean.iter().for_each(|digit| {
        if digit.len() == 2 {
            memory.save(1, &digit);
        } else if digit.len() == 3 {
            memory.save(7, &digit);
        } else if digit.len() == 4 {
            memory.save(4, &digit);
        } else if digit.len() == 7 {
            memory.save(8, &digit);
        }
    });

    println!("{:?}", &memory);

    // Find 6 9 0 3
    data_clean.iter().for_each(|digit| {
        if digit.len() == 6 {
            let decomp_one = memory.get_decomposed(1);

            if !digit.contains(decomp_one[0]) || !digit.contains(decomp_one[1]) {
                memory.save(6, &digit);
            } else {
                let is_0 = contain_needles(&memory.get_decomposed(4), &digit);

                if is_0 {
                    memory.save(9, &digit);
                } else {
                    memory.save(0, &digit);
                }
            }
        } else if digit.len() == 5 {
            let is_3 = contain_needles(&memory.get_decomposed(1), &digit);

            if is_3 {
                memory.save(3, &digit);
            }
        }
    });

    // find 5 2
    data_clean.iter().for_each(|digit| {
        if digit.len() == 5 {
            let mut segment_c = "";
            memory.get_decomposed(9).iter().for_each(|&letter| {
                if !contain_needles(&vec![letter], &memory.get(6)) {
                    segment_c = letter;
                }
            });
            // println!("segment_C {:?}", &segment_c);

            if not_contain_needles(&vec![segment_c], digit) && *digit != memory.get(3) {
                memory.save(5, &digit);
            } else if *digit != memory.get(3) {
                memory.save(2, &digit);
            }
        }
    });

    println!("{:?}", &memory);

    let number_clean = &data_and_needle[0][1];

    let number_found: Vec<usize> = number_clean.iter().map(|a| memory.get_number(a)).collect();

    println!("number_clean {:?}", &number_clean);
    println!("number_found {:?}", &number_found);

    let res = number_found
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("res {:?}", &res);

    0
}

fn contain_needles(needles: &Vec<&str>, haystack: &String) -> bool {
    needles
        .iter()
        .map(|&letter| haystack.contains(letter))
        .fold(true, |acc, x| acc && x)
}

fn not_contain_needles(needles: &Vec<&str>, haystack: &String) -> bool {
    needles
        .iter()
        .map(|&letter| !haystack.contains(letter))
        .fold(true, |acc, x| acc && x)
}

fn sort(string: &String) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect::<String>()
}

#[derive(Debug)]
struct Finder {
    memory: Vec<String>,
}

impl Finder {
    pub fn new() -> Self {
        Self {
            memory: vec!["".to_string(); 10],
        }
    }

    pub fn init(&mut self, data: &Vec<String>) {
        // Init with know value
        data.iter().for_each(|digit| {
            if digit.len() == 2 {
                self.save(1, &digit);
            } else if digit.len() == 3 {
                self.save(7, &digit);
            } else if digit.len() == 4 {
                self.save(4, &digit);
            } else if digit.len() == 7 {
                self.save(8, &digit);
            }
        });
    }

    pub fn deduce_other(&mut self, data: &Vec<String>) {
        // Find 6 9 0 3
        data.iter().for_each(|digit| {
            if digit.len() == 6 {
                let decomp_one = self.get_decomposed(1);

                if !digit.contains(decomp_one[0]) || !digit.contains(decomp_one[1]) {
                    self.save(6, &digit);
                } else {
                    let is_0 = contain_needles(&self.get_decomposed(4), &digit);

                    if is_0 {
                        self.save(9, &digit);
                    } else {
                        self.save(0, &digit);
                    }
                }
            } else if digit.len() == 5 {
                let is_3 = contain_needles(&self.get_decomposed(1), &digit);

                if is_3 {
                    self.save(3, &digit);
                }
            }
        });

        // find 5 2
        data.iter().for_each(|digit| {
            if digit.len() == 5 {
                let mut segment_c = "";
                self.get_decomposed(9).iter().for_each(|&letter| {
                    if !contain_needles(&vec![letter], &self.get(6)) {
                        segment_c = letter;
                    }
                });
                // println!("segment_C {:?}", &segment_c);

                if not_contain_needles(&vec![segment_c], digit) && *digit != self.get(3) {
                    self.save(5, &digit);
                } else if *digit != self.get(3) {
                    self.save(2, &digit);
                }
            }
        });
    }

    pub fn decode_number(&self, segment_number: &Vec<String>) -> String {
        let number_found: Vec<usize> = segment_number.iter().map(|a| self.get_number(a)).collect();
        number_found
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn save(&mut self, number: usize, string: &String) {
        self.memory[number] = string.to_string();
    }

    pub fn get(&self, number: usize) -> String {
        self.memory[number].to_string()
    }

    pub fn get_decomposed(&self, number: usize) -> Vec<&str> {
        self.memory[number]
            .split("")
            .filter(|s| (*s).len() > 0)
            .collect()
    }
    pub fn get_number(&self, string: &String) -> usize {
        // println!("{:?}", string);
        self.memory
            .iter()
            .position(|a| a.as_str() == string.as_str())
            .unwrap()
    }
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
