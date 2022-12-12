use core::fmt;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Item {
    worry: u64,
}

impl Item {
    fn new(worry: u64) -> Item {
        Item { worry }
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Divide,
    Unknown,
}
#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: (Operation, Option<u64>),
    check: u64,
    target_indices: (usize, usize),
    targets: Option<(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>)>,
    inspections: usize,
}

impl Monkey {
    fn new(
        items: Vec<Item>,
        operation: (Operation, Option<u64>),
        check: u64,
        target_indices: (usize, usize),
        targets: Option<(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>)>,
    ) -> Monkey {
        Monkey {
            items,
            operation,
            check,
            target_indices,
            targets,
            inspections: 0,
        }
    }

    fn get_items(&self) -> &Vec<Item> {
        return &self.items;
    }

    fn get_items_mut(&mut self) -> &mut Vec<Item> {
        return &mut self.items;
    }

    fn set_targets(&mut self, monkeys: &Vec<Rc<RefCell<Monkey>>>) {
        let if_true = monkeys.get(self.target_indices.0).unwrap();
        let if_false = monkeys.get(self.target_indices.1).unwrap();

        self.targets = Some((if_true.clone(), if_false.clone()));
    }

    fn throw_nth_item(&mut self, index: usize, target: Rc<RefCell<Monkey>>) {
        let item = self.items.remove(index);

        target.borrow_mut().get_items_mut().push(item);
    }

    fn inspect_items(&mut self, modulo: Option<u64>) {
        for item in &mut self.items {
            let value = match self.operation.1 {
                Some(n) => n,
                None => item.worry,
            };

            match self.operation.0 {
                Operation::Add => item.worry += value,
                Operation::Multiply => item.worry *= value,
                Operation::Divide => item.worry /= value,
                Operation::Unknown => panic!("Operation was not set"),
            }

            if let Some(m) = modulo {
                item.worry %= m;
            }

            self.inspections += 1;
        }
    }

    fn check_items(&mut self, relief: fn(&mut Item)) {
        for item in self.items.iter_mut() {
            relief(item);
        }

        let true_indices: Vec<usize> = self
            .items
            .iter_mut()
            .enumerate()
            .filter(|i| i.1.worry % self.check == 0)
            .map(|x| x.0)
            .collect();

        for index in true_indices.iter().enumerate() {
            self.throw_nth_item(index.1 - index.0, self.targets.as_ref().unwrap().0.clone());
        }

        let false_indices: Vec<usize> = self
            .items
            .iter_mut()
            .enumerate()
            .filter(|i| i.1.worry % self.check != 0)
            .map(|x| x.0)
            .collect();

        for index in false_indices.iter().enumerate() {
            self.throw_nth_item(index.1 - index.0, self.targets.as_ref().unwrap().1.clone());
        }
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Monkey: {:?}, {:?}, {}, {:?}",
            self.items, self.operation, self.check, self.target_indices
        )
    }
}

