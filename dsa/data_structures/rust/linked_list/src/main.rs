#[derive(Debug, Clone)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Node {
        Node { val, next: None }
    }

    fn add(&mut self, nn: Node) {
        let mut pointer = self;
        loop {
            match pointer.next {
                Some(ref mut next) => pointer = next,
                None => {
                    pointer.next = Some(Box::new(nn));
                    break;
                }
            }
        }
    }
}

fn main() {
    let mut ll = Node::new(1);
    ll.add(Node { val: 2, next: None });
    ll.add(Node { val: 3, next: None });
    ll.add(Node { val: 4, next: None });
    ll.add(Node { val: 5, next: None });

    println!("{:?}", ll);
}
