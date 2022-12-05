use std::{collections::VecDeque};

#[derive(Debug)]
struct Instruction {
    quantity: i32,
    from: usize,
    to: usize,
}

fn move_crates(
    arrangement: &mut Vec<VecDeque<char>>,
    instruction: Instruction,
) -> Result<(), &'static str> {
    let from = instruction.from;
    let to = instruction.to;
    let quantity = instruction.quantity;

    // println!("About to move {} crates from {} to {}", quantity, from, to);

    let mut temp: VecDeque<char> = VecDeque::new();

    {
        let from_stack = arrangement.get_mut(from - 1).unwrap();

        // println!("From Stack: {:?}", from_stack);

        if from_stack.len() < quantity as usize {
            return Err("Quantity is greater than size of source stack!");
        }

        for _i in 0..quantity {
            temp.push_front(from_stack.pop_front().unwrap());
        }
    }

    // println!("Temp: {:?}", temp);

    let to_stack = arrangement.get_mut(to - 1).unwrap();

    // println!("To Stack: {:?}", to_stack);

    for _i in 0..quantity {
        //Part 1 Solution
        // to_stack.push_front(temp.pop_back().unwrap());

        //Part 2 Solution
        to_stack.push_front(temp.pop_front().unwrap());
    }

    Ok(())
}

fn main() {
    let f = std::fs::read_to_string("src/input/input.txt").expect("Invalid File");

    let mut arrangement: Vec<VecDeque<char>> = Vec::new();

    for _x in 0..f.lines().nth(0).unwrap().len() {
        arrangement.push(VecDeque::new());
    }

    for line in f.lines().take_while(|l| !l.is_empty()) {
        for c in line.chars().enumerate() {
            arrangement.get_mut(c.0).unwrap().push_back(c.1.clone());
        }
    }

    let mut indices = Vec::new();

    for line in arrangement.iter().enumerate() {
        if line.1.iter().all(|&c| !c.is_alphabetic()) {
            indices.push(line.0);
        }
    }

    for index in indices.iter().enumerate() {
        arrangement.remove(index.1 - index.0);
    }

    for stack in &mut arrangement {
        stack.retain(|c| !c.is_whitespace());
    }

    let str_instructions: Vec<&str> = f
        .lines()
        .skip(f.lines().take_while(|l| !l.is_empty()).count() + 1)
        .collect();

    let instructions = str_instructions
        .iter()
        .map(|&x| {
            x.split_whitespace()
                .filter(|&s| s.chars().all(|c| c.is_digit(10)))
                .collect::<Vec<&str>>()
        })
        .map(|x| Instruction {
            quantity: x[0].parse().unwrap(),
            from: x[1].parse().unwrap(),
            to: x[2].parse().unwrap(),
        })
        .collect::<Vec<Instruction>>();

    for instruction in instructions {
        move_crates(&mut arrangement, instruction).unwrap();
    }

    for stack in &arrangement {
        print!("{}", stack.front().unwrap()); 
    }

    
}
