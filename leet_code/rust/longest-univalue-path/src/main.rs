#![feature(test)]
/* Medium
 * https://leetcode.com/problems/longest-univalue-path/
 */

use std::{cell::RefCell, rc::Rc};
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

impl Solution {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, max_univalue_at_root: &mut i32) -> i32 {
        if node == None {
            return 0;
        }

        let left = node.as_ref().unwrap().borrow().left.clone();
        let right = node.as_ref().unwrap().borrow().right.clone();

        let (mut univalue_l, mut univalue_r) = (
            Solution::dfs(left.clone(), max_univalue_at_root),
            Solution::dfs(right.clone(), max_univalue_at_root),
        );

        if left != None
            && node.as_ref().unwrap().borrow().val == left.as_ref().unwrap().borrow().val
        {
            univalue_l += 1;
        } else {
            univalue_l = 0;
        }
        if right != None
            && node.as_ref().unwrap().borrow().val == right.as_ref().unwrap().borrow().val
        {
            univalue_r += 1;
        } else {
            univalue_r = 0;
        }
        if left != None
            && right != None
            && node.as_ref().unwrap().borrow().val == left.as_ref().unwrap().borrow().val
            && node.as_ref().unwrap().borrow().val == right.as_ref().unwrap().borrow().val
        {
            *max_univalue_at_root = (univalue_l + univalue_r).max(*max_univalue_at_root);
        } else {
            *max_univalue_at_root = univalue_l.max(univalue_r).max(*max_univalue_at_root);
        }
        univalue_l.max(univalue_r)
    }

    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_univalue_at_root = 0;
        let res = Solution::dfs(root.clone(), &mut max_univalue_at_root);
        res.max(max_univalue_at_root)
    }
}

struct OtherSolution;

impl OtherSolution {
    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        parent_node: Option<Rc<RefCell<TreeNode>>>,
        max_univalue_at_root: &mut i32,
    ) -> i32 {
        if node == None {
            return 0;
        }

        let left = node.as_ref().unwrap().borrow().left.clone();
        let right = node.as_ref().unwrap().borrow().right.clone();

        let mut univalue_l = OtherSolution::dfs(
            node.as_ref().unwrap().borrow().left.clone(),
            node.clone(),
            max_univalue_at_root,
        );
        let mut univalue_r = OtherSolution::dfs(
            node.as_ref().unwrap().borrow().right.clone(),
            node.clone(),
            max_univalue_at_root,
        );

        *max_univalue_at_root = (univalue_l + univalue_r).max(*max_univalue_at_root);

        if node.as_ref().unwrap().borrow().val == parent_node.as_ref().unwrap().borrow().val {
            return univalue_l.max(univalue_r) + 1;
        }

        0
    }

    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_univalue_at_root = 0;
        OtherSolution::dfs(root.clone(), root.clone(), &mut max_univalue_at_root);
        max_univalue_at_root
    }
}

fn tree_1() -> (Option<Rc<RefCell<TreeNode>>>, i32) {
    let mut node_4 = TreeNode::new(4);
    node_4.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node_4.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    let mut node_5 = TreeNode::new(5);
    node_5.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    let mut root = TreeNode::new(5);
    root.left = Some(Rc::new(RefCell::new(node_4)));
    root.right = Some(Rc::new(RefCell::new(node_5)));

    (Some(Rc::new(RefCell::new(root))), 2)
}

fn tree_2() -> (Option<Rc<RefCell<TreeNode>>>, i32) {
    let mut node_4 = TreeNode::new(4);
    node_4.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    node_4.right = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    let mut node_5 = TreeNode::new(5);
    node_5.right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(node_4)));
    root.right = Some(Rc::new(RefCell::new(node_5)));

    (Some(Rc::new(RefCell::new(root))), 2)
}

fn tree_3() -> (Option<Rc<RefCell<TreeNode>>>, i32) {
    let mut node_1 = TreeNode::new(1);

    let mut node_1_r = TreeNode::new(1);
    node_1_r.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    let mut node_1_l = TreeNode::new(1);
    node_1_l.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    node_1_l.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    node_1.left = Some(Rc::new(RefCell::new(node_1_l)));
    node_1.right = Some(Rc::new(RefCell::new(node_1_r)));
    let mut root = TreeNode::new(1);
    root.right = Some(Rc::new(RefCell::new(node_1)));

    (Some(Rc::new(RefCell::new(root))), 4)
}
fn tree_4() -> (Option<Rc<RefCell<TreeNode>>>, i32) {
    let mut root = TreeNode::new(1);
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    (Some(Rc::new(RefCell::new(root))), 2)
}

fn test_cases() -> Vec<(Option<Rc<RefCell<TreeNode>>>, i32)> {
    vec![tree_1(), tree_2(), tree_3(), tree_4()]
}

#[cfg(test)]
mod tests {

    use super::{OtherSolution, Solution};

    #[test]
    fn test_longest_univalue_path() {
        for (input, output) in super::test_cases() {
            assert_eq!(output, OtherSolution::longest_univalue_path(input));
        }
    }

    extern crate test;
    use test::Bencher;
    #[bench]
    fn bench_solution(b: &mut Bencher) {
        b.iter(|| {
            for (input, output) in super::test_cases() {
                assert_eq!(output, OtherSolution::longest_univalue_path(input));
            }
        });
    }
    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| {
            for (input, output) in super::test_cases() {
                assert_eq!(output, Solution::longest_univalue_path(input));
            }
        });
    }
}
