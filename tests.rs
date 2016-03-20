pub fn main() {
    let vec = NestedList(Elem(0));
    flatten(vec);
}

enum NestedList<T> {
    Elem(t),
    NestedList,
}

use NestedList::{Elem, NestedList};


fn flatten<T>(vec: Vec<T>) -> Vec<T> {
    println!("hello")
}
