#![allow(dead_code)]

// 315. Count of Smaller Numbers After Self
// https://leetcode.com/problems/count-of-smaller-numbers-after-self/
//
// Given an integer array nums, return an integer array counts where counts[i] is the number of smaller elements to the right of nums[i].
//
// Example 1:
//
// Input: nums = [5,2,6,1]
// Output: [2,1,1,0]
// Explanation:
// To the right of 5 there are 2 smaller elements (2 and 1).
// To the right of 2 there is only 1 smaller element (1).
// To the right of 6 there is 1 smaller element (1).
// To the right of 1 there is 0 smaller element.
//
// Example 2:
//
// Input: nums = [-1]
// Output: [0]
//
// Example 3:
//
// Input: nums = [-1,-1]
// Output: [0,0]
//
// Constraints:
//
// - 1 <= nums.length <= 105
// - -104 <= nums[i] <= 104
//

struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let (n, m) = (nums.len(), 32768);
        let (mut a, mut ans) = ([0; 32768], vec![0; n]);
        for (i, &y) in nums.iter().enumerate().rev() {
            let (mut x, mut su) = (y as isize + 19999, 0);
            while x > 0 {
                su += a[x as usize];
                x -= -x & x;
            }
            let mut x = y as isize + 20000;
            ans[i] = su;
            while x < m {
                a[x as usize] += 1;
                x += -x & x;
            }
        }
        ans
    }
}

#[test]
fn test_count_smaller() {
    assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
    assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
}
