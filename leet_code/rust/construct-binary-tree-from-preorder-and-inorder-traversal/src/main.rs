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
        inorder: &Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut this_root_node = TreeNode::new(preorder[idx.0]);
        *idx = (idx.0 + 1, idx.1);

        if this_root_node.val != inorder[idx.1] {
            this_root_node.left = Self::tree_constructor(idx, preorder, inorder);
        }

        if this_root_node.val != inorder[idx.1] {
            this_root_node.right = Self::tree_constructor(idx, preorder, inorder);
        }
        *idx = (idx.0, idx.1 + 1);
        Some(Rc::new(RefCell::new(this_root_node)))
    }

    fn tree_constructor_pass1(
        preorder_idx: (usize, usize),
        inorder_idx: (usize, usize),
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder_idx.0 > preorder_idx.1 || inorder_idx.0 > inorder_idx.1 || preorder_idx.1 >= preorder.len() || inorder_idx.1>= inorder.len() {
            return None;
        }
        println!("{preorder_idx:?} {inorder_idx:?} {:?} {:?}", &preorder[preorder_idx.0..=preorder_idx.1], &inorder[inorder_idx.0..=inorder_idx.1]);
        let mut this_root_node = TreeNode::new(preorder[preorder_idx.0]);
        let mut in_order_root_idx = inorder_idx.0;
        //Find root node in the inorder
        let mut nodes= 0;
        while inorder[in_order_root_idx] != this_root_node.val {
            println!("  -> Finding root : {in_order_root_idx}");
            in_order_root_idx += 1;
            nodes += 1;
        }
        println!(" -> Root node location {in_order_root_idx} {}", inorder[in_order_root_idx]);
        //Now we have both left and right part
        //Process left part
        if in_order_root_idx > 0 {
            println!("  --> Go left {:?} {:?}", (preorder_idx.0 + 1, preorder_idx.0 + nodes), (inorder_idx.0, in_order_root_idx - 1));
            let left = Self::tree_constructor_pass1(
                (preorder_idx.0 + 1, preorder_idx.0+nodes),
                (inorder_idx.0, in_order_root_idx - 1),
                preorder,
                inorder,
            );
            this_root_node.left = left;
        }
        //Process right part
        println!("  --> Go right {:?} {:?}", (in_order_root_idx + 1, preorder_idx.1), (in_order_root_idx + 1, inorder_idx.1));
        let right = Self::tree_constructor_pass1(
            (preorder_idx.0 + nodes + 1, preorder_idx.1),
            (in_order_root_idx + 1, inorder_idx.1),
            preorder,
            inorder,
        );
        this_root_node.right = right;
        Some(Rc::new(RefCell::new(this_root_node)))
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let res = Self::tree_constructor_pass1(
            (0, preorder.len() - 1),
            (0, inorder.len() - 1),
            &preorder,
            &inorder,
        );
        println!("{res:#?}");
        //Self::tree_constructor(&mut (0, 0), &preorder, &inorder)
        res
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
                vec![3, 9, 20, 15, 7],
                vec![9, 3, 15, 20, 7],
                vec![3, 9, 20, 15, 7],
            ),
            (vec![-1], vec![-1], vec![-1]),
            (vec![1,2,3], vec![3,2,1], vec![1,2,3])
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
