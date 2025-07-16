/* Medium
 * https://leetcode.com/problems/sum-root-to-leaf-numbers/
 */

struct Solution;

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
use std::rc::Rc;

impl Solution {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if node == None {
            return 0;
        }

        if node.as_ref().unwrap().borrow().left == None
            && node.as_ref().unwrap().borrow().right == None
        {
            return sum * 10 + node.as_ref().unwrap().borrow().val;
        }

        Solution::dfs(
            node.as_ref().unwrap().borrow().left.clone(),
            sum * 10 + node.as_ref().unwrap().borrow().val,
        ) + Solution::dfs(
            node.as_ref().unwrap().borrow().right.clone(),
            sum * 10 + node.as_ref().unwrap().borrow().val,
        )
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs(root, 0)
    }
}

fn main() {
    let mut node_9 = TreeNode::new(9);
    let node_5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let mut node_1 = TreeNode::new(1);
    let node_2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    node_1.left = node_2;
    node_9.left = node_5;
    node_9.right = Some(Rc::new(RefCell::new(node_1)));

    let node_0 = TreeNode::new(0);
    let mut root = TreeNode::new(4);
    root.left = Some(Rc::new(RefCell::new(node_9)));
    root.right = Some(Rc::new(RefCell::new(node_0)));
    println!("{root:#?}",);
    println!("{}", Solution::sum_numbers(tree_1().0));
}

fn tree_1() -> (Option<Rc<RefCell<TreeNode>>>, i32) {
    let mut node_9 = TreeNode::new(9);
    node_9.left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    node_9.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    let mut root = TreeNode::new(4);
    root.left = Some(Rc::new(RefCell::new(node_9)));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(0))));

    (Some(Rc::new(RefCell::new(root))), 1026)
}

fn tree_2() -> (Option<Rc<RefCell<TreeNode>>>, i32) {
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    (Some(Rc::new(RefCell::new(root))), 25)
}

#[cfg(test)]
pub(crate) mod tests {

    use super::{tree_1, tree_2, Solution};

    #[test]
    fn test_sum_numbers() {
        let test_cases = vec![tree_1(), tree_2()];
        for (input, output) in test_cases {
            assert_eq!(output, Solution::sum_numbers(input));
        }
    }
}
