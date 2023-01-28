#![allow(dead_code)]

// 1399. Count Largest Group
// https://leetcode.com/problems/count-largest-group/
// https://leetcode.cn/problems/count-largest-group/
//
// Easy
//
// You are given an integer n.
//
// Each number from 1 to n is grouped according to the sum of its digits.
//
// Return the number of groups that have the largest size.
//
// Example 1:
//
// Input: n = 13
// Output: 4
// Explanation: There are 9 groups in total, they are grouped according sum of its digits of numbers from 1 to 13:
// [1,10], [2,11], [3,12], [4,13], [5], [6], [7], [8], [9].
// There are 4 groups with largest size.
//
// Example 2:
//
// Input: n = 2
// Output: 2
// Explanation: There are 2 groups [1], [2] of size 1.
//
// Constraints:
//
// -    1 <= n <= 10^4
//

struct Solution;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut count = vec![0; 37];
        for i in 1..=n {
            let mut sum = 0;
            let mut num = i;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            count[sum as usize] += 1;
        }
        let mut max = 0;
        let mut ans = 0;
        for &item in count.iter().take(37) {
            match item.cmp(&max) {
                std::cmp::Ordering::Greater => {
                    max = item;
                    ans = 1;
                }
                std::cmp::Ordering::Equal => ans += 1,
                _ => (),
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![(13, 4), (2, 2)];
    for (n, expected) in cases {
        assert_eq!(Solution::count_largest_group(n), expected);
    }
}
