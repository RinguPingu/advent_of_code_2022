use std::{cell::RefCell, rc::Rc};

pub trait FileSystemEntity {
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;

    fn add_directory(&mut self, dir: Directory);
    fn add_file(&mut self, file: File);

    fn get_name(&self) -> &String;
    fn get_size(&self) -> usize;
    fn get_contents(&self) -> Option<&Vec<Rc<RefCell<dyn FileSystemEntity>>>>;
    fn get_parent(&self) -> Option<&Rc<RefCell<dyn FileSystemEntity>>>;
    fn get_child(&self, name: &str) -> Option<&Rc<RefCell<dyn FileSystemEntity>>>;

    fn get_dir_sizes(&self, threshold: usize) -> Option<Vec<(String, usize)>>;
}

pub struct File {
    name: String,
    parent: Option<Rc<RefCell<dyn FileSystemEntity>>>,
    size: usize,
}

impl File {
    fn new(name: String, parent: Option<Rc<RefCell<dyn FileSystemEntity>>>, size: usize) -> File {
        File {
            name: name,
            parent: parent,
            size: size,
        }
    }
}

impl FileSystemEntity for File {
    fn is_dir(&self) -> bool {
        false
    }

    fn is_file(&self) -> bool {
        true
    }

    fn add_directory(&mut self, _dir: Directory) {}
    fn add_file(&mut self, _file: File) {}

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_contents(&self) -> Option<&Vec<Rc<RefCell<dyn FileSystemEntity>>>> {
        None
    }

    fn get_parent(&self) -> Option<&Rc<RefCell<dyn FileSystemEntity>>> {
        if let Some(p) = &self.parent {
            Some(p)
        } else {
            None
        }
    }

    fn get_child(&self, _name: &str) -> Option<&Rc<RefCell<dyn FileSystemEntity>>> {
        None
    }

    fn get_dir_sizes(&self, _threshold: usize) -> Option<Vec<(String, usize)>> {
        None
    }
}

pub struct Directory {
    name: String,
    parent: Option<Rc<RefCell<dyn FileSystemEntity>>>,
    contents: Vec<Rc<RefCell<dyn FileSystemEntity>>>,
}

impl Directory {
    fn new(
        name: String,
        parent: Option<Rc<RefCell<dyn FileSystemEntity>>>,
        contents: Vec<Rc<RefCell<dyn FileSystemEntity>>>,
    ) -> Directory {
        Directory {
            name: name,
            parent: parent,
            contents: contents,
        }
    }
}

impl FileSystemEntity for Directory {
    fn is_dir(&self) -> bool {
        true
    }

    fn is_file(&self) -> bool {
        false
    }

    fn add_directory(&mut self, dir: Directory) {
        self.contents.push(Rc::new(RefCell::new(dir)));
    }

    fn add_file(&mut self, file: File) {
        self.contents.push(Rc::new(RefCell::new(file)));
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_size(&self) -> usize {
        let mut sum: usize = 0;

        for entity in &self.contents {
            sum += entity.borrow().get_size();
        }

        return sum;
    }

    fn get_contents(&self) -> Option<&Vec<Rc<RefCell<dyn FileSystemEntity>>>> {
        Some(&self.contents)
    }

    fn get_parent(&self) -> Option<&Rc<RefCell<dyn FileSystemEntity>>> {
        if let Some(p) = &self.parent {
            Some(p)
        } else {
            None
        }
    }

    fn get_child(&self, name: &str) -> Option<&Rc<RefCell<dyn FileSystemEntity>>> {
        self.contents
            .iter()
            .find(|&d| d.borrow().get_name() == name)
    }

    fn get_dir_sizes(&self, threshold: usize) -> Option<Vec<(String, usize)>> {
        let mut sizes: Vec<(String, usize)> = Vec::new();

        for entity in &self.contents {
            if entity.borrow().is_dir() {
                let name = entity.borrow().get_name().to_string();
                let size = entity.borrow().get_size();
                
                // Part 1 Solution
                // if size <= threshold {

                // Part 2 Solution
                if size >= threshold {
                    sizes.push((name, size));
                }

                if !entity.borrow().get_contents().unwrap().is_empty() {
                    let mut more_sizes = entity.borrow().get_dir_sizes(threshold).unwrap();
                    sizes.append(&mut more_sizes);
                }
            }
        }

        return Some(sizes);
    }
}

fn main() {
    let input = std::fs::read_to_string("./input/input.txt").expect("Invalid File");

    let root = Rc::new(RefCell::new(Directory::new(
        "/".to_string(),
        None,
        Vec::new(),
    )));

    let mut current_directory: Rc<RefCell<dyn FileSystemEntity>> = root.clone();

    for mut line in input.lines().skip(1).map(|x| x.split_whitespace()) {
        match line.next().unwrap() {
            "$" => match line.next().unwrap() {
                "cd" => match line.next().unwrap() {
                    ".." => {
                        let parent = current_directory.borrow().get_parent().unwrap().clone();
                        current_directory = parent;
                    }
                    s => {
                        let child = current_directory.borrow().get_child(s).unwrap().clone();
                        current_directory = child;
                    }
                },
                _ => (),
            },
            s => match s {
                "dir" => current_directory.borrow_mut().add_directory(Directory::new(
                    line.next().unwrap().to_string(),
                    Some(current_directory.clone()),
                    Vec::new(),
                )),
                f => current_directory.borrow_mut().add_file(File::new(
                    line.next().unwrap().to_string(),
                    Some(current_directory.clone()),
                    f.parse().unwrap(),
                )),
            },
        };
    }

    let disk_space: usize = 70000000;

    let used_space: usize = root.borrow().get_size();

    let free_space: usize = disk_space - used_space;

    let required_space: usize = 30000000;

    let dirs = root.borrow().get_dir_sizes(required_space - free_space).unwrap().iter().map(|d| d.1).min().unwrap();

    println!("{:?}", dirs);


}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{Directory, FileSystemEntity};

    #[test]
    fn directory_if_parent_get_parent_returns_parent() {
        let foo = Rc::new(RefCell::new(Directory::new(
            "foo".to_string(),
            None,
            Vec::new(),
        )));
        let bar = Rc::new(RefCell::new(Directory::new(
            "bar".to_string(),
            Some(foo.clone()),
            Vec::new(),
        )));

        bar.borrow().get_parent().unwrap();
        assert_eq!(
            bar.borrow().get_parent().unwrap().borrow().get_name(),
            "foo"
        );
    }

    #[test]
    fn directory_add_directory() {
        let foo = Rc::new(RefCell::new(Directory::new(
            "foo".to_string(),
            None,
            Vec::new(),
        )));
        foo.borrow_mut().add_directory(Directory::new(
            "bar".to_string(),
            Some(foo.clone()),
            Vec::new(),
        ));
    }

    #[test]
    fn current_directory_changes() {
        let foo = Rc::new(RefCell::new(Directory::new(
            "foo".to_string(),
            None,
            Vec::new(),
        )));
        let bar = Rc::new(RefCell::new(Directory::new(
            "bar".to_string(),
            Some(foo.clone()),
            Vec::new(),
        )));

        let mut current_directory: Rc<RefCell<dyn FileSystemEntity>> = foo.clone();
        assert_eq!(current_directory.borrow().get_name(), "foo");

        current_directory = bar.clone();
        assert_eq!(current_directory.borrow().get_name(), "bar");

        let parent = current_directory.borrow().get_parent().unwrap().clone();
        current_directory = parent;
        assert_eq!(current_directory.borrow().get_name(), "foo");
    }
}
