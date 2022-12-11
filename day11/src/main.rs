use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Item {
    worry: i32,
}

impl Item {
    fn new(worry: i32) -> Item {
        Item { worry }
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Divide,
    Unknown
}
#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    // operation: fn(&mut Item),
    operation: (Operation, Option<i32>),
    // check: fn(&Item) -> bool,
    check: i32,
    targets: Option<(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>)>,
    target_indices: (i32, i32)
}

impl Monkey {
    fn new(
        items: Vec<Item>,
        // operation: fn(&mut Item),
        operation: (Operation, Option<i32>),
        // check: fn(&Item) -> bool,
        check: i32,
        targets: Option<(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>)>,
        target_indices: (i32, i32),
    ) -> Monkey {
        Monkey {
            items,
            operation,
            check,
            targets,
            target_indices
        }
    }

    fn get_items(&self) -> &Vec<Item> {
        return &self.items;
    }

    fn get_items_mut(&mut self) -> &mut Vec<Item> {
        return &mut self.items;
    }

    fn throw_nth_item(&mut self, index: usize, target: Rc<RefCell<Monkey>>) {
        let item = self.items.remove(index);

        target.borrow_mut().get_items_mut().push(item);
    }

    fn inspect_items(&mut self) {
        for item in &mut self.items {
            let value = match self.operation.1 {
                Some(n) => n,
                None => item.worry
            };

            match self.operation.0 {
                Operation::Add => item.worry += value,
                Operation::Multiply => item.worry *= value,
                Operation::Divide => item.worry /= value,
                Operation::Unknown => panic!("Operation was not set")
            }
        }
    }

    fn check_items(&mut self, relief: fn(&mut Item)) {
        for mut item in self.items.iter_mut() {
            relief(item);
        }

        let true_indices: Vec<usize> = self
            .items
            .iter_mut()
            .filter(|i| i.worry % self.check == 0)
            .enumerate()
            .map(|x| x.0)
            .collect();

        for index in true_indices.iter().enumerate() {
            self.throw_nth_item(index.1 - index.0, self.targets.as_ref().unwrap().0.clone());
        }

        let false_indices: Vec<usize> = self
            .items
            .iter_mut()
            .filter(|i| i.worry % self.check != 0)
            .enumerate()
            .map(|x| x.0)
            .collect();

        for index in false_indices.iter().enumerate() {
            self.throw_nth_item(index.1 - index.0, self.targets.as_ref().unwrap().1.clone());
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input/example_input.txt").unwrap();

    let monkey_strings: Vec<Vec<&str>> = input
        .split("\r\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .chunks(6)
        .map(|x| Vec::from(x))
        .collect();

    for string in &monkey_strings {
        println!("{:?}", string);
    }

    let mut monkeys: Vec<Monkey> = Vec::new();

    for raw_monkey in &monkey_strings {
        let mut items: Vec<Item> = Vec::new();
        let mut operation: (Operation, Option<i32>) = (Operation::Unknown, None);
        let mut check: i32 = 0;
        let mut targets: Option<(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>)> = None;
        let mut target_indices: (i32, i32) = (0,0);

        for string in raw_monkey {
            if string.starts_with("Monkey") {
                continue;
            }

            if string.starts_with("Starting") {
                let mut numbers: Vec<String> = string.split_whitespace().map(|s| s.replace(",", "")).filter(|s| s.len() <= 2 && s.len() >= 1).collect();
                for number in numbers {
                    items.push(Item::new(number.parse::<i32>().unwrap()));
                }
            }

            if string.starts_with("Operation") {
                let mut words = string.split_whitespace().rev();
                operation.1 = match words.next().unwrap() {
                    "old" => None,
                    i => Some(i.parse::<i32>().unwrap())
                };

                operation.0 = match words.next().unwrap() {
                    "+" => Operation::Add,
                    "*" => Operation::Multiply,
                    "/" => Operation::Divide,
                    _ => panic!("Unknown operation supplied")
                }
            }

            if string.starts_with("Test") {
                check = string.split_whitespace().last().unwrap().parse::<i32>().unwrap();
            }

            if string.starts_with("If true") {
                target_indices.0 = string.split_whitespace().last().unwrap().parse().unwrap();
            }

            if string.starts_with("If false") {
                target_indices.1 = string.split_whitespace().last().unwrap().parse().unwrap(); 
            }
        }

        monkeys.push(Monkey::new(items, operation, check, targets, target_indices));
    }

    for monkey in monkeys {
        println!("{:?}", monkey);
    }

    let relief: fn(&mut Item) = |a| a.worry /= 3;
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{Item, Monkey};

    #[test]
    fn monkey_inspecting_items_changes_worry_level() {
        let mut foo = Monkey::new(
            vec![Item::new(0), Item::new(10)],
            |i| i.worry += 1,
            |x| x.worry > 0,
            None,
        );

        foo.inspect_items();

        assert_eq!(foo.get_items()[0].worry, 1);
        assert_eq!(foo.get_items()[1].worry, 11);
    }

    #[test]
    fn monkey_checking_items_throws_correctly() {
        let mut foo = Rc::new(RefCell::new(Monkey::new(
            vec![Item::new(1), Item::new(-1)],
            |i| i.worry += 1,
            |x| x.worry > 0,
            None,
        )));
        let bar = Rc::new(RefCell::new(Monkey::new(
            Vec::new(),
            |i| i.worry *= 2,
            |x| x.worry % 2 == 0,
            None,
        )));
        let oof = Rc::new(RefCell::new(Monkey::new(
            Vec::new(),
            |i| i.worry /= 2,
            |x| x.worry.is_negative(),
            None,
        )));

        foo.borrow_mut().targets = Some((bar.clone(), oof.clone()));

        foo.borrow_mut().check_items(|_i| ());

        assert_eq!(foo.borrow().get_items().len(), 0);
        assert_eq!(bar.borrow().get_items().len(), 1);
        assert_eq!(oof.borrow().get_items().len(), 1);
    }
}
