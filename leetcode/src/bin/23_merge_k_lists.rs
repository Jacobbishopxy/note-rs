use leetcode::util::ListNode;

fn main() {
    let a1 = ListNode::from_vec(vec![1, 4, 5]);
    let a2 = ListNode::from_vec(vec![1, 3, 4]);
    let a3 = ListNode::from_vec(vec![2, 6]);

    let foo = vec![a1, a2, a3];

    println!("result: {:?}", merge_k_lists(foo));
}

pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
        return None;
    }

    let mut len = lists.len();

    while len > 1 {
        for i in 0..len / 2 {
            // 头尾项合并，形成新的 lists。lists 的长度为奇数时，最中间的项保留变为新 lists 的最后一项。
            lists[i] = M::merge_two_lists(lists[i].take(), lists[len - i - 1].take());
        }
        // list 的长度为奇数时，+1 使其成为偶数再被 2 除，作为下一次 while loop 的执行条件
        len = (len + 1) / 2;
    }

    lists[0].take()
}

pub struct M;

impl M {
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
