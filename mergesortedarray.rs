use std::vec::Vec;

// Define a struct named Solution
struct Solution;

// Implement methods for the Solution struct
impl Solution {
    // Define a static method 'merge' that takes mutable references to two vectors and their lengths
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) {
        // Initialize indices for nums1, nums2, and the merged result
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = m + n - 1;

        // Iterate while there are elements in both nums1 and nums2
        while i >= 0 && j >= 0 {
            // Compare elements from the end and place the larger one in the merged result
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }

        // If there are remaining elements in nums2, copy them to the merged result
        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}

// Define the main function
fn main() {
    // Create a mutable vector nums1 with initial values
    let mut nums1: Vec<i32> = vec![1, 2, 7, 0, 0, 0];
    
    // Create an immutable vector nums2
    let nums2: Vec<i32> = vec![2, 5, 6];

    // Call the merge method on the Solution struct to merge nums1 and nums2
    Solution::merge(&mut nums1, 3, &nums2, 3);

    // Print the merged result
    println!("{:?}", nums1);
}
