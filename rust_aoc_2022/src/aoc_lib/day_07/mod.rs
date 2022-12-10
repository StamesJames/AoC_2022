extern crate pest;

use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{self},
    path::Path,
    rc::Rc,
};

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "./aoc_lib/day_07/day_07.pest"]
pub struct ConsoleParser;

pub struct FileSystem {
    root: Rc<RefCell<FileSystemElem>>,
    current: Rc<RefCell<FileSystemElem>>,
}

impl FileSystem {
    pub fn total_size(&self) -> usize {
        (*self.root).borrow().size()
    }

    pub fn print(&self) {
        (*self.root).borrow().print(0);
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut result = FileSystem::new();
        let file_string = fs::read_to_string(path)?;
        let parse = ConsoleParser::parse(Rule::input, &file_string)?
            .next()
            .unwrap();

        for inner in parse.into_inner() {
            match inner.as_rule() {
                Rule::comand => {
                    let inner = inner.into_inner().next().unwrap();
                    match inner.as_rule() {
                        Rule::cd => {
                            let path = inner.into_inner().next().unwrap().as_str();
                            result.navigate_to(path);
                        },
                        Rule::ls => {},
                        _ => unreachable!()
                    }
                },
                Rule::ls_elem => {
                    let inner = inner.into_inner().next().unwrap();
                    match inner.as_rule() {
                        Rule::dir_elem => {
                            let dir_name = inner.into_inner().next().unwrap().as_str();
                            result.add_to_current(FileSystemElem::Dir {
                                name: dir_name.to_string(),
                                children: HashMap::new(),
                                parent: Some(result.current.clone()),
                            })
                        }
                        Rule::file_elem => {
                            let mut inner = inner.into_inner();
                            let file_size = inner.next().unwrap().as_str().parse::<usize>()?;
                            let file_name = inner.next().unwrap().as_str();
                            result.add_to_current(FileSystemElem::File {
                                name: file_name.to_string(),
                                size: file_size,
                                parent: result.current.clone(),
                            })
                        }
                        _ => unreachable!(),
                    }
                },
                Rule::EOI => {},
                _ => unreachable!(),
            }
        }

        Ok(result)
    }

    pub fn get_dir_sizes_above(&self, n: usize) -> Vec<usize> {
        (*self.root).borrow().get_dir_sizes_above(n)
    }
    pub fn get_dir_sizes_below(&self, n: usize) -> Vec<usize> {
        (*self.root).borrow().get_dir_sizes_below(n)
    }

    pub fn add_to_current(&mut self, elem: FileSystemElem) {
        self.current.borrow_mut().add_to_dir(elem);
    }

    pub fn navigate_to(&mut self, elem: &str) {
        match elem {
            ".." => {
                let next = (*self.current).borrow().parent();
                self.current = next;
            }
            "/" => {
                self.current = self.root.clone();
            }
            x => {
                let next = (*self.current).borrow().get_next(x);
                self.current = next;
            }
        }
    }

    pub fn new() -> Self {
        let root = FileSystemElem::Dir {
            children: HashMap::new(),
            name: "/".to_string(),
            parent: None,
        };
        let root = Rc::new(RefCell::new(root));
        Self {
            root: root.clone(),
            current: root,
        }
    }
}

pub enum FileSystemElem {
    Dir {
        name: String,
        children: HashMap<String, Rc<RefCell<FileSystemElem>>>,
        parent: Option<Rc<RefCell<FileSystemElem>>>,
    },
    File {
        name: String,
        size: usize,
        parent: Rc<RefCell<FileSystemElem>>,
    },
}

impl FileSystemElem {
    pub fn print(&self, depth:usize){
        match self {
            FileSystemElem::Dir { name, children, parent } => {
                println!("{} dir {} (size: {})", "\t".repeat(depth), name, self.size());
                for child in children.values() {
                    (**child).borrow().print(depth +1);
                }
            },
            FileSystemElem::File { name, size, parent } => {
                println!("{} file {} (size: {})", "\t".repeat(depth), name, size);
            },
        }

    }

    pub fn get_dir_sizes_above(&self, n: usize) -> Vec<usize> {
        match self {
            FileSystemElem::Dir { children, .. } => {
                let self_size = self.size();
                let mut result = children
                    .values()
                    .flat_map(|c| (**c).borrow().get_dir_sizes_above(n))
                    .collect::<Vec<usize>>();
                if self_size > n {
                    result.push(self_size);
                }
                result
            }
            FileSystemElem::File { .. } => vec![],
        }
    }

    pub fn get_dir_sizes_below(&self, n: usize) -> Vec<usize> {
        match self {
            FileSystemElem::Dir { children, .. } => {
                let self_size = self.size();
                let mut result = children
                    .values()
                    .flat_map(|c| (**c).borrow().get_dir_sizes_below(n))
                    .collect::<Vec<usize>>();
                if self_size < n {
                    result.push(self_size);
                }
                result
            }
            FileSystemElem::File { .. } => vec![],
        }
    }

    pub fn get_next(&self, next_name: &str) -> Rc<RefCell<FileSystemElem>> {
        match self {
            FileSystemElem::Dir { name, children, .. } => {
                if let Some(next) = children.get(next_name) {
                    next.clone()
                } else {
                    panic!("Dir has no child {name}")
                }
            }
            FileSystemElem::File { .. } => panic!("file can not have a next"),
        }
    }

    pub fn parent(&self) -> Rc<RefCell<FileSystemElem>> {
        match self {
            FileSystemElem::Dir { parent, .. } => {
                if let Some(parent) = parent {
                    parent.clone()
                } else {
                    panic!("Dir has no Parent")
                }
            }
            FileSystemElem::File { parent, .. } => parent.clone(),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            FileSystemElem::Dir { children, .. } => children
                .values()
                .map(|v| (**v).borrow().size())
                .sum::<usize>(),
            FileSystemElem::File { size, .. } => *size,
        }
    }

    pub fn name(&self) -> String {
        match self {
            FileSystemElem::Dir { name, .. } => name.clone(),
            FileSystemElem::File { name, .. } => name.clone(),
        }
    }

    pub fn add_to_dir(&mut self, elem: FileSystemElem) {
        match self {
            FileSystemElem::Dir { children, name, .. } => {
                if !children.values().any(|e| (**e).borrow().name() == *name) {
                    children.insert(elem.name(), Rc::new(RefCell::new(elem)));
                }
            }
            FileSystemElem::File { .. } => panic!("Can't add to File"),
        }
    }
}
