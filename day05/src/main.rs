use shared::read_file;
use std::fmt;

fn main() {
    let filename = "day05/src/input.txt";
    let vents_lines = read_file(filename);

    let res = part1(&vents_lines);
    println!("PART 1 : {}", res);

    let res = part2(&vents_lines);
    println!("PART 2 : {}", res);
}

fn part1(vents_lines: &Vec<String>) -> u32 {
    let vector = extract_vector(vents_lines);
    // println!("vector {:?}", vector);
    let hv_vector = vector
        .into_iter()
        .filter(|v| v.0[0] == v.1[0] || v.0[1] == v.1[1])
        .collect::<Vec<(Vec<u32>, Vec<u32>)>>();
    // println!("hv_vector {:?}", hv_vector);

    let size = size_grid(&hv_vector);
    // println!("size {}", size);

    let mut matrice = Matrice::new(size);

    for vector in hv_vector {
        matrice.add_vector(&vector);
    }
    // println!("matrice {}", matrice);

    matrice.count_more_than_two()
}

fn part2(vents_lines: &Vec<String>) -> u32 {
    let vectors = extract_vector(vents_lines);
    // println!("vector {:?}", vectors);

    let size = size_grid(&vectors);
    // println!("size {}", size);

    let mut matrice = Matrice::new(size);

    for vector in vectors {
        matrice.add_vector(&vector);
    }
    // println!("matrice {}", matrice);

    matrice.count_more_than_two()
}

#[derive(Debug)]
struct Matrice {
    lines: Vec<Vec<u32>>,
}

impl Matrice {
    pub fn new(size: u32) -> Self {
        let mut lines = vec![];

        for _ in 0..size {
            let mut line = vec![];
            for _ in 0..size {
                line.push(0);
            }
            lines.push(line);
        }

        Self { lines: lines }
    }

    pub fn add_vector(&mut self, vector: &(Vec<u32>, Vec<u32>)) {
        let mut line: Vec<(u32, u32)> = vec![];

        let up_x = vector.0[0] as i64 - vector.1[0] as i64;
        let left_y = vector.0[1] as i64 - vector.1[1] as i64;
        let mut x = vector.0[0];
        let mut y = vector.0[1];
        let max_len = if up_x.abs() > left_y.abs() {
            up_x.abs()
        } else {
            left_y.abs()
        };
        for _ in 0..=max_len {
            line.push((x, y));
            if x != vector.1[0] || y != vector.1[1] {
                if up_x < 0 {
                    x += 1;
                } else if up_x > 0 {
                    x -= 1;
                }
                if left_y < 0 {
                    y += 1;
                } else if left_y > 0 {
                    y -= 1;
                }
            }
        }

        // println!(" line {:?}", line);

        line.into_iter().for_each(|point| {
            self.lines[point.1 as usize][point.0 as usize] += 1;
        })
    }

    pub fn count_more_than_two(&self) -> u32 {
        let mut res = 0;
        self.lines.iter().for_each(|line| {
            line.iter().for_each(|case| {
                if *case > 1 {
                    res += 1;
                }
            });
        });

        res
    }
}

impl fmt::Display for Matrice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")?;
        for line in self.lines.iter() {
            let line_string = line
                .iter()
                .map(|c| {
                    if *c == 0 {
                        ".".to_string()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");
            println!("");
            write!(f, "{}", line_string)?;
        }
        write!(f, "")
    }
}

fn size_grid(vectors: &Vec<(Vec<u32>, Vec<u32>)>) -> u32 {
    let mut max = 0;
    vectors.iter().for_each(|vector| {
        if max < vector.0[0] {
            max = vector.0[0];
        }
        if max < vector.0[1] {
            max = vector.0[1];
        }
        if max < vector.1[0] {
            max = vector.1[0];
        }
        if max < vector.1[1] {
            max = vector.1[1];
        }
    });

    max + 1
}

fn extract_vector(vents_lines: &Vec<String>) -> Vec<(Vec<u32>, Vec<u32>)> {
    vents_lines
        .iter()
        .map(|line| {
            let tmp = line.split(" ").collect::<Vec<&str>>();
            let point_a = tmp[0]
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let point_b = tmp[2]
                .split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (point_a, point_b)
        })
        .collect::<Vec<(Vec<u32>, Vec<u32>)>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/demo.txt";
        let res = part1(&read_file(filename));

        assert_eq!(res, 5);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/demo.txt";
        let res = part2(&read_file(filename));

        assert_eq!(res, 12);
    }
}
