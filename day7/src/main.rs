use std::{cell::RefCell, rc::Rc};

pub trait FileSystemEntity {
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;

    fn get_name(&self) -> &String;
    fn get_size(&self) -> usize;
    fn get_contents(&mut self) -> Option<&RefCell<Vec<Rc<dyn FileSystemEntity>>>>;
    fn get_parent(&self) -> Option<&Rc<dyn FileSystemEntity>>;
}

struct File {
    name: String,
    parent: Option<Rc<dyn FileSystemEntity>>,
    size: usize,
}

impl FileSystemEntity for File {
    fn is_dir(&self) -> bool {
        false
    }

    fn is_file(&self) -> bool {
        true
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_contents(&mut self) -> Option<&RefCell<Vec<Rc<dyn FileSystemEntity>>>> {
        None
    }

    fn get_parent(&self) -> Option<&Rc<dyn FileSystemEntity>> {
        if let Some(p) = &self.parent {
            Some(p)
        } else {
            None
        }
    }
}

struct Directory {
    name: String,
    parent: Option<Rc<dyn FileSystemEntity>>,
    contents: RefCell<Vec<Rc<dyn FileSystemEntity>>>,
}

impl Directory {
    fn new(name: String, parent: Option<Rc<dyn FileSystemEntity>>, contents: RefCell<Vec<Rc<dyn FileSystemEntity>>>) -> Directory {
        Directory {
            name: name,
            parent: parent,
            contents: contents
        }
    }

    // fn add_directory(&mut self, dir: Directory) {
    //     self.contents
    // }
}

impl FileSystemEntity for Directory {
    fn is_dir(&self) -> bool {
        true
    }

    fn is_file(&self) -> bool {
        false
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_size(&self) -> usize {
        let mut sum: usize = 0;

        for entity in self.contents.borrow().iter() {
            sum += entity.get_size();
        }

        return sum;
    }

    fn get_contents(&mut self) -> Option<&RefCell<Vec<Rc<dyn FileSystemEntity>>>> {
        Some(&self.contents)
    }

    fn get_parent(&self) -> Option<&Rc<dyn FileSystemEntity>> {
        if let Some(p) = &self.parent {
            Some(p)
        } else {
            None
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input/example_input.txt").expect("Invalid File");

    let root = Rc::new(Directory {
        name: "/".to_string(),
        parent: None,
        contents: RefCell::new(Vec::new()),
    });

    let mut current_directory: Rc<dyn FileSystemEntity> = root.clone();

    for raw_line in input.lines().skip(1) {
        // let mut dir_contents = current_directory.get_contents().unwrap().borrow_mut();
        let mut line = raw_line.split_whitespace();

        match line.nth(0).unwrap() {
            "$" => match line.nth(1).unwrap() {
                "cd" => match line.nth(2).unwrap() {
                    ".." => current_directory = current_directory.get_parent().unwrap().clone(),
                    s => {
                        current_directory = current_directory.get_contents().unwrap().borrow_mut()
                        // dir_contents
                            .iter()
                            .find(|&d| d.is_dir() && d.get_name() == s)
                            .unwrap()
                            .clone()
                    }
                },
                _ => (),
            },
            s => match s {
                "dir" => current_directory.get_contents().unwrap().borrow_mut().push(Rc::new(Directory {
                    name: line.nth(1).unwrap().to_string(),
                    parent: Some(current_directory.clone()),
                    contents: RefCell::new(Vec::new()),
                })),
                f => current_directory.get_contents().unwrap().borrow_mut().push(Rc::new(File {
                    name: line.nth(1).unwrap().to_string(),
                    parent: Some(current_directory.clone()),
                    size: f.parse().unwrap(),
                })),
            },
        }
    }
}
