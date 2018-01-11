pub struct Node {
       pub data: i32,
       pub node: Option<Box<Node>>
}

pub struct LinkedList {
    //head: Box<Node>,
    //z: Box<Node>
}

impl LinkedList {
    pub fn new() -> LinkedList {
            let ll: LinkedList = LinkedList {
                /*
               let z = Node {
                    data: 0,
                    node: None
                };

                let head = Node {
                    data: 0,
                    node: Some(z)
                };
                */
        };
        ll
    }
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

pub fn print_list(ll: &Node) {
    loop {
        println!("{}", ll.data);
        /*
        match ll.node {
            Some(_) => println!("some"),
            None => println!("tail")
        }
        */

        let n = &ll.node;
            // &std::option::Option<std::boxed::Box<Node>> is non-empty
        /*
        match n {
            Some(x) => println!("some"),
            None => println!("tail")
        }
        */
        break;
    }
}

fn main() {
    let ll = Node::new();
    print_list(&ll);
}