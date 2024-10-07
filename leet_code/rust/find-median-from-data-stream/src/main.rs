//At most 5*DATA_SIZE^4 calls?
const DATA_SIZE: usize = 25000;

struct Heap {
    length: usize,
    data: [i64; DATA_SIZE],
}

impl Heap {
    fn new() -> Self {
        Heap {
            length: 0,
            data: [0; DATA_SIZE],
        }
    }

    fn push(&mut self, element: i64) {
        self.data[self.length] = element;
        self.length += 1;
        let mut pos = self.length - 1;
        let mut parent_pos = pos;
        while pos > 0 {
            parent_pos = (parent_pos - 1) >> 1;
            if self.data[parent_pos] > self.data[pos] {
                break;
            }
            (self.data[parent_pos], self.data[pos]) = (self.data[pos], self.data[parent_pos]);
            pos = parent_pos;
        }
    }

    fn peek(&self) -> Option<i64> {
        match self.length {
            0 => None,
            _ => Some(self.data[0]),
        }
    }

    fn pop(&mut self) -> Option<i64> {
        match self.length {
            0 => None,
            _ => {
                self.length -= 1;
                (self.data[0], self.data[self.length]) = (self.data[self.length], self.data[0]);
                let (mut largest, mut pos, mut l) = (0, 0, 1);
                loop {
                    l = (pos << 1) | 0x1;
                    if l < self.length && self.data[l] > self.data[largest] {
                        largest = l;
                    }
                    if l + 1 < self.length && self.data[l + 1] > self.data[largest] {
                        largest = l + 1;
                    }
                    if largest == pos {
                        break;
                    }
                    (self.data[pos], self.data[largest]) = (self.data[largest], self.data[pos]);
                    pos = largest;
                }
                Some(self.data[self.length])
            }
        }
    }
}

struct MedianFinder {
    left_numbers: Heap,
    right_numbers: Heap,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            left_numbers: Heap::new(),
            right_numbers: Heap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.left_numbers.length == 0 || (num as i64) < self.left_numbers.peek().unwrap() {
            self.left_numbers.push(num as i64);
        } else {
            self.right_numbers.push(-num as i64);
        }

        if (self.left_numbers.length as i64 - self.right_numbers.length as i64).abs() > 1 {
            if self.left_numbers.length > self.right_numbers.length {
                self.right_numbers.push(-self.left_numbers.pop().unwrap());
            } else {
                self.left_numbers.push(-self.right_numbers.pop().unwrap());
            }
        }
    }

    fn find_median(&mut self) -> f64 {
        match self.left_numbers.length == self.right_numbers.length {
            true => {
                (self.left_numbers.peek().unwrap() + -self.right_numbers.peek().unwrap()) as f64
                    / 2.0
            }
            false => {
                if self.left_numbers.length > self.right_numbers.length {
                    return self.left_numbers.peek().unwrap() as f64;
                }
                -self.right_numbers.peek().unwrap() as f64
            }
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

fn main() {
    let mut mf = MedianFinder::new();

    /*
    mf.add_num(0);
    mf.add_num(0);
    mf.add_num(-DATA_SIZE);
    mf.add_num(-DATA_SIZE);
    mf.add_num(1);
    mf.add_num(-DATA_SIZE0);
    mf.add_num(180);
    mf.add_num(200);
    mf.add_num(400);
    mf.add_num(500);
    mf.add_num(0);
    */

    mf.add_num(-1);
    println!("{}", mf.find_median());
    mf.add_num(-2);
    println!("{}", mf.find_median());
    mf.add_num(-3);
    println!("{}", mf.find_median());
    mf.add_num(-4);
    println!("{}", mf.find_median());
    mf.add_num(-5);
    println!("{}", mf.find_median());
    mf.add_num(-6);
    println!("{}", mf.find_median());
}
