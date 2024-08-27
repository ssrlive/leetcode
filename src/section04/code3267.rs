#![allow(dead_code)]

// 3267. Count Almost Equal Pairs II
// https://leetcode.com/problems/count-almost-equal-pairs-ii/
// https://leetcode.cn/problems/count-almost-equal-pairs-ii/
//
// Hard
//
// Attention: In this version, the number of operations that can be performed, has been increased to twice.
//
// You are given an array nums consisting of positive integers.
//
// We call two integers x and y almost equal if both integers can become equal after performing the following operation at most twice:
//
//     Choose either x or y and swap any two digits within the chosen number.
//
// Return the number of indices i and j in nums where i < j such that nums[i] and nums[j] are almost equal.
//
// Note that it is allowed for an integer to have leading zeros after performing an operation.
//
// Example 1:
//
// Input: nums = [1023,2310,2130,213]
//
// Output: 4
//
// Explanation:
//
// The almost equal pairs of elements are:
//
//     1023 and 2310. By swapping the digits 1 and 2, and then the digits 0 and 3 in 1023, you get 2310.
//     1023 and 213. By swapping the digits 1 and 0, and then the digits 1 and 2 in 1023, you get 0213, which is 213.
//     2310 and 213. By swapping the digits 2 and 0, and then the digits 3 and 2 in 2310, you get 0213, which is 213.
//     2310 and 2130. By swapping the digits 3 and 1 in 2310, you get 2130.
//
// Example 2:
//
// Input: nums = [1,10,100]
//
// Output: 3
//
// Explanation:
//
// The almost equal pairs of elements are:
//
//     1 and 10. By swapping the digits 1 and 0 in 10, you get 01 which is 1.
//     1 and 100. By swapping the second 0 with the digit 1 in 100, you get 001, which is 1.
//     10 and 100. By swapping the first 0 with the digit 1 in 100, you get 010, which is 10.
//
// Constraints:
//
//     2 <= nums.length <= 5000
//     1 <= nums[i] < 10^7
//

struct Solution;

impl Solution {
    pub fn count_pairs(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut ans = 0;
        let mut p = std::collections::HashMap::new();

        for &x in &nums {
            let mut a = vec![];
            let mut f10 = 1;
            let mut x1 = x;

            while x1 > 0 {
                a.push((x1 % 10, f10));
                x1 /= 10;
                f10 *= 10;
            }

            let mut h = std::collections::HashSet::new();
            h.insert(x);

            for i in 1..a.len() {
                for j in 0..i {
                    let x = x - a[i].0 * a[i].1 - a[j].0 * a[j].1 + a[i].0 * a[j].1 + a[j].0 * a[i].1;
                    h.insert(x);

                    let tmp = a[i].0;
                    a[i].0 = a[j].0;
                    a[j].0 = tmp;

                    for k in 1..a.len() {
                        for l in 0..k {
                            h.insert(x - a[k].0 * a[k].1 - a[l].0 * a[l].1 + a[k].0 * a[l].1 + a[l].0 * a[k].1);
                        }
                    }

                    let tmp = a[i].0;
                    a[i].0 = a[j].0;
                    a[j].0 = tmp;
                }
            }

            for x in &h {
                if let Some(&c) = p.get(x) {
                    ans += c;
                }
            }

            *p.entry(x).or_insert(0) += 1;
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_pairs(vec![1023, 2310, 2130, 213]), 4);
    assert_eq!(Solution::count_pairs(vec![1, 10, 100]), 3);
    assert_eq!(Solution::count_pairs(vec![1, 10, 100, 1000]), 6);
}
