#![allow(dead_code)]

// 3265. Count Almost Equal Pairs I
// https://leetcode.com/problems/count-almost-equal-pairs-i/
// https://leetcode.cn/problems/count-almost-equal-pairs-i/
//
// Medium
//
// You are given an array nums consisting of positive integers.
//
// We call two integers x and y in this problem almost equal if both integers
// can become equal after performing the following operation at most once:
//
//     Choose either x or y and swap any two digits within the chosen number.
//
// Return the number of indices i and j in nums where i < j such that nums[i] and nums[j] are almost equal.
//
// Note that it is allowed for an integer to have leading zeros after performing an operation.
//
// Example 1:
//
// Input: nums = [3,12,30,17,21]
//
// Output: 2
//
// Explanation:
//
// The almost equal pairs of elements are:
//
//     3 and 30. By swapping 3 and 0 in 30, you get 3.
//     12 and 21. By swapping 1 and 2 in 12, you get 21.
//
// Example 2:
//
// Input: nums = [1,1,1,1,1]
//
// Output: 10
//
// Explanation:
//
// Every two elements in the array are almost equal.
//
// Example 3:
//
// Input: nums = [123,231]
//
// Output: 0
//
// Explanation:
//
// We cannot swap any two digits of 123 or 231 to reach the other.
//
// Constraints:
//
//     2 <= nums.length <= 100
//     1 <= nums[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut freq = std::collections::BTreeMap::new();
        for n in nums {
            *freq.entry(n).or_insert(0) += 1;
            ans += freq[&n] - 1;
            let mut s = format!("{:07}", n).chars().collect::<Vec<_>>();
            for i in 0..s.len() {
                for j in i + 1..s.len() {
                    s.swap(i, j);
                    let temp = s.iter().collect::<String>().parse::<i32>().unwrap();
                    if temp != n {
                        ans += freq.get(&temp).unwrap_or(&0);
                    }
                    s.swap(i, j);
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_pairs(vec![3, 12, 30, 17, 21]), 2);
    assert_eq!(Solution::count_pairs(vec![1, 1, 1, 1, 1]), 10);
    assert_eq!(Solution::count_pairs(vec![123, 231]), 0);
}
