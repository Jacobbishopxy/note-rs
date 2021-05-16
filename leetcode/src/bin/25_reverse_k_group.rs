use leetcode::util::ListNode;

fn main() {
    let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let k = 2;

    let res = reverse_k_group(head, k);
    let ans = res.unwrap().as_ref().to_vec();

    println!("{:?}", ans);
}

type OBL = Option<Box<ListNode>>;

pub fn reverse_k_group(mut head: OBL, k: i32) -> OBL {
    let mut tail = &mut head;
    for _ in 0..k {
        match tail.as_mut() {
            Some(node) => {
                tail = &mut node.next;
            }
            None => {
                return head;
            }
        }
    }
    let tail = reverse_k_group(tail.take(), k);
    reverse(head, tail)
}

fn reverse(mut head: OBL, mut tail: OBL) -> OBL {
    while let Some(mut node) = head {
        head = node.next.take();

        node.next = tail.take();

        tail = Some(node)
    }
    tail
}
