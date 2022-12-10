use std::fs::read_to_string;

struct Instruction {
    duration: usize,
    value: i32
}

impl Instruction {
    fn new(duration: usize, value: i32) {
        Instruction{duration: duration, value: value};
    }
}
fn main() {
    let input = std::fs::read_to_string("./input/example_input.txt").expect("File Error!");

    let cycle: usize = 0;

    let interesting_numbers: Vec<i32> = Vec::new();

    let commands = input.lines().map(|s| match s {
      "noop" => Instruction::new(1, 0),
      i => //Split the string, confirm it's an "addx" and store the value together with the duration of 2  
    })
}
