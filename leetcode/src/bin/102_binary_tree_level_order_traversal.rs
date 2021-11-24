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

    println!("{:?}", level_order(Some(Rc::new(RefCell::new(root)))));
}

fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];

    level_order_traverse(root, 0, &mut res);

    return res;
}

fn level_order_traverse(
    root: Option<Rc<RefCell<TreeNode>>>,
    depth: usize,
    res: &mut Vec<Vec<i32>>,
) {
    if let Some(rt) = root {
        if depth >= res.len() {
            res.push(vec![]);
        }
        res[depth].push(rt.borrow().val);
        level_order_traverse(rt.borrow().left.clone(), depth + 1, res);
        level_order_traverse(rt.borrow().right.clone(), depth + 1, res);
    }

    return;
}
