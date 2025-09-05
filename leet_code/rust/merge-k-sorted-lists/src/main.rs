//https://leetcode.com/problems/merge-k-sorted-lists/
struct Solution {}
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
    fn merge_two_lists(
        mut list_a: Option<Box<ListNode>>,
        mut list_b: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list_a.is_some() {
            if list_b.is_some() {
                let mut res = Box::new(ListNode::new(0));
                let mut res_head = res.as_mut();
                while list_a.is_some() && list_b.is_some() {
                    if list_a.as_ref().unwrap().val <= list_b.as_ref().unwrap().val {
                        let next = list_a.as_mut().unwrap().next.take();
                        res_head.next = list_a;
                        list_a = next;
                    } else {
                        let next = list_b.as_mut().unwrap().next.take();
                        res_head.next = list_b;
                        list_b = next;
                    }
                    res_head = res_head.next.as_mut().unwrap();
                }
                res_head.next = list_a.or(list_b);
                return res.next;
            }
            return list_a;
        }
        list_b
    }

    fn divide(
        lists: &Vec<Option<Box<ListNode>>>,
        start: usize,
        end: usize,
    ) -> Option<Box<ListNode>> {
        if start == end {
            return lists[start].clone();
        }
        let mid = start + (end - start) / 2;
        Self::merge_two_lists(
            Self::divide(&lists, start, mid),
            Self::divide(&lists, mid + 1, end),
        )
    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let i = 0;
        if lists.len() >= 1 {
            return Solution::divide(&lists, 0, lists.len() - 1);
        }
        None
    }
}

fn main() {
    println!("Hello, world!");
    for testcase in testcase() {
        println!(
            "{:#?}",
            Solution::merge_k_lists(vec![testcase.0, testcase.2])
        );
    }
}

fn testcase() -> Vec<(Option<Box<ListNode>>, i32, Option<Box<ListNode>>)> {
    let list_1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 10,
                    next: Some(Box::new(ListNode::new(12))),
                })),
            })),
        })),
    }));
    let list_1_o = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 13,
                    next: Some(Box::new(ListNode::new(16))),
                })),
            })),
        })),
    }));
    let list_2_o = Some(Box::new(ListNode {
        val: 8,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 11,
                next: Some(Box::new(ListNode {
                    val: 14,
                    next: Some(Box::new(ListNode::new(15))),
                })),
            })),
        })),
    }));

    vec![(list_1.clone(), 2, list_1_o), (list_1.clone(), 3, list_2_o)]
}

#[cfg(test)]
mod tests {
    use crate::{testcase, Solution};
}
