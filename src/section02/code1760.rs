#![allow(dead_code)]

/*

// 1760. Minimum Limit of Balls in a Bag
// https://leetcode.com/problems/minimum-limit-of-balls-in-a-bag/
// https://leetcode.cn/problems/minimum-limit-of-balls-in-a-bag/
//
// Medium
//
// You are given an integer array nums where the ith bag contains nums[i] balls. You are also given an integer maxOperations.

You can perform the following operation at most maxOperations times:

    Take any bag of balls and divide it into two new bags with a positive number of balls.
        For example, a bag of 5 balls can become two new bags of 1 and 4 balls, or two new bags of 2 and 3 balls.

Your penalty is the maximum number of balls in a bag. You want to minimize your penalty after the operations.

Return the minimum possible penalty after performing the operations.

Example 1:

Input: nums = [9], maxOperations = 2
Output: 3
Explanation:
- Divide the bag with 9 balls into two bags of sizes 6 and 3. [9] -> [6,3].
- Divide the bag with 6 balls into two bags of sizes 3 and 3. [6,3] -> [3,3,3].
The bag with the most number of balls has 3 balls, so your penalty is 3 and you should return 3.

Example 2:

Input: nums = [2,4,8,2], maxOperations = 4
Output: 2
Explanation:
- Divide the bag with 8 balls into two bags of sizes 4 and 4. [2,4,8,2] -> [2,4,4,4,2].
- Divide the bag with 4 balls into two bags of sizes 2 and 2. [2,4,4,4,2] -> [2,2,2,4,4,2].
- Divide the bag with 4 balls into two bags of sizes 2 and 2. [2,2,2,4,4,2] -> [2,2,2,2,2,4,2].
- Divide the bag with 4 balls into two bags of sizes 2 and 2. [2,2,2,2,2,4,2] -> [2,2,2,2,2,2,2,2].
The bag with the most number of balls has 2 balls, so your penalty is 2, and you should return 2.

Constraints:

    1 <= nums.length <= 10^5
    1 <= maxOperations, nums[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut l = 1;
        let mut r = *nums.iter().max().unwrap();
        while l < r {
            let m = (l + r) / 2;
            let mut cnt = 0;
            for &n in nums.iter() {
                cnt += (n - 1) / m;
            }
            if cnt <= max_operations {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![9], 2, 3),
        (vec![2, 4, 8, 2], 4, 2),
        (vec![7, 17], 2, 7),
    ];
    for (nums, max_operations, expected) in cases {
        assert_eq!(Solution::minimum_size(nums, max_operations), expected);
    }
}
