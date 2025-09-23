//https://leetcode.com/problems/binary-tree-postorder-traversal/

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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut visited_list = Vec::new();
        if let Some(node) = root {
            let left = Self::postorder_traversal(node.borrow_mut().left.take());
            let right = Self::postorder_traversal(node.borrow_mut().right.take());
            visited_list.extend(left);
            visited_list.extend(right);
            visited_list.push(node.borrow().val);
        }
        visited_list
    }
}

fn main() {
    println!("Hello, world!");
}
