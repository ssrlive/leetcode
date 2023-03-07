#![allow(dead_code)]

/*

// 2367. Number of Arithmetic Triplets
// https://leetcode.com/problems/number-of-arithmetic-triplets/
// https://leetcode.cn/problems/number-of-arithmetic-triplets/
//
// Easy
//
// You are given a 0-indexed, strictly increasing integer array nums and a positive integer diff. A triplet (i, j, k) is an arithmetic triplet if the following conditions are met:

    i < j < k,
    nums[j] - nums[i] == diff, and
    nums[k] - nums[j] == diff.

Return the number of unique arithmetic triplets.

Example 1:

Input: nums = [0,1,4,6,7,10], diff = 3
Output: 2
Explanation:
(1, 2, 4) is an arithmetic triplet because both 7 - 4 == 3 and 4 - 1 == 3.
(2, 4, 5) is an arithmetic triplet because both 10 - 7 == 3 and 7 - 4 == 3.

Example 2:

Input: nums = [4,5,6,7,8,9], diff = 2
Output: 2
Explanation:
(0, 2, 4) is an arithmetic triplet because both 8 - 6 == 2 and 6 - 4 == 2.
(1, 3, 5) is an arithmetic triplet because both 9 - 7 == 2 and 7 - 5 == 2.

Constraints:

    3 <= nums.length <= 200
    0 <= nums[i] <= 200
    1 <= diff <= 50
    nums is strictly increasing.
*/

struct Solution;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        use std::cmp::Ordering;
        let mut count = 0;
        let len = nums.len();
        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                match (nums[j] - nums[i]).cmp(&diff) {
                    Ordering::Less => continue,
                    Ordering::Greater => break,
                    Ordering::Equal => {}
                }
                for k in j + 1..len {
                    match (nums[k] - nums[j]).cmp(&diff) {
                        Ordering::Less => continue,
                        Ordering::Greater => break,
                        Ordering::Equal => count += 1,
                    }
                }
            }
        }
        count
    }
}

#[test]
fn test() {
    let nums = vec![0, 1, 4, 6, 7, 10];
    let diff = 3;
    let count = Solution::arithmetic_triplets(nums, diff);
    assert_eq!(count, 2);

    let nums = vec![4, 5, 6, 7, 8, 9];
    let diff = 2;
    let count = Solution::arithmetic_triplets(nums, diff);
    assert_eq!(count, 2);
}
