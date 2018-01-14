pub struct Node {
       pub data: i32,
       pub node: Option<Box<Node>>
}

struct LinkedList {
    junk: i32, // remove me
    head: Box<Node>
}

impl LinkedList {
    pub fn new() -> LinkedList {
        let z: Box<Node> = Box::new(Node {
            data: 0,
            node: None
        });

        let ll = LinkedList {
            junk: 4,
            head: Box::new(Node {
                data: 4,
                node: Some(z)
            })
        };
        ll
    } // END LinkedList new()

    pub fn print(&mut self) {
        println!("{}", self.head.data);
        /*
        if self.head.node.is_some() {
            println!("{}", self.head.node.unwrap().data);
        }
        else {
            println!("head points to nothing");
        }
        */
        // ll.head.node = Some(ll.z);
    }

    pub fn insert_after(&self) {
    }
} // END impl LinkedList

fn main() {
    let mut _ll = LinkedList::new();
    _ll.print();
    // LinkedList::print(_ll);
}
