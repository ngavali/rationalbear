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

struct Solution {}

impl Solution {
    pub fn add_two_numbers_ng(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut p = l1;
        let mut q = l2;

        let mut carry: i32 = 0;

        while p != None || q != None {
            let sum = match (&p, &q) {
                (Some(l1), Some(l2)) => l1.val + l2.val + carry,
                (Some(l1), None) => l1.val + carry,
                (None, Some(l2)) => l2.val + carry,
                (None, None) => carry,
            };

            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            p = if p != None { p.unwrap().next } else { p };
            q = if q != None { q.unwrap().next } else { q };
        }
        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
        }

        dummy_head.next
    }

    pub fn add_two_numbers (
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut L1 = l1;
        let mut L2 = l2;
        let mut head = ListNode::new(0);
        let mut L3 = &mut head;

        let mut sum = 0;
        while L1 != None || L2 != None || L3.val/10 > 0 {
            sum = match ( &L1, &L2 ) {
                ( Some(node1) , Some(node2) ) => {
                    let v1 = node1.val;
                    let v2 = node2.val;
                    v1 + v2
                },
                ( None, None ) => 0,
                ( Some(node1), None ) => {
                    let v1 = node1.val;
                    v1
                },
                ( None, Some(node2) ) => {
                    let v2 = node2.val;
                    v2 
                }
            } + sum/10;

            println!("Added {}", L3.val%10);

            L3.next = Some(Box::new(ListNode::new(sum%10)));
            L3 = L3.next.as_mut().unwrap();

            println!("Now {}", L3.val);
            if L1 != None { L1 =  L1.unwrap().next }; 
            if L2 != None { L2 =  L2.unwrap().next };

        }
        head.next
    }
}

fn main() {
    let l1: Option<Box<ListNode>> = Some(Box::new(ListNode{val: 2, next: Some(Box::new( ListNode{ val: 4, next: Some(Box::new( ListNode{ val: 3, next : None} )) })) }));
    let l2: Option<Box<ListNode>> = Some(Box::new(ListNode{val: 5, next: Some(Box::new( ListNode{ val: 6, next: Some(Box::new( ListNode{ val: 4, next : None} )) })) }));
    println!("{:?}",    Solution::add_two_numbers(l1, l2));
}
