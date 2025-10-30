//https://leetcode.com/problems/house-robber-iii/
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

struct Solution;

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let robbed_money_left = Self::dfs(left);
            let robbed_money_right = Self::dfs(right);
            return (
                node.borrow().val + robbed_money_left.1 + robbed_money_right.1,
                robbed_money_left.0.max(robbed_money_left.1)
                    + robbed_money_right.0.max(robbed_money_right.1),
            );
        }
        (0, 0)
    }
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let res = Self::dfs(root);
        res.0.max(res.1)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;
    fn testcases() -> Vec<(Option<Rc<RefCell<TreeNode>>>, i32)> {
        vec![
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
                7,
            ),
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
                9,
            ),
        ]
    }

    use super::Solution;
    #[test]
    fn test_rob() {
        for (root, want) in testcases() {
            assert_eq!(Solution::rob(root), want);
        }
    }
}
