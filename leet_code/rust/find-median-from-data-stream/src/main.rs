use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug)]
struct MedianFinder {
    numbers: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            //At most 5*10^4 calls?
            numbers: BinaryHeap::with_capacity(100000),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.numbers.push(num);
    }

    fn find_median(&mut self) -> f64 {
        let val = self.numbers.clone().into_sorted_vec();

        println!("{val:?}");
        if val.len() % 2 == 0 {
            (val[val.len() / 2 - 1] + val[val.len() / 2]) as f64 / 2.0
        } else {
            val[val.len() / 2] as f64
        }
        /*
        match self.left_numbers.len() == self.right_numbers.len() {
            true => {
                (*self.left_numbers.peek().unwrap() + self.right_numbers.peek().unwrap().0) as f64
                    / 2.0
            }
            false => {
                if self.left_numbers.len() > self.right_numbers.len() {
                    return *self.left_numbers.peek().unwrap() as f64;
                }
                self.right_numbers.peek().unwrap().0 as f64
            }
        }*/
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
    mf.add_num(-100000);
    mf.add_num(-100000);
    mf.add_num(1);
    mf.add_num(-100);
    mf.add_num(180);
    mf.add_num(200);
    mf.add_num(400);
    mf.add_num(500);
    mf.add_num(0);
    */

    mf.add_num(1);
    mf.add_num(2);
    mf.add_num(3);
    println!("{:?}", mf);
    println!("{}", mf.find_median());
}
