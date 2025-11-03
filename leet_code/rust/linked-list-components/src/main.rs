//https://leetcode.com/problems/linked-list-components/
//No DSU here ! we donot have all elements
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
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut nums_map = vec![false; 10001];
        for n in nums {
            nums_map[n as usize] = true;
        }
        let mut subsets = 0;
        let mut head = head;
        let mut connected = false;
        while let Some(node) = head {
            if nums_map[node.val as usize] {
                if !connected {
                    connected = true;
                    subsets+=1;
                }
            } else {
                connected = false;
            }
            head = node.next;
        }
        subsets
    }
}

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use super::ListNode;
    fn testcases() -> Vec<(Option<Box<ListNode>>, Vec<i32>, i32)> {
        vec![
            (
                Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode { val: 3, next: None })),
                        })),
                    })),
                })),
                vec![0, 1, 3],
                2,
            ),
            (
                Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 3,
                                next: Some(Box::new(ListNode { val: 4, next: None })),
                            })),
                        })),
                    })),
                })),
                vec![0, 3, 1, 4],
                2,
            ),
        ]
    }

    use super::Solution;
    #[test]
    fn test_num_components() {
        for (head, nums, want) in testcases() {
            assert_eq!(Solution::num_components(head, nums), want);
        }
    }
}
