use std::rc::Rc;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Rc<Node<T>>>,
    data: T,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&self, data: T) -> Self {
        LinkedList {
            head: Some(Rc::new(Node {
                data,
                next: self.head.clone(),
            }))
        }
    }
}

#[test]
fn test_linked_list() {
    let list_of_nums = LinkedList::new().append(1).append(2);
    println!("nums : {:?}", list_of_nums);

    let list_of_str = LinkedList::new().append("foo").append("bar");
    println!("strings : {:?}", list_of_str);
}

