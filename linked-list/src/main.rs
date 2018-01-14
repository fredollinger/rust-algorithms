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
    head: Box<Node>,
    z: Box<Node>
}

impl LinkedList {
    pub fn new() -> LinkedList {
        let ll = LinkedList {
            junk: 4,
            z: Box::new(Node {
                data: 4,
                node: None
            }),

            // https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct
            head: Box::new(Node {
                data: 0,
                //node: Some(z)
                node: None
            })
        };
        ll
    } // END LinkedList new()

    pub fn init(ll: &mut LinkedList) {
        // ll.head.node = Some(ll.z);
    }
} // END impl LinkedList

fn main() {
    let mut _ll = LinkedList::new();
    LinkedList::init(&mut _ll);
}
