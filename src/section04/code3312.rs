#![allow(dead_code)]

// 3312. Sorted GCD Pair Queries
// https://leetcode.com/problems/sorted-gcd-pair-queries/
// https://leetcode.cn/problems/sorted-gcd-pair-queries/
//
// Hard
//
// You are given an integer array nums of length n and an integer array queries.
//
// Let gcdPairs denote an array obtained by calculating the GCD of all possible pairs (nums[i], nums[j]),
// where 0 <= i < j < n, and then sorting these values in ascending order.
//
// For each query queries[i], you need to find the element at index queries[i] in gcdPairs.
//
// Return an integer array answer, where answer[i] is the value at gcdPairs[queries[i]] for each query.
//
// The term gcd(a, b) denotes the greatest common divisor of a and b.
//
// Example 1:
//
// Input: nums = [2,3,4], queries = [0,2,2]
//
// Output: [1,2,2]
//
// Explanation:
//
// gcdPairs = [gcd(nums[0], nums[1]), gcd(nums[0], nums[2]), gcd(nums[1], nums[2])] = [1, 2, 1].
//
// After sorting in ascending order, gcdPairs = [1, 1, 2].
//
// So, the answer is [gcdPairs[queries[0]], gcdPairs[queries[1]], gcdPairs[queries[2]]] = [1, 2, 2].
//
// Example 2:
//
// Input: nums = [4,4,2,1], queries = [5,3,1,0]
//
// Output: [4,2,1,1]
//
// Explanation:
//
// gcdPairs sorted in ascending order is [1, 1, 1, 2, 2, 4].
//
// Example 3:
//
// Input: nums = [2,2], queries = [0,0]
//
// Output: [2,2]
//
// Explanation:
//
// gcdPairs = [2].
//
// Constraints:
//
//     2 <= n == nums.length <= 10^5
//     1 <= nums[i] <= 5 * 10^4
//     1 <= queries.length <= 10^5
//     0 <= queries[i] < n * (n - 1) / 2
//

struct Solution;

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        // let n = nums.len() as i32;
        let mx = *nums.iter().max().unwrap();
        let mut mp = vec![0; (mx + 1) as usize];
        for &it in nums.iter() {
            mp[it as usize] += 1;
        }
        let mut gcd_value: Vec<i64> = vec![0; (mx + 1) as usize];
        for i in (1..=mx).rev() {
            let mut c1 = 0;
            let mut sub = 0_i64;
            for j in (i..=mx).step_by(i as usize) {
                c1 += mp[j as usize];
                sub += gcd_value[j as usize];
            }
            gcd_value[i as usize] = c1 * (c1 - 1) / 2 - sub;
        }
        for i in 1..=mx {
            gcd_value[i as usize] += gcd_value[(i - 1) as usize];
        }
        let mut ans = vec![];
        for &q in queries.iter() {
            let mut lo = 1;
            let mut hi = mx;
            let mut val = -1;
            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                if gcd_value[mid as usize] >= (q + 1) {
                    val = mid;
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            }
            ans.push(val);
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::gcd_values(vec![2, 3, 4], vec![0, 2, 2]), vec![1, 2, 2]);
    assert_eq!(Solution::gcd_values(vec![4, 4, 2, 1], vec![5, 3, 1, 0]), vec![4, 2, 1, 1]);
    assert_eq!(Solution::gcd_values(vec![2, 2], vec![0, 0]), vec![2, 2]);
}
