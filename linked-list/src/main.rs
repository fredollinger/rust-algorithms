pub struct Node {
       pub data: i32,
       pub node: Option<Box<Node>>
}

impl Node {
    pub fn new() -> Node {
       let z: Box<Node> = Box::new(Node {
            data: 0,
            node: None
       });

        let head = Node {
            data: 0,
            node: Some(z)
        };
        head
    }
} // END impl Node

struct LinkedList {
    junk: i32, // remove me
    head: Box<Node>
    //z: Box<Node>
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
        // ll.head.node = Some(ll.z);
    }
} // END impl LinkedList

fn main() {
    let mut _ll = LinkedList::new();
    LinkedList::print(_ll);
}
