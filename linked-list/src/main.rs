pub struct Node {
       pub data: i32,
       pub node: Option<Box<Node>>
}

struct LinkedList {
    pub head: Box<Node>
}

impl LinkedList {
    pub fn new() -> LinkedList {

        let z: Box<Node> = Box::new(Node {
            data: 0,
            node: None
        });

        let ll = LinkedList {
            head: Box::new(Node {
                data: 4,
                node: Some(z)
            })
        };
        ll
    } // END LinkedList new()

    pub fn print(ll: &mut LinkedList) {
        println!("{}", ll.head.data);
        /*
        if ll.head.node.is_some() {
            println!("{}", ll.head.node.unwrap().data);
        }
        else {
            println!("head points to nothing");
        }
        */
        ll.head.node = Some(ll.head.node.unwrap());
    }

    // NOTE THIS IS TOTALLY BROKEN FOR NOW
    // We need to actually find _n and insert after that
    pub fn insert_after_head(ll: &mut LinkedList, d: i32, _n: Option<Box<Node>>)
    {
       let last = ll.head.node.take();
       ll.head.node = Some(Box::new(Node {
            data: d,
            node: last
        }));
    }
} // END impl LinkedList

fn main() {
    let mut _ll = LinkedList::new();
    //_ll.print();
    let n = _ll.head.node.take();
    LinkedList::insert_after_head(&mut _ll, 1, n);
    LinkedList::print(&mut _ll);
    // LinkedList::insert_after_head(&mut _ll, 2, n);
    // LinkedList::insert_after_head(&mut _ll, 3, n);
}
