pub struct Node {
       pub data: i32,
       pub node: Option<Box<Node>>
}

pub struct LinkedList {
    junk: i32, // remove me
    head: Box<Node>,
    z: Box<Node>
}

impl LinkedList {
    pub fn new() -> LinkedList {
        let ll = LinkedList {
            junk: 0,
            z: Box::new(Node {
                data: 0,
                node: None
            }),

            // https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct
            head: Box::new(Node {
                data: 0,
                node: Some(Self.z)
                // node: None
            })
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
            //node: None
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

        let _n = &ll.node;
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
    let _ll = LinkedList::new();
    /*
    let z = Node {
         data: 0,
         node: None
    };

    print_list(&ll);
    */
}
