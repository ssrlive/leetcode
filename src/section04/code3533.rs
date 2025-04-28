#![allow(dead_code)]

// 3533. Concatenated Divisibility
// https://leetcode.com/problems/concatenated-divisibility/
// https://leetcode.cn/problems/concatenated-divisibility/
//
// Hard
//
// You are given an array of positive integers nums and a positive integer k.
// A of nums is said to form a divisible concatenation if, when you concatenate the decimal representations
// of the numbers in the order specified by the permutation, the resulting number is divisible by k.
//
// Return the permutation (when considered as a list of integers) that forms a divisible concatenation.
// If no such permutation exists, return an empty list.
//
// Example 1:
//
// Input: nums = [3,12,45], k = 5
//
// Output: [3,12,45]
//
// Explanation:
// Permutation	Concatenated Value	Divisible by 5
// [3, 12, 45]	31245	Yes
// [3, 45, 12]	34512	No
// [12, 3, 45]	12345	Yes
// [12, 45, 3]	12453	No
// [45, 3, 12]	45312	No
// [45, 12, 3]	45123	No
//
// The lexicographically smallest permutation that forms a divisible concatenation is [3,12,45].
//
// Example 2:
//
// Input: nums = [10,5], k = 10
//
// Output: [5,10]
//
// Explanation:
// Permutation	Concatenated Value	Divisible by 10
// [5, 10]	510	Yes
// [10, 5]	105	No
//
// The lexicographically smallest permutation that forms a divisible concatenation is [5,10].
//
// Example 3:
//
// Input: nums = [1,2,3], k = 5
//
// Output: []
//
// Explanation:
//
// Since no permutation of nums forms a valid divisible concatenation, return an empty list.
//
// Constraints:
//
//     1 <= nums.length <= 13
//     1 <= nums[i] <= 10^5
//     1 <= k <= 100
//

struct Solution;

impl Solution {
    pub fn concatenated_divisibility(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums.iter().map(|&x| x as i64).collect::<Vec<_>>();
        nums.sort();
        let k = k as i64;
        let n = nums.len();
        let mut dp = vec![vec![-1; 1 << n]; k as usize];
        let mut path = vec![];
        let mut ans = vec![];
        let mut find = false;
        #[allow(clippy::too_many_arguments)]
        fn rec(find: &mut bool, path: &mut Vec<i64>, mask: i64, k: i64, arr: &[i64], rem: i64, dp: &mut Vec<Vec<i64>>, ans: &mut Vec<i64>) {
            if *find {
                return;
            }
            if mask == (1 << arr.len()) - 1 {
                if rem == 0 {
                    // valid permutation
                    if ans.is_empty() {
                        for e in path {
                            ans.push(*e);
                        }
                    }
                    *find = true;
                    return;
                } else {
                    return;
                }
            }
            if dp[rem as usize][mask as usize] != -1 {
                return;
            }
            for i in 0..arr.len() {
                if mask & (1 << i) != 0 {
                    continue;
                }
                let no = arr[i];
                let len = no.to_string().len() as i64;
                let zero = 10_i64.pow(len as u32);
                path.push(no);
                rec(find, path, mask | (1 << i), k, arr, (rem * zero + no) % k, dp, ans);
                path.pop();
            }
            dp[rem as usize][mask as usize] = 1;
        }
        rec(&mut find, &mut path, 0, k, &nums, 0, &mut dp, &mut ans);
        ans.iter().map(|&x| x as i32).collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::concatenated_divisibility(vec![3, 12, 45], 5), vec![3, 12, 45]);
    assert_eq!(Solution::concatenated_divisibility(vec![10, 5], 10), vec![5, 10]);
    assert_eq!(Solution::concatenated_divisibility(vec![1, 2, 3], 5), vec![]);
}
