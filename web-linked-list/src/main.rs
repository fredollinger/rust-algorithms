use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: List,
}

impl List {
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            //next: List(self.head),
            // help: try using a variant of the expected type: `List(Link::Empty)`
            next: mem::replace(&mut self.head, Link::Empty),
        });
    
        // self.head = Link::More(new_node);
    }
}

fn main() {
    println!("Hello, world!");
}
