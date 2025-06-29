use ok_singly_linked_stack::second::List;

#[test]
fn test_push_and_pop() {
    let mut list = List::new();
    list.push(1);
    assert_eq!(list.pop(), Some(1));
}