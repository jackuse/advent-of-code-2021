use shared::read_file;

fn main() {
    let filename = "day10/src/input.txt";
    let data = read_file(filename);

    let res = part1(&data);
    println!("PART 1 : {}", res);

    let res = part2(&data);
    println!("PART 2 : {}", res);
}

#[derive(Debug)]
struct ParseError {
    char: String,
    message: String,
}

fn complementary_symbol(sym: &str) -> &str {
    match sym {
        "(" => ")",
        "[" => "]",
        "{" => "}",
        "<" => ">",
        ")" => "(",
        "]" => "[",
        "}" => "{",
        ">" => "<",
        _ => "",
    }
}

fn parse_char<'a, 'b>(char: &'a str, stack: &'b mut Stack<&'a str>) -> Result<bool, ParseError> {
    return match char {
        "(" => {
            stack.push(char);
            Ok(true)
        }
        "[" => {
            stack.push(char);
            Ok(true)
        }
        "{" => {
            stack.push(char);
            Ok(true)
        }
        "<" => {
            stack.push(char);
            Ok(true)
        }
        ")" => {
            let tmp = stack.pop().unwrap();
            if tmp != "(" {
                Err(ParseError {
                    message: format!(
                        "Expected {}, but found ) instead",
                        complementary_symbol(tmp)
                    ),
                    char: ")".to_string(),
                })
            } else {
                Ok(true)
            }
        }
        "]" => {
            let tmp = stack.pop().unwrap();
            if tmp != "[" {
                Err(ParseError {
                    message: format!(
                        "Expected {}, but found ] instead",
                        complementary_symbol(tmp)
                    ),
                    char: "]".to_string(),
                })
            } else {
                Ok(true)
            }
        }
        "}" => {
            let tmp = stack.pop().unwrap();
            if tmp != "{" {
                Err(ParseError {
                    message: format!(
                        "Expected {}, but found }} instead",
                        complementary_symbol(tmp)
                    ),
                    char: "}".to_string(),
                })
            } else {
                Ok(true)
            }
        }
        ">" => {
            let tmp = stack.pop().unwrap();
            if tmp != "<" {
                Err(ParseError {
                    message: format!(
                        "Expected {}, but found > instead",
                        complementary_symbol(tmp)
                    ),
                    char: ">".to_string(),
                })
            } else {
                Ok(true)
            }
        }
        a => Err(ParseError {
            message: format!("Unexpected symbol {}", a),
            char: a.to_string(),
        }),
    };
}

fn part1(data: &Vec<String>) -> u32 {
    let string = &data[0];

    let mut stack: Stack<&str> = Stack::new();

    let mut errors: Vec<ParseError> = vec![];

    data.iter().for_each(|line| {
        line.split("").filter(|s| s.len() > 0).for_each(|char| {
            let res = parse_char(char, &mut stack);
            match res {
                Ok(_) => (),
                Err(err) => {
                    println!("On line {} \n--{}", line, err.message);
                    errors.push(err);
                }
            };
        });
    });

    let res: u32 = errors.iter().map(|err| match err.char.as_str() {
        ")" => 3,
        "]" => 57,
        "}" => 1197,
        ">" => 25137,
        _ => 0,
    }).sum();

    // println!("errors {:?}", errors);
    // println!("res {}", res);
    // println!("{}", data[0]);
    // println!("{} / {} / {} / {}", par, cro, aco, cev);

    res
}
fn part2(data: &Vec<String>) -> u32 {
    0
}

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/demo.txt";
        let res = part1(&read_file(filename));

        assert_eq!(res, 26397);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/demo.txt";
        let res = part2(&read_file(filename));

        assert_eq!(res, 0);
    }
}
