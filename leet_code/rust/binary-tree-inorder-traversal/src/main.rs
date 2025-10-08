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
    fn lca(root: &Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<i32> {
        if let Some(node) = root {
            if !(node.borrow().val == p || node.borrow().val == q) {
                let left = Self::lca(&node.borrow().left, p, q);
                let right = Self::lca(&node.borrow().right, p, q);
                if right.is_none() {
                    return left;
                }
                if left.is_none() {
                    return right;
                }
            }
            return Some(node.borrow().val);
        }
        None
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        Some(Rc::new(RefCell::new(TreeNode::new(
            Self::lca(&root, p, q).unwrap(),
        ))))
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

    fn testcases() -> Vec<(
        Option<Rc<RefCell<TreeNode>>>,
        Option<Rc<RefCell<TreeNode>>>,
        Option<Rc<RefCell<TreeNode>>>,
        Option<Rc<RefCell<TreeNode>>>,
    )> {
        vec![
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 6,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 7,
                                left: None,
                                right: None,
                            }))),
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 4,
                                left: None,
                                right: None,
                            }))),
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            ),
            (
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 6,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 7,
                                left: None,
                                right: None,
                            }))),
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 4,
                                left: None,
                                right: None,
                            }))),
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            ),
        ]
    }

    use super::Solution;
    #[test]
    fn test_lowest_common_ancestor() {
        for (root, p, q, want) in testcases() {
            assert_eq!(Solution::lowest_common_ancestor(root, p, q), want);
        }
    }
}
