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

    pub fn print(ll: LinkedList) {
        println!("{}", ll.head.data);
        //println!("{}", ll.head.node.data);
        // ll.head.node = Some(ll.z);
    }

    pub fn insert_after(&self) {
    }
} // END impl LinkedList

fn main() {
    let mut _ll = LinkedList::new();
    LinkedList::print(_ll);
}
