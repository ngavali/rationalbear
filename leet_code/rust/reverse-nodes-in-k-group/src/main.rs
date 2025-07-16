/* Hard
 * https://leetcode.com/problems/reverse-nodes-in-k-group/
 *
 * Referred this solution
 * https://www.algobreath.com/notes/reverse-nodes-in-k-group-in-rust
 */

struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    fn reverse_ll(
        mut curr_iter: Option<Box<ListNode>>,
        mut prev: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        while let Some(mut curr_node) = curr_iter {
            (curr_node.next, curr_iter) = (prev, curr_node.next.take());
            prev = Some(curr_node);
        }
        prev
    }

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut container = Some(Box::new(ListNode { next: head, val: 0 }));
        let mut curr = container.as_mut();

        let mut cont = true;

        loop {
            let mut start = curr.as_mut().unwrap().next.take();
            if start.is_none() {
                break;
            }
            let mut end = start.as_mut();

            for _ in 0..k - 1 {
                end = end.unwrap().next.as_mut();
                if end.is_none() {
                    cont = false;
                    break;
                }
            }

            match cont {
                true => {
                    let next_start = end.as_mut().unwrap().next.take();
                    curr.as_mut().unwrap().next = Solution::reverse_ll(start, next_start);
                    for _ in 0..k {
                        curr = curr.unwrap().next.as_mut();
                    }
                }
                false => {
                    curr.as_mut().unwrap().next = start;
                    break;
                }
            };
        }

        container.unwrap().next
    }
}

fn main() {
    for (i, (head, k, mut exp_o)) in testcase().into_iter().enumerate() {
        let mut got = Solution::reverse_k_group(head, k);
        println!("Testcase #{i}");
        while let Some(node) = got.clone() {
            if let Some(exp_o_node) = exp_o.clone() {
                assert_eq!(exp_o_node.val, node.val);
                exp_o = exp_o_node.next;
            }
            got = node.next;
        }
    }
}

fn testcase() -> Vec<(Option<Box<ListNode>>, i32, Option<Box<ListNode>>)> {
    let list_1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(5))),
                })),
            })),
        })),
    }));
    let list_1_o = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(5))),
                })),
            })),
        })),
    }));
    let list_2_o = Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode::new(5))),
                })),
            })),
        })),
    }));

    vec![(list_1.clone(), 2, list_1_o), (list_1.clone(), 3, list_2_o)]
}

#[cfg(test)]
mod tests {
    use crate::{testcase, Solution};

    #[test]
    fn test_reverse_k_group() {
        for (i, (head, k, mut exp_o)) in testcase().into_iter().enumerate() {
            println!("Testcase #{i}");
            let mut got = Solution::reverse_k_group(head, k);
            while let Some(node) = got.clone() {
                if let Some(exp_o_node) = exp_o.clone() {
                    assert_eq!(exp_o_node.val, node.val);
                    exp_o = exp_o_node.next;
                }
                got = node.next;
            }
        }
    }
}
