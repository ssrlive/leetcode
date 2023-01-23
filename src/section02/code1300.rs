#![allow(dead_code)]

// 1300. Sum of Mutated Array Closest to Target
// https://leetcode.com/problems/sum-of-mutated-array-closest-to-target/
// https://leetcode.cn/problems/sum-of-mutated-array-closest-to-target/
//
// Medium
//
// Given an integer array arr and a target value target, return the integer value such that when we change
// all the integers larger than value in the given array to be equal to value, the sum of the array gets
// as close as possible (in absolute difference) to target.
//
// In case of a tie, return the minimum such integer.
//
// Notice that the answer is not neccesarilly a number from arr.
//
// Example 1:
//
// Input: arr = [4,9,3], target = 10
// Output: 3
// Explanation: When using 3 arr converts to [3, 3, 3] which sums 9 and that's the optimal answer.
//
// Example 2:
//
// Input: arr = [2,3,5], target = 10
// Output: 5
//
// Example 3:
//
// Input: arr = [60864,25176,27249,21296,20204], target = 56803
// Output: 11361
//
// Constraints:
//
// -    1 <= arr.length <= 10^4
// -    1 <= arr[i], target <= 10^5
//

struct Solution;

impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr = arr;
        arr.sort();
        let n = arr.len();
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + arr[i];
        }
        let mut ans = 0;
        let mut diff = target;
        for i in 0..=arr[n - 1] {
            let index = match arr.binary_search(&i) {
                Ok(index) => index,
                Err(index) => index,
            };
            let cur = prefix[index] + (n - index) as i32 * i;
            if (cur - target).abs() < diff {
                ans = i;
                diff = (cur - target).abs();
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![4, 9, 3], 10, 3),
        (vec![2, 3, 5], 10, 5),
        (vec![60864, 25176, 27249, 21296, 20204], 56803, 11361),
    ];
    for (arr, target, expect) in cases {
        assert_eq!(Solution::find_best_value(arr, target), expect);
    }
}
