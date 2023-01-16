#![allow(dead_code)]

// 2442. Count Number of Distinct Integers After Reverse Operations
// https://leetcode.com/problems/count-number-of-distinct-integers-after-reverse-operations/
// https://leetcode.cn/problems/count-number-of-distinct-integers-after-reverse-operations/
//
// You are given an array nums consisting of positive integers.
//
// You have to take each integer in the array, reverse its digits, and add it to the end of the array.
// You should apply this operation to the original integers in nums.
//
// Return the number of distinct integers in the final array.
//
// Example 1:
//
// Input: nums = [1,13,10,12,31]
// Output: 6
// Explanation: After including the reverse of each number, the resulting array is [1,13,10,12,31,1,31,1,21,13].
// The reversed integers that were added to the end of the array are underlined. Note that for the integer 10,
// after reversing it, it becomes 01 which is just 1.
// The number of distinct integers in this array is 6 (The numbers 1, 10, 12, 13, 21, and 31).
//
// Example 2:
//
// Input: nums = [2,2,2]
// Output: 1
// Explanation: After including the reverse of each number, the resulting array is [2,2,2,2,2,2].
// The number of distinct integers in this array is 1 (The number 2).
//
// Constraints:
//
// - 1 <= nums.length <= 10^5
// - 1 <= nums[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        pub const fn reverse(mut num: i32) -> i32 {
            let mut res: i32 = 0;
            while num != 0 {
                res *= 10;
                res += num % 10;
                num /= 10;
            }
            res
        }

        let mut res = Vec::with_capacity(nums.len() * 2);
        res.extend(&nums);
        for num in nums {
            res.push(reverse(num));
        }
        res.sort_unstable();
        res.dedup();
        res.len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_distinct_integers(vec![1, 13, 10, 12, 31]), 6);
    assert_eq!(Solution::count_distinct_integers(vec![2, 2, 2]), 1);
}
