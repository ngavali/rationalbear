use std::collections::HashMap;

//At most 5*DATA_SIZE^4 calls?
const DATA_SIZE: usize = 100;

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

struct Storage {
    left: Heap,
    right: Heap,
}

struct Solution;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut result = Vec::with_capacity(nums.len() - k as usize + 1);
        let k = k as usize;

        let mut balance;

        let mut to_remove = HashMap::<i64, i64>::new();
        let mut storage = Storage {
            left: Heap::new(),
            right: Heap::new(),
        };
        let mut i = 0;

        //Populate the heap first
        while i < k {
            storage.left.push(nums[i] as i64);
            i += 1;
        }

        while i > (k + 1) / 2 {
            storage.right.push(-storage.left.pop().unwrap());
            i -= 1;
        }

        let odd = k % 2 != 0;

        result.push(match odd {
            true => storage.left.peek().unwrap() as f64,
            false => {
                (storage.left.peek().unwrap() as f64 + -storage.right.peek().unwrap() as f64) / 2.0
            }
        });

        let mut median;
        i = k;

        while i < nums.len() {
            median = result[i - k];
            to_remove
                .entry(nums[i - k] as i64)
                .and_modify(|v| *v += 1)
                .or_insert(1);

            //Old num was removed from left or right
            balance = match nums[i - k] as f64 <= median {
                true => -1,
                false => 1,
            };

            //New num was added to left or right
            if nums[i] <= median as i32 {
                storage.left.push(nums[i] as i64);
                balance += 1;
            } else {
                storage.right.push(-(nums[i] as i64));
                balance -= 1;
            }

            if balance > 0 {
                storage.right.push(-storage.left.pop().unwrap());
            } else if balance < 0 {
                storage.left.push(-storage.right.pop().unwrap());
            }

            //Pop if remove num on top
            while let Some(k) = storage.left.peek() {
                match to_remove.get_mut(&k) {
                    Some(v) => {
                        if *v > 0 {
                            *v -= 1;
                            storage.left.pop();
                        } else if *v == 0 {
                            to_remove.remove(&k);
                        }
                    }
                    _ => break,
                }
            }
            //Pop if remove num on top
            while let Some(k) = storage.right.peek() {
                match to_remove.get_mut(&(-k)) {
                    Some(v) => {
                        if *v > 0 {
                            storage.right.pop();
                            *v -= 1;
                        } else if *v == 0 {
                            to_remove.remove(&-k);
                        }
                    }
                    _ => break,
                }
            }

            result.push(match odd {
                true => storage.left.peek().unwrap() as f64,
                false => {
                    (storage.left.peek().unwrap() as f64 + -storage.right.peek().unwrap() as f64)
                        / 2.0
                }
            });
            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_median_sliding_window() {
        let testcases = vec![
            (
                vec![7, 9, 3, 8, 0, 2, 4, 8, 3, 9],
                1,
                vec![7.0, 9.0, 3.0, 8.0, 0.0, 2.0, 4.0, 8.0, 3.0, 9.0],
            ),
            (
                vec![1, 3, -1, -3, 5, 3, 6, 7],
                3,
                vec![1.0, -1.0, -1.0, 3.0, 5.0, 6.0],
            ),
            (
                vec![1, 2, 3, 4, 2, 3, 1, 4, 2],
                3,
                vec![2.0, 3.0, 3.0, 3.0, 2.0, 3.0, 2.0],
            ),
            (vec![1, 2], 1, vec![1.0, 2.0]),
            (vec![2147483647, 2147483647], 2, vec![2147483647.0]),
            (
                vec![7, 8, 8, 3, 8, 1, 5, 3, 5, 4],
                3,
                vec![8.0, 8.0, 8.0, 3.0, 5.0, 3.0, 5.0, 4.0],
            ),
        ];
        for (i, testcase) in testcases.iter().enumerate() {
            println!("Testcase: {:3}", i);
            assert_eq!(
                Solution::median_sliding_window(testcase.0.clone(), testcase.1),
                testcase.2
            );
        }
    }
}
