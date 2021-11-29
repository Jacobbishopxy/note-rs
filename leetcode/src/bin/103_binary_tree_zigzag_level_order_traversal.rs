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

    println!(
        "{:?}",
        zigzag_level_order(Some(Rc::new(RefCell::new(root))))
    );
}

fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];

    zigzag_level_order_traverse(root, 0, &mut res);

    res
}

fn zigzag_level_order_traverse(
    root: Option<Rc<RefCell<TreeNode>>>,
    depth: usize,
    res: &mut Vec<Vec<i32>>,
) {
    if let Some(rt) = root {
        if depth >= res.len() {
            res.push(Vec::new());
        }

        if depth % 2 == 0 {
            res[depth].push(rt.borrow().val);
        } else {
            res[depth].insert(0, rt.borrow().val);
        }

        zigzag_level_order_traverse(rt.borrow().left.clone(), depth + 1, res);
        zigzag_level_order_traverse(rt.borrow().right.clone(), depth + 1, res);
    }

    return;
}
