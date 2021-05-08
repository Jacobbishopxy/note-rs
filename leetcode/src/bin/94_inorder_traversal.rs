use leetcode::util::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    println!("{:?}", inorder_traversal(root));
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];

    if let r @ Some(_) = root {
        inorder(r, &mut res);
    }

    res
}

fn inorder(tree_node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(r) = tree_node {
        inorder(r.borrow().left.clone(), res);
        res.push(r.borrow().val.clone());
        inorder(r.borrow().right.clone(), res);
    }

    return;
}
