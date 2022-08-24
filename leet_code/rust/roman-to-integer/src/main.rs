use std::{collections::HashMap, hash::Hash};

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
       let mut num = 0;
       let mut key : usize = 0;
       let number = s.as_bytes();
       let hashMap : HashMap<char, u16> = HashMap::from([('I', 1), ('V', 5), ('X', 10), ('L', 50), ('C',100), ('D',500), ('M',1000)]);
       while key < s.len() {
           if key > 0 && hashMap[&(number[key] as char)] > hashMap[&(number[key-1] as char)] {
              num += hashMap[&(number[key] as char)] - 2*hashMap[&(number[key-1] as char)];               
           } else {
              num += hashMap[&(number[key] as char)];
           }
           key+=1;
           /*
           num +=  match number[key] as char {
               'I' => 1 ,
               'V' => if key > 0 && number[key-1] as char == 'I' {
                        3
                    } else {
                        5
                    },
               'X' => if key > 0 && number[key-1] as char == 'I' {
                        8
                   } else {
                        10
                   },
               'L' => if key > 0 && number[key-1] as char == 'X' {
                        30
                    } else {
                        50
                    },
               'C' => if key > 0 && number[key-1] as char == 'X' {
                        80
                    } else {
                        100
                   },
               'D' => if key > 0 && number[key-1] as char == 'C' {
                        300
                    } else {
                        500
                    },
               'M' => if key > 0 && number[key-1] as char == 'C' {
                        800
                    } else {
                        1000
                    },
               _ => 0,
           };
           */
       }
       println!("{}", num);
       num as i32
    }
}

fn main() {
    assert_eq!(3, Solution::roman_to_int("III".to_string()));
    assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
    assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
}
