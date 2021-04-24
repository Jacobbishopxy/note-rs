fn main() {
    let foo = ListNode::create(vec![1, 2, 3, 4, 5]);

    println!("result: {:?}", foo);
    println!("result: {:?}", remove_nth_from_end(foo, 2));
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn create(vs: Vec<i32>) -> Option<Box<ListNode>> {
        if vs.len() == 0 {
            return None;
        }

        let mut res: Option<Box<ListNode>> = None;
        let mut next = &mut res;

        for &v in vs.iter() {
            *next = Some(Box::new(ListNode::new(v)));

            if let Some(b) = next {
                next = &mut b.next;
            }
        }

        res
    }
}

fn remove(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
    match head {
        Some(tail) => {
            let (rest, tail_idx) = remove(tail.next, n);
            let res = if tail_idx == n {
                rest
            } else {
                Some(Box::new(ListNode {
                    val: tail.val,
                    next: rest,
                }))
            };
            (res, tail_idx + 1)
        }
        None => {
            return (None, 1);
        }
    }
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let (res, _) = remove(head, n);
    res
}
