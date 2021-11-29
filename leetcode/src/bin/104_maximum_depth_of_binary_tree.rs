use std::cell::RefCell;
use std::rc::Rc;

use leetcode::util::TreeNode;

fn main() {
    let root = TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    };

    println!("{:?}", max_depth(Some(Rc::new(RefCell::new(root)))));
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;

    max_depth_traverse(root, 1, &mut res);

    res
}

fn max_depth_traverse(root: Option<Rc<RefCell<TreeNode>>>, depth: i32, res: &mut i32) {
    if let Some(rt) = root {
        if depth >= *res {
            *res = depth;
        }
        max_depth_traverse(rt.borrow().left.clone(), depth + 1, res);
        max_depth_traverse(rt.borrow().right.clone(), depth + 1, res);
    }

    return;
}
