use std::cell::RefCell;
use std::rc::Rc;

use leetcode::util::TreeNode;

fn main() {
    let root = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        }))),
    };

    println!("{:?}", is_symmetric(Some(Rc::new(RefCell::new(root)))));

    let root = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    };

    println!("{:?}", is_symmetric(Some(Rc::new(RefCell::new(root)))));
}

fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(rt) = root {
        return is_equal(rt.borrow().left.clone(), rt.borrow().right.clone());
    }

    true
}

fn is_equal(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (l, r) {
        (Some(l), Some(r)) => {
            if l.borrow().val != r.borrow().val {
                false
            } else {
                is_equal(l.borrow().left.clone(), r.borrow().right.clone())
                    && is_equal(l.borrow().right.clone(), r.borrow().left.clone())
            }
        }
        (None, None) => true,
        _ => false,
    }
}
