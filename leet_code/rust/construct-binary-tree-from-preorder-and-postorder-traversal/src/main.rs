//https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/

struct Solution;

//Definition for a binary tree node.
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

impl Solution {
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            Self::inorder_traversal(node.borrow_mut().left.take());
            println!("{}", node.borrow().val);
            Self::inorder_traversal(node.borrow_mut().right.take());
        }
    }
    fn tree_constructor(
        idx: &mut (usize, usize),
        preorder: &Vec<i32>,
        postorder: &Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut this_root_node = TreeNode::new(preorder[idx.0]);
        *idx = (idx.0 + 1, idx.1);

        if this_root_node.val != postorder[idx.1] {
            this_root_node.left = Self::tree_constructor(idx, preorder, postorder);
        }

        if this_root_node.val != postorder[idx.1] {
            this_root_node.right = Self::tree_constructor(idx, preorder, postorder);
        }
        *idx = (idx.0, idx.1 + 1);
        Some(Rc::new(RefCell::new(this_root_node)))
    }

    fn tree_constructor_pass1(
        preorder_idx: (usize, usize),
        postorder_idx: (usize, usize),
        preorder: &Vec<i32>,
        postorder: &Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder_idx.0 > preorder_idx.1 || postorder_idx.0 > postorder_idx.1 {
            return None;
        }
        let mut this_root_node = TreeNode::new(preorder[preorder_idx.0]);
        //We have a left node here!
        if preorder_idx.0 + 1 <= preorder_idx.1 && postorder_idx.1 > postorder_idx.0 {
            //Search for right node
            let mut right_node_idx = preorder_idx.0;
            while right_node_idx < preorder_idx.1
                && preorder[right_node_idx] != postorder[postorder_idx.1 - 1]
            {
                right_node_idx += 1;
            }
            //Lets partition left and right parts and call construction from here
            let left = Self::tree_constructor_pass1(
                (preorder_idx.0 + 1, right_node_idx - 1),
                (
                    postorder_idx.0,
                    postorder_idx.1 - (preorder_idx.1 - right_node_idx) - 2,
                ),
                preorder,
                postorder,
            );
            this_root_node.left = left;
            let right = Self::tree_constructor_pass1(
                (right_node_idx, preorder_idx.1),
                (
                    postorder_idx.1 - (preorder_idx.1 - right_node_idx) - 1,
                    postorder_idx.1 - 1,
                ),
                preorder,
                postorder,
            );
            this_root_node.right = right;
        }
        Some(Rc::new(RefCell::new(this_root_node)))
    }

    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::tree_constructor_pass1( (0, preorder.len() - 1), (0, postorder.len() - 1), &preorder, &postorder,)
        Self::tree_constructor(&mut (0, 0), &preorder, &postorder)
    }

    pub fn bfs(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut stack = VecDeque::new();
        if let Some(root) = tree {
            stack.push_back(root);
        }
        while let Some(node) = stack.pop_front() {
            ans.push(node.borrow().val);
            if let Some(left) = node.borrow_mut().left.take() {
                stack.push_back(left);
            }
            if let Some(right) = node.borrow_mut().right.take() {
                stack.push_back(right);
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");

    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: None,
                right: None,
            }))),
        }))),
    })));
    Solution::inorder_traversal(tree);
}

#[cfg(test)]
mod tests {

    use super::Solution;

    fn testcases() -> Vec<(Vec<i32>, Vec<i32>, Vec<i32>)> {
        vec![
            (
                vec![1, 2, 3, 4, 5, 7, 6, 8],
                vec![3, 5, 4, 2, 6, 8, 7, 1],
                vec![1, 2, 7, 3, 4, 6, 8, 5],
            ),
            (
                vec![1, 2, 4, 5, 3, 6, 7],
                vec![4, 5, 2, 6, 7, 3, 1],
                vec![1, 2, 3, 4, 5, 6, 7],
            ),
            (
                vec![1, 2, 4, 5, 3, 6, 7],
                vec![4, 5, 2, 6, 7, 3, 1],
                vec![1, 2, 3, 4, 5, 6, 7],
            ),
            (vec![1], vec![1], vec![1]),
        ]
    }

    #[test]
    fn test_construct_from_pre_post() {
        use std::cell::RefCell;
        use std::rc::Rc;

        for (preorder, postorder, want) in testcases() {
            assert_eq!(
                Solution::bfs(Solution::construct_from_pre_post(preorder, postorder)),
                want
            );
        }
    }
}
