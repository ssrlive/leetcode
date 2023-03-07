#![allow(dead_code)]

/*

// 2354. Number of Excellent Pairs
// https://leetcode.com/problems/number-of-excellent-pairs/
// https://leetcode.cn/problems/number-of-excellent-pairs/
//
// Hard
//
// You are given a 0-indexed positive integer array nums and a positive integer k.

A pair of numbers (num1, num2) is called excellent if the following conditions are satisfied:

    Both the numbers num1 and num2 exist in the array nums.
    The sum of the number of set bits in num1 OR num2 and num1 AND num2 is greater than or equal to k, where OR is the bitwise OR operation and AND is the bitwise AND operation.

Return the number of distinct excellent pairs.

Two pairs (a, b) and (c, d) are considered distinct if either a != c or b != d. For example, (1, 2) and (2, 1) are distinct.

Note that a pair (num1, num2) such that num1 == num2 can also be excellent if you have at least one occurrence of num1 in the array.

Example 1:

Input: nums = [1,2,3,1], k = 3
Output: 5
Explanation: The excellent pairs are the following:
- (3, 3). (3 AND 3) and (3 OR 3) are both equal to (11) in binary. The total number of set bits is 2 + 2 = 4, which is greater than or equal to k = 3.
- (2, 3) and (3, 2). (2 AND 3) is equal to (10) in binary, and (2 OR 3) is equal to (11) in binary. The total number of set bits is 1 + 2 = 3.
- (1, 3) and (3, 1). (1 AND 3) is equal to (01) in binary, and (1 OR 3) is equal to (11) in binary. The total number of set bits is 1 + 2 = 3.
So the number of excellent pairs is 5.

Example 2:

Input: nums = [5,1,1], k = 10
Output: 0
Explanation: There are no excellent pairs for this array.

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 10^9
    1 <= k <= 60
*/

struct Solution;

impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashSet;
        fn binary_search(v: &Vec<i32>, a: i32) -> i64 {
            let (mut left, mut right) = (0, v.len() - 1);
            if a > v[v.len() - 1] {
                return 0i64;
            }
            while left < right {
                let mid = left + (right - left) / 2;
                if v[mid] < a {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            (v.len() - left) as i64
        }

        let mut s: HashSet<i32> = HashSet::new();
        let mut v = vec![];
        for a in nums {
            if s.contains(&a) {
                continue;
            }
            s.insert(a);
            let mut cnt = 0;
            for i in 0..32 {
                if (a & (1 << i)) > 0 {
                    cnt += 1;
                }
            }
            v.push(cnt);
        }
        v.sort();
        let mut ret: i64 = 0;
        for a in &v {
            ret += binary_search(&v, k - *a);
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_excellent_pairs(vec![1, 2, 3, 1], 3), 5);
    assert_eq!(Solution::count_excellent_pairs(vec![5, 1, 1], 10), 0);
}
