use std::cell::RefCell;
use std::rc::Rc;

use leetcode::util::TreeNode;

fn main() {
    let po = vec![1, 2, 4, 7, 3, 5, 6, 8];
    let io = vec![4, 7, 2, 1, 5, 3, 8, 6];

    println!("{:?}", build_tree(po, io));
}

fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let len = inorder.len() as i32 - 1;
    return build(&preorder, 0, len, &inorder, 0, len);
}

fn build(
    preorder: &Vec<i32>,
    pl: i32,
    pr: i32,
    inorder: &Vec<i32>,
    il: i32,
    ir: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if pl > pr || il > ir {
        return None;
    }
    let mut root = TreeNode {
        val: preorder[pl as usize],
        left: None,
        right: None,
    };

    let mut rootin = il;
    while rootin <= ir && inorder[rootin as usize] != root.val {
        rootin += 1;
    }
    let l = rootin - il;
    root.left = build(preorder, pl + 1, pl + l, inorder, il, rootin - 1);
    root.right = build(preorder, pl + l + 1, pr, inorder, rootin + 1, ir);

    return Some(Rc::new(RefCell::new(root)));
}
