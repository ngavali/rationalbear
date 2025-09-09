//https://leetcode.com/problems/binary-tree-inorder-traversal/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn in_order_dfs(root: &Option<Rc<RefCell<TreeNode>>>, inorder_list: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::in_order_dfs(&node.borrow().left, inorder_list);
            inorder_list.push(node.borrow().val);
            Self::in_order_dfs(&node.borrow().right, inorder_list);
        }
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut inorder_list = Vec::new();
        Self::in_order_dfs(&root, &mut inorder_list);
        inorder_list
    }
}

fn main() {
    println!("Hello, world!");
}
