#![allow(unused)]

mod first;

use first::List;

fn main() {
    let mut list = List::new();
    list.push(30);
    println!("{:#?}", list);
}