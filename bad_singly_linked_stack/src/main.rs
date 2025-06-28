
// #![allow(unused)]
// #[derive(Debug)]
// enum List<T> {
//     Cons(T, Box<List<T>>),
//     Nil,
// }
use bad_singly_linked_stack::first::List;

fn main() {
    let list: List = List::Elem(1, Box::new(List::Elem(2, Box::new(List::Empty))));
    println!("{:#?}", list);
}
