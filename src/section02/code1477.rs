#![allow(dead_code)]

/*

// 1477. Find Two Non-overlapping Sub-arrays Each With Target Sum
// https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum/
// https://leetcode.cn/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum/
//
// Medium
//
// You are given an array of integers arr and an integer target.

You have to find two non-overlapping sub-arrays of arr each with a sum equal target. There can be multiple answers so you have to find an answer where the sum of the lengths of the two sub-arrays is minimum.

Return the minimum sum of the lengths of the two required sub-arrays, or return -1 if you cannot find such two sub-arrays.

Example 1:

Input: arr = [3,2,2,4,3], target = 3
Output: 2
Explanation: Only two sub-arrays have sum = 3 ([3] and [3]). The sum of their lengths is 2.

Example 2:

Input: arr = [7,3,4,7], target = 7
Output: 2
Explanation: Although we have three non-overlapping sub-arrays of sum = 7 ([7], [3,4] and [7]), but we will choose the first and third sub-arrays as the sum of their lengths is 2.

Example 3:

Input: arr = [4,3,2,6,2,3,4], target = 6
Output: -1
Explanation: We have only one sub-array of sum = 6.

Constraints:

    1 <= arr.length <= 10^5
    1 <= arr[i] <= 1000
    1 <= target <= 10^8
*/

struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        use std::collections::HashMap;
        let (mut sum, mut res, mut lsize) = (0, i32::MAX, i32::MAX);
        let mut prefix_sum: HashMap<i32, i32> = HashMap::new();

        prefix_sum.insert(0, -1);

        for (i, val) in arr.iter().enumerate() {
            sum += *val;
            prefix_sum.insert(sum, i as i32);
        }

        sum = 0;
        for (i, val) in arr.iter().enumerate() {
            sum += *val;
            let index = i as i32;

            if prefix_sum.contains_key(&(sum - target)) {
                lsize = lsize.min(index - *prefix_sum.get(&(sum - target)).unwrap());
            }

            if prefix_sum.contains_key(&(sum + target)) && lsize != i32::MAX {
                let rsize = *prefix_sum.get(&(sum + target)).unwrap() - index;
                res = res.min(rsize + lsize);
            }
        }

        if res == i32::MAX {
            -1
        } else {
            res
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![3, 2, 2, 4, 3], 3, 2),
        (vec![7, 3, 4, 7], 7, 2),
        (vec![4, 3, 2, 6, 2, 3, 4], 6, -1),
        (vec![5, 5, 4, 4, 5], 3, -1),
        (vec![3, 1, 1, 1, 5, 1, 2, 1], 3, 3),
        (vec![84, -37, 32, 40, 95], 167, -1),
    ];
    for (arr, target, expected) in cases {
        assert_eq!(Solution::min_sum_of_lengths(arr, target), expected);
    }
}
