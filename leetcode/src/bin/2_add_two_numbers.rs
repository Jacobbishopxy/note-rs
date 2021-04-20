fn main() {
    let l1 = ListNode::create(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = ListNode::create(vec![9, 9, 9, 9]);

    println!("{:?}\n", l1);
    println!("{:?}\n", l2);
    println!("{:?}\n", add_two_numbers(l1, l2));
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

    fn create(vals: Vec<i32>) -> Option<Box<Self>> {
        let mut ln: Option<Box<ListNode>> = None;

        for &i in vals.iter().rev() {
            match ln {
                v @ Some(_) => ln = Some(Box::new(ListNode { val: i, next: v })),
                None => ln = Some(Box::new(ListNode::new(i))),
            }
        }

        ln
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l = (l1, l2);
    let mut val_sum;
    let mut res = None;
    let mut tail = &mut res;
    let mut step_forward = 0;

    loop {
        val_sum = step_forward;

        match l {
            (Some(v1), Some(v2)) => {
                val_sum += v1.val + v2.val;
                l = (v1.next, v2.next);
            }
            (None, Some(v2)) => {
                val_sum += v2.val;
                l = (None, v2.next);
            }
            (Some(v1), None) => {
                val_sum += v1.val;
                l = (v1.next, None);
            }
            (None, None) => {
                break;
            }
        }

        if val_sum >= 10 {
            step_forward = 1;
        } else {
            step_forward = 0;
        }
        val_sum = val_sum % 10;

        *tail = Some(Box::new(ListNode::new(val_sum)));
        if let Some(tail_box) = tail {
            tail = &mut tail_box.next;
        }
    }

    if step_forward == 1 {
        *tail = Some(Box::new(ListNode::new(1)));
    }

    res
}
