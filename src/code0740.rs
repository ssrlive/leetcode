#![allow(dead_code)]

// 740. Delete and Earn
// https://leetcode.com/problems/delete-and-earn/
//
// You are given an integer array nums. You want to maximize the number of points you get by performing the following operation any number of times:
//
// - Pick any nums[i] and delete it to earn nums[i] points. Afterwards, you must delete every element equal to nums[i] - 1 and every element equal to nums[i] + 1.
//
// Return the maximum number of points you can earn by applying the above operation some number of times.
//
// Example 1:
//
// Input: nums = [3,4,2]
// Output: 6
// Explanation: You can perform the following operations:
// - Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
// - Delete 2 to earn 2 points. nums = [].
// You earn a total of 6 points.
//
// Example 2:
//
// Input: nums = [2,2,3,3,3,4]
// Output: 9
// Explanation: You can perform the following operations:
// - Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
// - Delete a 3 again to earn 3 points. nums = [3].
// - Delete a 3 once more to earn 3 points. nums = [].
// You earn a total of 9 points.
//
// Constraints:
//
// - 1 <= nums.length <= 2 * 10^4
// - 1 <= nums[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for v in nums {
            *map.entry(v).or_insert(0) += 1;
        }

        let mut arr = map.into_iter().collect::<Vec<(i32, i32)>>();
        arr.sort();
        let n = arr.len();

        let mut memo = vec![];
        let mut last = -1;
        for &item in arr.iter().take(n) {
            let (v, num) = item;
            if last + 1 != v {
                memo.push(vec![]);
            }
            let li = memo.len() - 1;
            memo[li].push(v * num);
            last = v;
        }

        let mut result = 0;
        for arr in memo {
            let n = arr.len();

            let mut dp = vec![(0, 0); n + 1];
            for i in 0..n {
                let v = arr[i];
                dp[i + 1].0 = dp[i + 1].0.max(dp[i].1 + v);
                dp[i + 1].1 = dp[i + 1].1.max(dp[i].0).max(dp[i].1);
            }
            result += dp[n].0.max(dp[n].1);
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
    assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
}
