struct Solution {}

impl Solution {
    pub fn hammingWeight (mut n: u32) -> i32 {
       let mut i = 0;
       let mut ones : u32 = 0;
       while i < 32 {
            ones = ones + ( n & 1 );
            n>>=1;
            i+=1;
       } 
       ones as i32
    }
}

fn main() {
    println!("{}", Solution::hammingWeight(0b00000000000000000000000000001011));
}
