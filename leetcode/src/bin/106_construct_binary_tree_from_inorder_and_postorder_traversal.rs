use std::cell::RefCell;
use std::rc::Rc;

use leetcode::util::TreeNode;

fn main() {
    let po = vec![9, 3, 15, 20, 7];
    let io = vec![9, 15, 7, 20, 3];

    println!("{:?}", build_tree(po, io));
}

fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let len = inorder.len() as i32 - 1;
    return build(&inorder, 0, len, &postorder, 0, len);
}

fn build(
    inorder: &Vec<i32>,
    il: i32,
    ir: i32,
    postorder: &Vec<i32>,
    pl: i32,
    pr: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if pl > pr {
        return None;
    }
    let mut root = TreeNode {
        val: postorder[pr as usize],
        left: None,
        right: None,
    };

    let mut rootin = il;
    while rootin < ir && inorder[rootin as usize] != root.val {
        rootin += 1;
    }
    let l = rootin - il;
    root.left = build(inorder, il, rootin - 1, postorder, pl, pl + l - 1);
    root.right = build(inorder, rootin + 1, ir, postorder, pl + l, pr - 1);

    return Some(Rc::new(RefCell::new(root)));
}
