//https://leetcode.com/problems/reverse-linked-list-ii/
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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let (mut left, mut right) = (left - 1, right - left + 1);
        let mut container = Some(Box::new(ListNode { next: head, val: 0 }));
        let mut curr = container.as_mut();
        if left != right || curr.as_ref().unwrap().next.is_some() {
            while left > 0 && curr.is_some() {
                if let Some(node) = curr {
                    curr = node.as_mut().next.as_mut().take();
                    left -= 1;
                }
            }
            let mut next_node = None;
            let mut nx = curr.as_mut().unwrap().next.take();
            while right > 0 && nx.is_some() {
                if let Some(mut node) = nx {
                    nx = node.next.take();
                    node.next = next_node;
                    next_node = Some(node);
                    right -= 1;
                }
            }
            if curr.is_some() {
                curr.as_mut().unwrap().next = next_node;
                while let Some(node) = curr {
                    if node.as_mut().next.is_none() {
                        curr = Some(node);
                        break;
                    } else {
                        curr = node.as_mut().next.as_mut().take();
                    }
                }
                curr.as_mut().unwrap().next = nx;
            }
        }
        container.as_mut().unwrap().next.take()
    }
}

fn main() {
    for (head, left, right, want) in testcases() {
        println!("--------------------");
        println!(
            "Solution:\nwant={:#?}\ngot={:#?}",
            want,
            Solution::reverse_between(head, left, right)
        );
    }
}

type TestCases = Vec<(Option<Box<ListNode>>, i32, i32, Option<Box<ListNode>>)>;

fn testcases() -> TestCases {
    vec![
        (
            Some(Box::new(ListNode {
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
            })),
            2,
            4,
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode::new(5))),
                        })),
                    })),
                })),
            })),
        ),
        (
            Some(Box::new(ListNode::new(5))),
            1,
            1,
            Some(Box::new(ListNode::new(5))),
        ),
        (
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode {
                                    val: 6,
                                    next: Some(Box::new(ListNode {
                                        val: 7,
                                        next: Some(Box::new(ListNode::new(8))),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
            4,
            7,
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 7,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode {
                                        val: 4,
                                        next: Some(Box::new(ListNode::new(8))),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::{testcases, Solution};

    #[test]
    fn test_reverse_between() {
        for (head, left, right, want) in testcases() {
            assert_eq!(Solution::reverse_between(head, left, right), want);
        }
    }
}
