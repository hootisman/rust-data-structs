/*
struct Jaja {
    value: i32,
    value2: String,
}

*/
/* 
 *
 * Linked List
 *
 */
use std::mem;

pub struct List {
    head: Link,
}
enum Link {
    None,
    Some(Box<Node>),
}
struct Node {
    value: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::None }
    }
    pub fn push(&mut self,n:i32){
        let new_node = Box::new(Node {
            value: n,
            next: mem::replace(&mut self.head, Link::None),
        });
        self.head = Link::Some(new_node)
    }
}
fn main() {
    let list1: List = List::new();
}
