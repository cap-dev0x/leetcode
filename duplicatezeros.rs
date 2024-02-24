use std::vec::Vec;

struct Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i = 0;
        while i < arr.len() {
            if arr[i] == 0 {
                arr.insert(i+1,0);
                arr.pop();
                i = i + 2;
            }
            else {
                i = i + 1;
            }
        }
    }
}

fn main() {
    println!("Hello, World!");
}