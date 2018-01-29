# Exploring Box Using A Linked List in Rust #

As part of my learning of rust and algorithms, I have decided to try to
implement a linked list in rust.

## What is a Linked List? ##

A linked list is a data structure created from nodes. Each node is a struct
which contains the data, in this example, i32, and a pointer to the next
node.

One of the hardest part so far is wrapping my mind around the Box type.

We need a Box for a linked list because it's rusts safe replacement for a
pointer. A normal linked list uses pointers to point to the next node in a
list.

Here's the node structure in C:

````
struct node {
    int data;
    struct node *next;
}
````

It provides 3 rudimentary methods:

* initialize()

* insert_after()

* delete_after()

Here's the same structure in Rust:

```
pub struct Node {
    pub data: i32,
    pub node: Option<Box<Node>>
}
```

I created a public structure with public data. In this case, our data is going
to be a 32 bit integer, but in the future, I could use a generic type.

The node variable points to more nodes which is how we link our list together.
The head will point to the next node and so on all the way till we get to z.

And here's the LinkedList:

```
struct LinkedList {
    pub head: Box<Node>
}
```

```
pub fn new() -> LinkedList {
    let ll = LinkedList {
        z: Box::new(Node {
            data: 0,
            node: None
        }),

        head: Box::new(Node {
            data: 0,
            //node: Some(Self.z)
            //node: None
        })
    };
    ll
} // END LinkedList new()
