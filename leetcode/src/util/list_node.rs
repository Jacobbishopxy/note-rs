#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vs: Vec<i32>) -> Option<Box<ListNode>> {
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

    pub fn to_vec(&self) -> Vec<i32> {
        let mut res = vec![self.val];
        let mut next = &self.next;

        while let Some(v) = next {
            res.push(v.val);
            next = &v.next;
        }

        res
    }
}
