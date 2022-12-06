use std::{path::{Path, self}, fs, collections::{VecDeque, HashMap}, hash::Hash};

pub fn get_first_message(path:&Path) -> Option<usize>{
    let string = fs::read_to_string(path).expect("couldn't find the file");
    return find_first_x_distinct(14, string);
}

pub fn get_first_packet_index(path: &Path) -> Option<usize> {
    let string = fs::read_to_string(path).expect("couldn't find the file");
    return find_first_x_distinct(4, string);
}

pub fn find_first_x_distinct(x:usize, string: String) -> Option<usize>{
    let mut counter = Counter::new();
    let mut queue = VecDeque::new();

    for (i, c) in string.chars().enumerate() {
        if queue.len() > x-1 {
            let old = queue.pop_front().unwrap();
            counter.sub(old);
        }
        counter.add(c);
        queue.push_back(c);
        if counter.entry_count() == x {
            return Some(i+1);
        }
    }

    None
}

struct Counter<T>
where
    T: Hash + Eq
{
    content: HashMap<T, usize>
}

impl<T> Counter<T>
where
    T: Hash + Eq
{
    fn new() -> Self { Self { content: HashMap::new() } }

    fn add(&mut self, c:T){
        if let Some(val) = self.content.get_mut(&c) {
            *val = *val + 1;
        } else {
            self.content.insert(c, 1);
        }
    }

    fn sub(&mut self, c:T){
        if let Some(val) = self.content.get_mut(&c){
            *val = *val - 1;
            if *val == 0{
                self.content.remove(&c);
            }
        }
    }

    fn entry_count(&self) -> usize{
        self.content.len()
    }
}