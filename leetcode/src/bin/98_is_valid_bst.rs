use leetcode::util::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let root = TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    };
    let root = Some(Rc::new(RefCell::new(root)));

    println!("{:?}", is_valid_bst(root));
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    recursive(&root, i64::MIN, i64::MAX)
}

fn recursive(root: &Option<Rc<RefCell<TreeNode>>>, lower: i64, upper: i64) -> bool {
    match root {
        Some(r) => {
            let val: i64 = r.borrow().val as i64;
            if val as i64 <= lower || val as i64 >= upper {
                false
            } else {
                recursive(&r.borrow().left, lower, val as i64)
                    && recursive(&r.borrow().right, val as i64, upper)
            }
        }
        None => true,
    }
}
