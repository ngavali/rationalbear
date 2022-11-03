use std::{cell::RefCell, rc::Rc};

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

struct Solution {}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match nums.len() {
            0 => None,
            _ => {
                let mid = nums.len() / 2 as usize;

                Some(Rc::new(RefCell::new(TreeNode {
                    val: nums[mid],
                    left: Solution::sorted_array_to_bst(nums[..mid].to_vec()),
                    right: Solution::sorted_array_to_bst(nums[mid + 1..].to_vec()),
                })))
            }
        }
    }

    fn DFS_inorder(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Solution::DFS_inorder(node.left.clone(), result);
            result.push(node.val);
            Self::DFS_inorder(node.right.clone(), result);
        }
    }
}

fn main() {
    let mut result: Vec<i32> = Vec::new();
    Solution::DFS_inorder(
        Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
        &mut result,
    );
    println!("{:?}", result);
}
