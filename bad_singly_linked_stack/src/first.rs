#![allow(unused)]
#[derive(Debug)]
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

fn main() {
    let list: List = List::Elem(1, Box::new(List::Elem(2, Box::new(List::Empty))));
    println!("{:#?}", list);
}