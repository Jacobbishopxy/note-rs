fn main() {
    let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);

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

    fn from_vec(vs: Vec<i32>) -> Option<Box<Self>> {
        let mut ln: Option<Box<ListNode>> = None;

        for &i in vs.iter().rev() {
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
    let (mut l1, mut l2) = (l1, l2);
    let mut val_sum = 0;
    let mut res = None;
    let mut next = &mut res;

    loop {
        match (l1, l2) {
            (Some(v1), Some(v2)) => {
                val_sum += v1.val + v2.val;
                l1 = v1.next;
                l2 = v2.next;
            }
            (None, Some(v2)) => {
                val_sum += v2.val;
                l1 = None;
                l2 = v2.next;
            }
            (Some(v1), None) => {
                val_sum += v1.val;
                l1 = v1.next;
                l2 = None;
            }
            (None, None) => {
                break;
            }
        }

        *next = Some(Box::new(ListNode::new(val_sum % 10)));
        val_sum /= 10;

        if let Some(tail_box) = next {
            // 把 next 指向了 res 的 next 字段
            // 下一个循环的解引用直接对 next 字段赋值
            next = &mut tail_box.next;
        }
    }

    if val_sum != 0 {
        *next = Some(Box::new(ListNode::new(1)));
    }

    res
}
