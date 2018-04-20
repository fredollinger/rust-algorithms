pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: List,
}

/*
impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}
*/

/*
fn foo(arg1: Type1, arg2: Type2) -> ReturnType {
        // body
}
*/

fn main() {
    println!("Hello, world!");
}
