use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct DoubleLinkedList<T> {
    head: Option<Rc<DoubleLinkedListNode<T>>>
}

#[derive(Debug)]
struct DoubleLinkedListNode<T> {
    // Rc类型和&引用一样，提供了“共享性”, 允许多个指针指向同一块内存。
    // 因此它必然不能提供“可变性”。它没有违反“内存安全”原则，
    // 它没有设计直接修改内部数据的成员方法，每个所有者对内部数据只有只读功能，因此，它是安全的
    next: Option<Rc<DoubleLinkedListNode<T>>>,
    prev: RefCell<Option<Weak<DoubleLinkedListNode<T>>>>,
    data: T,
}

impl<T> DoubleLinkedList<T> {
    fn new() -> Self {
        DoubleLinkedList { head: None }
    }

    fn append(&mut self, data: T) -> Self {
        let new_node = Rc::new(DoubleLinkedListNode {
            data,
            // Rc指针是没有实现Copy trait的，如果使用直接赋值方式，那么会执行move语义，导致前一个指针从此失效，后一个指针开始起作用，而且引用计数值不变。
            // 如果需要创造新的Rc指针，必须手工调用clone()函数，此时引用计数值才会加1。当某个Rc指针失效时，会导致引用计数值减1。当引用计数值减到0的时候，共享内存空间才会被释放。
            next: self.head.clone(),
            prev: RefCell::new(None),
        });

        match self.head.clone() {
            Some(node) => {
                // Rc<T>指针的原理: 它指向的内容，不仅包括了在堆上分配的T类型的变量，而且还（至少）包括一个整数类型的引用计数值。
                // 标准库中的实现方式，其实包括了两个整数值，一个表示强引用的个数，
                // 一个表示弱引用的个数。每次调用 Rc::clone(&self) 方法的时候，不仅要把指针本身复制一份，还要把指向的计数值加一。
                // node.prev = Some(Rc::downgrade(&new_node)) // 相当于修改Rc指针，这里是node，里面的值，不允许
                // 但是，Rc指针并未提供可变性，那么怎么才能使用一个没有写权限的指针，修改它所指向的值呢？
                // 所以，这个计数值，本身必须具备“内部可变性”，在这里，使用 Cell<T> 类型就是非常合适的。
                // 标准库中的引用计数值，就是使用的 Cell 类型。
                // 同理,在某些场景下，需要为了生命周期管理的方便，选择具有“共享性”特点的指针，同时又需要通过这样的指针修改所指向的内容。
                // 那么，就需要用 &/Rc/Arc 指向一个具备“内部可变性”的类型。
                // 所以，这里需要Cell类型，Cell 类型在许多情况下都非常有用。
                // 但它有一个重要限制：它里面只能包含具有 Copy 属性的类型。它的对外方法 get/set 都是基于这个前提设计的。
                // 那么对于不具备 Copy 属性的类型，又需要内部可变性怎么办？就需要RefCell类型
                let mut prev = node.prev.borrow_mut();
                *prev = Some(Rc::downgrade(&new_node))
            }
            None => {}
        }

        DoubleLinkedList {
            head: Some(new_node)
        }
    }
}

#[test]
fn test_double_linked_list() {
    let list_of_nums = DoubleLinkedList::new().append(1).append(2).append(3);
    println!("nums: {:?}", list_of_nums)
}