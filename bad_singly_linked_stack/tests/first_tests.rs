use bad_singly_linked_stack::first::List;

#[test]
fn check_empty_list_behaves_right() {
    let mut list = List::new();
    assert_eq!(list.pop(), None);
}

#[test]
fn check_normal_removal() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
}

#[test]
fn check_push_more() {
    let mut list = List::new();
    list.push(4);
    list.push(5);
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));
}

#[test]
fn check_exhaustion() {
    let mut list = List::new();
    list.push(1);
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}