fn main() {
    let input = std::fs::read_to_string("./input/input.txt").unwrap();

    let monkey_strings: Vec<Vec<&str>> = input
        .split("\r\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .chunks(6)
        .map(|x| Vec::from(x))
        .collect();

    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = Vec::new();

    for raw_monkey in &monkey_strings {
        let mut items: Vec<Item> = Vec::new();
        let mut operation: (Operation, Option<u64>) = (Operation::Unknown, None);
        let mut check: u64 = 0;
        let mut target_indices: (usize, usize) = (0, 0);
        let targets: Option<(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>)> = None;

        for string in raw_monkey {
            if string.starts_with("Monkey") {
                continue;
            }

            if string.starts_with("Starting") {
                let numbers: Vec<String> = string
                    .split_whitespace()
                    .map(|s| s.replace(",", ""))
                    .filter(|s| s.len() <= 2 && s.len() >= 1)
                    .collect();
                for number in numbers {
                    items.push(Item::new(number.parse::<u64>().unwrap()));
                }
            }

            if string.starts_with("Operation") {
                let mut words = string.split_whitespace().rev();
                operation.1 = match words.next().unwrap() {
                    "old" => None,
                    i => Some(i.parse::<u64>().unwrap()),
                };

                operation.0 = match words.next().unwrap() {
                    "+" => Operation::Add,
                    "*" => Operation::Multiply,
                    "/" => Operation::Divide,
                    _ => panic!("Unknown operation supplied"),
                }
            }

            if string.starts_with("Test") {
                check = string
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
            }

            if string.starts_with("If true") {
                target_indices.0 = string.split_whitespace().last().unwrap().parse().unwrap();
            }

            if string.starts_with("If false") {
                target_indices.1 = string.split_whitespace().last().unwrap().parse().unwrap();
            }
        }

        monkeys.push(Rc::new(RefCell::new(Monkey::new(
            items,
            operation,
            check,
            target_indices,
            targets,
        ))));
    }

    for monkey in &monkeys {
        monkey.borrow_mut().set_targets(&monkeys);
    }

    // Part 1 Relief
    // let relief: fn(&mut Item) = |a| {
    //     let mut x = a.worry as f32;
    //     x /= 3.0;

    //     a.worry = x.floor() as i32;
    // };

    // Part 2 Relief
    let relief: fn(&mut Item) = |a| ();

    // Part 1 Solution
    // for i in 0..20 {

    // Part 2 Solution
    let modulo: u64 = monkeys.iter().map(|m| m.borrow().check).product();
    for _i in 0..10000 {
        for monkey in monkeys.iter_mut().enumerate() {
            if monkey.1.borrow().get_items().len() == 0 {
                continue;
            }

            monkey.1.borrow_mut().inspect_items(Some(modulo));
            monkey.1.borrow_mut().check_items(relief);
        }
    }

    monkeys.sort_by(|a, b| a.borrow().inspections.cmp(&b.borrow().inspections));

    let monkey_business: usize = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.borrow_mut().inspections)
        .product();

    println!("{}", monkey_business);
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{Item, Monkey, Operation};

    #[test]
    fn monkey_inspecting_items_changes_worry_level() {
        let mut foo = Monkey::new(
            vec![Item::new(0), Item::new(10)],
            (Operation::Add, Some(1)),
            2,
            (1, 2),
            None,
        );

        foo.inspect_items();

        assert_eq!(foo.get_items()[0].worry, 1);
        assert_eq!(foo.get_items()[1].worry, 11);
    }

    #[test]
    fn set_target_sets_correct_targets() {
        let mut foo = Rc::new(RefCell::new(Monkey::new(
            vec![Item::new(0), Item::new(9)],
            (Operation::Add, Some(1)),
            2,
            (1, 2),
            None,
        )));

        let bar = Rc::new(RefCell::new(Monkey::new(
            Vec::new(),
            (Operation::Multiply, Some(2)),
            2,
            (0, 2),
            None,
        )));
        let oof = Rc::new(RefCell::new(Monkey::new(
            Vec::new(),
            (Operation::Multiply, None),
            5,
            (0, 1),
            None,
        )));

        let monkeys: Vec<Rc<RefCell<Monkey>>> = vec![foo.clone(), bar.clone(), oof.clone()];

        for monkey in &monkeys {
            monkey.borrow_mut().set_targets(&monkeys);
        }

        foo.borrow_mut().check_items(|_i| ());

        assert_eq!(foo.borrow_mut().get_items().len(), 0);
        assert_eq!(bar.borrow().get_items().len(), 1);
        assert_eq!(oof.borrow().get_items().len(), 1);
    }

    #[test]
    fn monkey_checking_items_throws_correctly() {
        let mut monkeys: Vec<Monkey> = Vec::new();

        let mut foo = Monkey::new(
            vec![Item::new(0), Item::new(9)],
            (Operation::Add, Some(1)),
            2,
            (1, 2),
            None,
        );
        let bar = Rc::new(RefCell::new(Monkey::new(
            Vec::new(),
            (Operation::Multiply, Some(2)),
            2,
            (0, 2),
            None,
        )));
        let oof = Rc::new(RefCell::new(Monkey::new(
            Vec::new(),
            (Operation::Multiply, None),
            5,
            (0, 1),
            None,
        )));

        foo.targets = Some((bar.clone(), oof.clone()));

        foo.check_items(|_i| ());

        assert_eq!(foo.get_items().len(), 0);
        assert_eq!(bar.borrow().get_items().len(), 1);
        assert_eq!(oof.borrow().get_items().len(), 1);
    }

    #[test]
    fn checking_items_throws_correctly_for_two_rounds() {
        let one = Rc::new(RefCell::new(Monkey::new(
            vec![Item::new(79), Item::new(98)],
            (Operation::Multiply, Some(19)),
            23,
            (2, 3),
            None,
        )));

        let two = Rc::new(RefCell::new(Monkey::new(
            vec![Item::new(54), Item::new(65), Item::new(74)],
            (Operation::Add, Some(6)),
            19,
            (2, 0),
            None,
        )));

        let three = Rc::new(RefCell::new(Monkey::new(
            vec![Item::new(79), Item::new(60), Item::new(97)],
            (Operation::Multiply, None),
            13,
            (1, 3),
            None,
        )));

        let four = Rc::new(RefCell::new(Monkey::new(
            vec![Item::new(74)],
            (Operation::Add, Some(3)),
            17,
            (0, 1),
            None,
        )));

        let monkeys: Vec<Rc<RefCell<Monkey>>> =
            vec![one.clone(), two.clone(), three.clone(), four.clone()];

        for monkey in &monkeys {
            monkey.borrow_mut().set_targets(&monkeys);
        }

        for monkey in &monkeys {
            println!("{}", monkey.borrow());
        }

        println!();

        let relief: fn(&mut Item) = |a| {
            let mut x = a.worry as f32;
            x /= 3.0;

            a.worry = x.floor() as u64;
        };

        for round in 0..2 {
            println!("Round {}", round + 1);
            for monkey in monkeys.iter().enumerate() {
                println!("Monkey {}", monkey.0);
                if monkey.1.borrow().get_items().is_empty() {
                    continue;
                } else {
                    monkey.1.borrow_mut().inspect_items();
                    monkey.1.borrow_mut().check_items(relief);
                }
                println!();
            }

            println!();
            for monkey in &monkeys {
                println!("{}", monkey.borrow());
            }
            println!();
        }

        assert_eq!(one.borrow().get_items().first().unwrap().worry, 695);
    }
}
