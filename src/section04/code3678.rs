#![allow(dead_code)]

// 3678. Smallest Absent Positive Greater Than Average
// https://leetcode.com/problems/smallest-absent-positive-greater-than-average/
// https://leetcode.cn/problems/smallest-absent-positive-greater-than-average/
//
// Easy
//
// You are given an integer array nums.
//
// Return the smallest absent positive integer in nums such that it is strictly greater than the average of all elements in nums.
//
// The average of an array is defined as the sum of all its elements divided by the number of elements.
//
// Example 1:
//
// Input: nums = [3,5]
//
// Output: 6
//
// Explanation:
//
// The average of nums is (3 + 5) / 2 = 8 / 2 = 4.
// The smallest absent positive integer greater than 4 is 6.
//
// Example 2:
//
// Input: nums = [-1,1,2]
//
// Output: 3
//
// Explanation:
//
// ​​​​​​​The average of nums is (-1 + 1 + 2) / 3 = 2 / 3 = 0.667.
// The smallest absent positive integer greater than 0.667 is 3.
//
// Example 3:
//
// Input: nums = [4,-1]
//
// Output: 2
//
// Explanation:
//
// The average of nums is (4 + (-1)) / 2 = 3 / 2 = 1.50.
// The smallest absent positive integer greater than 1.50 is 2.
//
// Constraints:
//
// 1 <= nums.length <= 100
// -100 <= nums[i] <= 100​​​​​​​
//

struct Solution;

impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        fn set_n(bm: &mut [u8; 26], n: i32) {
            // << (left shift) intends to increase the value: here => shift single '1' at position to set in the respective 'player/layer' (bm[i/8])
            let off = 100;
            let i = (off + n) as usize;

            if i > 200 {
                return;
            }

            bm[i / 8] |= 1 << (i % 8);
        }

        fn is_n_set(bm: &[u8; 26], n: i32) -> bool {
            let off = 100;
            let i = (off + n) as usize;

            if i > 200 {
                return false;
            }

            bm[i / 8] & (1 << (i % 8)) != 0
        }

        if nums.is_empty() {
            return 0;
        }

        let mut bm = [0u8; 26];

        let mut sum = 0;
        for &n in nums.iter() {
            set_n(&mut bm, n);
            sum += n;
        }
        let avg = sum / nums.len() as i32;

        for i in 1..=200 {
            let v = avg + i;
            if v > 0 && !is_n_set(&bm, v) {
                return v;
            }
        }

        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_absent(vec![3, 5]), 6);
    assert_eq!(Solution::smallest_absent(vec![-1, 1, 2]), 3);
    assert_eq!(Solution::smallest_absent(vec![4, -1]), 2);
}
