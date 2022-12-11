#[derive(Debug)]
struct Instruction {
    duration: usize,
    value: i32,
}

impl Instruction {
    fn new(duration: usize, value: i32) -> Instruction {
        Instruction {
            duration: duration,
            value: value,
        }
    }
}
fn main() {
    let input = std::fs::read_to_string("./input/input.txt").expect("File Error!");

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|s| match s {
            "noop" => return Instruction::new(1, 0),
            i => {
                let mut inst = i.split_whitespace();
                match inst.next().unwrap() {
                    "addx" => {
                        return Instruction::new(2, inst.next().unwrap().parse::<i32>().unwrap())
                    }
                    _ => return Instruction::new(0, 0),
                }
            }
        })
        .collect();

    let mut register: i32 = 1;

    let mut signals: Vec<i32> = Vec::new();

    for inst in instructions {
        for _step in 0..inst.duration {
            signals.push(register.clone());
        }

        register += inst.value;
    }

    signals.push(register.clone());

    // Part 1 Solution
    // let signal_strengths: Vec<i32> = signals.iter().enumerate().map(|x| (x.0 as i32 + 1) * x.1).collect();

    // let mut interesting_numbers: Vec<&i32> = Vec::new();

    // for s in signal_strengths.iter().skip(19).step_by(40) {
    //     interesting_numbers.push(s);
    // }

    // println!("\nSUM: {}", interesting_numbers.iter().fold(0, |acc, s| acc + *s));

    // Part 2 Solution
    for chunk in signals.chunks(40) {
        for s in chunk.iter().enumerate() {
            if (s.0 as i32 - 1..=s.0 as i32 + 1).contains(&s.1) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}
