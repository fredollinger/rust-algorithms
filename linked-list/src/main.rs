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

fn main() {
    let ll = LinkedList::new();
}
