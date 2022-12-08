extern crate pest;


use std::{rc::Rc, cell::RefCell};

use pest::{Parser, iterators::Pair};


#[derive(Parser)]
#[grammar = "./aoc_lib/day_07/day_07.pest"]
struct ConsoleParser;

struct FileSystem
{
    root: Rc<RefCell<FileSystemElem>>,
    current: Rc<RefCell<FileSystemElem>>
}

impl FileSystem
{
    fn add_to_current(&mut self, elem: FileSystemElem){
        self.current.borrow_mut().add_to_dir(elem);
    }

    fn new() -> Self{
        let root = FileSystemElem::Dir { children: Vec::new(), name: "/".to_string() };
        let root = Rc::new(RefCell::new(root));
        Self { root: root.clone(), current: root }
    }
}

enum FileSystemElem{
    Dir{
        children: Vec<FileSystemElem>,
        name: String,
    },
    File{
        name: String,
        size: usize
    }
}

impl FileSystemElem{
    fn name(&self)->String{
        match self {
            FileSystemElem::Dir { children, name } => name.clone(),
            FileSystemElem::File { name, size } => name.clone(),
        }
    }

    fn add_to_dir(&mut self, elem: FileSystemElem){
        match self {
            FileSystemElem::Dir { children, name } => {
                if !children.iter().any(|e| e.name() == *name) {
                    children.push(elem);
                }
            },
            FileSystemElem::File { name, size } => todo!(),
        }
    }
}