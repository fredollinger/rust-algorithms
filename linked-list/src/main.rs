pub struct LinkedList {
       pub data: i32,
       pub node: Option<Box<LinkedList>>
}

fn main() {

   let z: Box<LinkedList> = Box::new(LinkedList {
        data: 0,
        node: None
    });

/*
    let head = LinkedList {
        data: 1,
        node: Some(Box(z))
    };
    */

    // println!("Hello, world! {}", head.data);
}
