/* Medium
 * https://leetcode.com/problems/find-bottom-left-tree-value/
 *
 */

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

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;
impl Solution {
    fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q = VecDeque::new();
        let mut root = root;
        q.push_back(root);
        let mut left_most = 0i32;
        while !q.is_empty() {
            left_most = q.front().unwrap().as_ref().unwrap().borrow().val;
            for i in 0..q.len() {
                root = q.pop_front().unwrap();
                if root.as_ref().unwrap().borrow().left != None {
                    q.push_back(root.as_ref().unwrap().borrow().left.clone());
                }
                if root.as_ref().unwrap().borrow().right != None {
                    q.push_back(root.as_ref().unwrap().borrow().right.clone());
                }
            }
        }
        left_most
    }

    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
        max_depth: &mut i32,
        last_node_val: &mut i32,
        //last_node: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if *node == None {
            return;
        }

        if depth > *max_depth {
            *max_depth = depth;
            //*last_node = node.clone();
            *last_node_val = node.as_ref().unwrap().borrow().val;
        }

        Solution::dfs(
            &node.as_ref().unwrap().borrow().left,
            depth + 1,
            max_depth,
            last_node_val,
        );
        Solution::dfs(
            &node.as_ref().unwrap().borrow().right,
            depth + 1,
            max_depth,
            last_node_val,
        );
    }
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut max_depth, mut last_node_val) = (0, root.as_ref().unwrap().borrow().val);
        Solution::dfs(&root, 0, &mut max_depth, &mut last_node_val);
        last_node_val
    }
}

fn main() {
    println!("Hello, world!");
}
