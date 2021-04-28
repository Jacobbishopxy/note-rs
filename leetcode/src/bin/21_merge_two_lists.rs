fn main() {
    let foo = ListNode::from_vec(vec![1, 2, 4]);
    let bar = ListNode::from_vec(vec![1, 3, 4]);

    println!("result {:?}", merge_two_lists(foo.clone(), bar.clone()));
    println!("result {:?}", Solution2::merge_two_lists(foo, bar));
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

    fn from_vec(vs: Vec<i32>) -> Option<Box<ListNode>> {
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

pub fn merge_two_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    let mut ptr = &mut res;

    loop {
        match (l1.as_mut(), l2.as_mut()) {
            (Some(v1), Some(v2)) => {
                if v1.val < v2.val {
                    //
                    let next = v1.next.take();
                    ptr.as_mut().unwrap().next = l1.take();
                    l1 = next;
                } else {
                    //
                    let next = v2.next.take();
                    ptr.as_mut().unwrap().next = l2.take();
                    l2 = next;
                }
                ptr = &mut ptr.as_mut().unwrap().next;
            }
            (Some(_), None) => {
                //
                ptr.as_mut().unwrap().next = l1.take();
                break;
            }
            (None, Some(_)) => {
                //
                ptr.as_mut().unwrap().next = l2.take();
                break;
            }
            (None, None) => {
                break;
            }
        }
    }

    res.unwrap().next
}

pub struct Solution2;

impl Solution2 {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
                    l.next = Self::merge_two_lists(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Self::merge_two_lists(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }
}
