#![allow(dead_code)]

/*

// 1819. Number of Different Subsequences GCDs
// https://leetcode.com/problems/number-of-different-subsequences-gcds/
// https://leetcode.cn/problems/number-of-different-subsequences-gcds/
//
// Hard
//
// You are given an array nums that consists of positive integers.

The GCD of a sequence of numbers is defined as the greatest integer that divides all the numbers in the sequence evenly.

    For example, the GCD of the sequence [4,6,16] is 2.

A subsequence of an array is a sequence that can be formed by removing some elements (possibly none) of the array.

    For example, [2,5,10] is a subsequence of [1,2,1,2,4,1,5,10].

Return the number of different GCDs among all non-empty subsequences of nums.

Example 1:

Input: nums = [6,10,3]
Output: 5
Explanation: The figure shows all the non-empty subsequences and their GCDs.
The different GCDs are 6, 10, 3, 2, and 1.

Example 2:

Input: nums = [5,15,40,5,6]
Output: 7

Constraints:

    1 <= nums.length <= 10^5
    1 <= nums[i] <= 2 * 10^5
*/

struct Solution;

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let tmp = a;
                a = b;
                b = tmp % b;
            }
            a
        }

        let max_num = *nums.iter().max().unwrap();
        let mut vec = vec![false; max_num as usize + 1];
        for num in nums {
            vec[num as usize] = true;
        }

        let mut res = 0;
        for i in 1..=max_num {
            let mut _gcd = 0;
            for j in (i..=max_num).step_by(i as usize) {
                if vec[j as usize] {
                    _gcd = gcd(_gcd, j);
                }
            }
            if _gcd == i {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![6, 10, 3], 5),
        (vec![5, 15, 40, 5, 6], 7),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::count_different_subsequence_gc_ds(nums), expected);
    }
}
