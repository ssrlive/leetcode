#![allow(dead_code)]

// 3424. Minimum Cost to Make Arrays Identical
// https://leetcode.com/problems/minimum-cost-to-make-arrays-identical/
// https://leetcode.cn/problems/minimum-cost-to-make-arrays-identical/
//
// Medium
//
// You are given two integer arrays arr and brr of length n, and an integer k.
// You can perform the following operations on arr any number of times:
//
// - Split arr into any number of contiguous subarrays and rearrange these subarrays
//   in any order. This operation has a fixed cost of k.
// - Choose any element in arr and add or subtract a positive integer x to it. The cost of this operation is x.
//
// Return the minimum total cost to make arr equal to brr.
//
// Example 1:
//
// Input: arr = [-7,9,5], brr = [7,-2,-5], k = 2
//
// Output: 13
//
// Explanation:
//
// - Split arr into two contiguous subarrays: [-7] and [9, 5] and rearrange them as [9, 5, -7], with a cost of 2.
// - Subtract 2 from element arr[0]. The array becomes [7, 5, -7]. The cost of this operation is 2.
// - Subtract 7 from element arr[1]. The array becomes [7, -2, -7]. The cost of this operation is 7.
// - Add 2 to element arr[2]. The array becomes [7, -2, -5]. The cost of this operation is 2.
//
// The total cost to make the arrays equal is 2 + 2 + 7 + 2 = 13.
//
// Example 2:
//
// Input: arr = [2,1], brr = [2,1], k = 0
//
// Output: 0
//
// Explanation:
//
// Since the arrays are already equal, no operations are needed, and the total cost is 0.
//
// Constraints:
//
//     1 <= arr.length == brr.length <= 10^5
//     0 <= k <= 2 * 10^10
//     -10^5 <= arr[i] <= 10^5
//     -10^5 <= brr[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn min_cost(arr: Vec<i32>, brr: Vec<i32>, k: i64) -> i64 {
        fn cal(arr: &[i32], brr: &[i32]) -> i64 {
            arr.iter().zip(brr.iter()).map(|(a, b)| (a - b).abs() as i64).sum()
        }

        let mut arr = arr;
        let mut brr = brr;
        let mut ans = cal(&arr, &brr);

        arr.sort_unstable();
        brr.sort_unstable();

        ans = ans.min(cal(&arr, &brr) + k);
        ans
    }
}

#[test]
fn test() {
    let arr = vec![-7, 9, 5];
    let brr = vec![7, -2, -5];
    let k = 2;
    let res = 13;
    assert_eq!(Solution::min_cost(arr, brr, k), res);

    let arr = vec![2, 1];
    let brr = vec![2, 1];
    let k = 0;
    let res = 0;
    assert_eq!(Solution::min_cost(arr, brr, k), res);
}
