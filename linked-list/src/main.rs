pub struct LinkedList {
       pub data: i32,
       pub node: Option<Box<LinkedList>>
}

impl LinkedList {
    pub fn new() -> LinkedList {
       let z: Box<LinkedList> = Box::new(LinkedList {
            data: 0,
            node: None
        });

        let head = LinkedList {
            data: 1,
            node: Some(z)
        };
        head
    }
} // END impl LinkedList

pub fn print_list(ll: &LinkedList) {
    loop {
        println!("{}", ll.data);
         match ll.node {
            Some(_) => println!("some"),
            None => println!("tail")
        }

        /*
        let n = &ll.node;
        match &n {
            Some(x) => println!("some"),
            None => println!("tail")
        }
        */
        break;
    }
}

fn main() {
    let ll = LinkedList::new();
    print_list(&ll);
}
