#![allow(dead_code)]

// 414. Third Maximum Number
// https://leetcode.com/problems/third-maximum-number/
//
// Given integer array nums, return the third maximum number in this array. If the
// third maximum does not exist, return the maximum number.
//
// Example 1:
//
// Input: nums = [3,2,1]
// Output: 1
// Explanation: The third maximum is 1.
//
// Example 2:
//
// Input: nums = [1,2]
// Output: 2
// Explanation: The third maximum does not exist, so the maximum (2) is returned instead.
//
// Example 3:
//
// Input: nums = [2,2,3,1]
// Output: 1
// Explanation: Note that the third maximum here means the third maximum distinct number.
// Both numbers with value 2 are both considered as second maximum.
//
// Constraints:
//
// - 1 <= nums.length <= 104
// - -231 <= nums[i] <= 231 - 1
//
// Follow up: Can you find an O(n) solution?

struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        use std::cmp::Ordering;
        fn compare(v: i32, max: Option<i32>) -> Ordering {
            match max {
                Some(max) => v.cmp(&max),
                None => Ordering::Greater,
            }
        }

        fn _third_max(nums: Vec<i32>) -> Option<i32> {
            let mut i = nums.into_iter();
            let mut max1 = i.next()?;
            let mut max2 = None;
            let mut max3 = None;

            for n in i {
                match n.cmp(&max1) {
                    Ordering::Greater => {
                        max3 = max2;
                        max2 = Some(max1);
                        max1 = n;
                    }
                    Ordering::Equal => (),
                    Ordering::Less => match compare(n, max2) {
                        Ordering::Greater => {
                            max3 = max2;
                            max2 = Some(n);
                        }
                        Ordering::Equal => (),
                        Ordering::Less => {
                            if compare(n, max3) == Ordering::Greater {
                                max3 = Some(n);
                            }
                        }
                    },
                }
            }

            Some(max3.unwrap_or(max1))
        }

        _third_max(nums).unwrap_or_default()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
    assert_eq!(Solution::third_max(vec![1, 2]), 2);
    assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    assert_eq!(Solution::third_max(vec![1, 2, -2147483648]), -2147483648);
}
