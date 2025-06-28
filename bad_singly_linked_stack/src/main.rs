mod first;

use first::List;

fn main() {
    let mut list = List::new();
    list.push(30);
    list.push(60);
    list.push(120);
    println!("{:#?}", list);
}