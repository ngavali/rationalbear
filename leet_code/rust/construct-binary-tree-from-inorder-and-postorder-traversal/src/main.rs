//https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
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
use std::rc::Rc;
use std::collections::{VecDeque,HashMap};

impl Solution {
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            Self::inorder_traversal(node.borrow_mut().left.take());
            println!("{}", node.borrow().val);
            Self::inorder_traversal(node.borrow_mut().right.take());
        }
    }

    fn construct_tree(inorder_idx: (usize,usize), postorder_idx: &mut usize, inorder: &Vec<i32>, postorder: &Vec<i32>, inorder_root_map: &HashMap<i32,usize>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder_idx.0 > inorder_idx.1 {
            return None;
        }
        let mut node = TreeNode::new(postorder[*postorder_idx]);
        if *postorder_idx > 0 {
            *postorder_idx -= 1;
            let inorder_root_idx = inorder_root_map.get(&node.val).unwrap();
            node.right = Self::construct_tree((*inorder_root_idx+1, inorder_idx.1), postorder_idx, inorder, postorder, inorder_root_map);
            if *inorder_root_idx != 0 {
                node.left = Self::construct_tree((inorder_idx.0, *inorder_root_idx-1), postorder_idx, inorder, postorder, inorder_root_map);
            }
        }
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_root_map: HashMap<i32, usize> = inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        Self::construct_tree((0, inorder.len()-1), &mut (postorder.len()-1), &inorder, &postorder, &inorder_root_map) 
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
                vec![9,3,15,20,7],
                vec![9,15,7,20,3],
                vec![3,9,20,15,7],
            ),
            (vec![-1], vec![-1], vec![-1]),
        ]
    }

    #[test]
    fn test_build_tree() {
        use std::cell::RefCell;
        use std::rc::Rc;

        for (preorder, inorder, want) in testcases() {
            assert_eq!(Solution::bfs(Solution::build_tree(preorder, inorder)), want);
        }
    }
}
