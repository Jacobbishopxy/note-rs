//! fifth.rs
//! TMLL chapter 5

pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push(&'a mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem,
            // 当你 push 到尾部，next 总是 None
            next: None,
        });

        // 把 box 放到正确的位置，并获取其 Node 的一个引用
        let new_tail = match self.tail.take() {
            Some(old_tail) => {
                // 如果旧尾部存在，更新它使其指向新尾部
                old_tail.next = Some(new_tail);
                old_tail.next.as_deref_mut()
            }
            None => {
                // 否者，更新头部指向新尾部
                self.head = Some(new_tail);
                self.head.as_deref_mut()
            }
        };

        self.tail = new_tail;
    }

    pub fn pop(&'a mut self) -> Option<T> {
        // 获取 list 的现有头部
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            // 如果没有头部，确认设定尾部为 None
            if self.head.is_none() {
                self.tail = None;
            }

            head.elem
        })
    }
}

mod test_fifth {
    // use super::List;
    // #[test]
    // fn basics() {
    //     let mut list = List::new();

    //     // Check empty list behaves right
    //     assert_eq!(list.pop(), None);

    //     // Populate list
    //     list.push(1);
    //     list.push(2);
    //     list.push(3);

    //     // Check normal removal
    //     assert_eq!(list.pop(), Some(1));
    //     assert_eq!(list.pop(), Some(2));

    //     // Push some more just to make sure nothing's corrupted
    //     list.push(4);
    //     list.push(5);

    //     // Check normal removal
    //     assert_eq!(list.pop(), Some(3));
    //     assert_eq!(list.pop(), Some(4));

    //     // Check exhaustion
    //     assert_eq!(list.pop(), Some(5));
    //     assert_eq!(list.pop(), None);
    // }
}
