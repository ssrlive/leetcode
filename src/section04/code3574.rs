#![allow(dead_code)]

// 3574. Maximize Subarray GCD Score
// https://leetcode.com/problems/maximize-subarray-gcd-score/
// https://leetcode.cn/problems/maximize-subarray-gcd-score/
//
// Hard
//
// You are given an array of positive integers nums and an integer k.
//
// You may perform at most k operations. In each operation, you can choose one element in the array
// and double its value. Each element can be doubled at most once.
//
// The score of a contiguous is defined as the product of its length and the greatest common divisor (GCD) of all its elements.
//
// Your task is to return the maximum score that can be achieved by selecting a contiguous subarray from the modified array.
//
// Note:
//
//     The greatest common divisor (GCD) of an array is the largest integer that evenly divides all the array elements.
//
// Example 1:
//
// Input: nums = [2,4], k = 1
//
// Output: 8
//
// Explanation:
//
//     Double nums[0] to 4 using one operation. The modified array becomes [4, 4].
//     The GCD of the subarray [4, 4] is 4, and the length is 2.
//     Thus, the maximum possible score is 2 × 4 = 8.
//
// Example 2:
//
// Input: nums = [3,5,7], k = 2
//
// Output: 14
//
// Explanation:
//
//     Double nums[2] to 14 using one operation. The modified array becomes [3, 5, 14].
//     The GCD of the subarray [14] is 14, and the length is 1.
//     Thus, the maximum possible score is 1 × 14 = 14.
//
// Example 3:
//
// Input: nums = [5,5,5], k = 1
//
// Output: 15
//
// Explanation:
//
//     The subarray [5, 5, 5] has a GCD of 5, and its length is 3.
//     Since doubling any element doesn't improve the score, the maximum score is 3 × 5 = 15.
//
// Constraints:
//
//     1 <= n == nums.length <= 1500
//     1 <= nums[i] <= 10^9
//     1 <= k <= n
//

struct Solution;

impl Solution {
    pub fn max_gcd_score(nums: Vec<i32>, k: i32) -> i64 {
        let nums: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let k = k as i64;

        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 { a } else { gcd(b, a % b) }
        }

        let n = nums.len();
        let mut power_of_2 = vec![0; n];
        for i in 0..n {
            let mut x = nums[i];
            while x % 2 == 0 {
                power_of_2[i] += 1;
                x /= 2;
            }
        }
        let mut ans = 0;
        for l in 0..n {
            let mut gcd_val = 0;
            let mut min_power = 100;
            let mut min_power_cnt = 0;
            for r in l..n {
                gcd_val = gcd(gcd_val, nums[r]);
                if power_of_2[r] < min_power {
                    min_power = power_of_2[r];
                    min_power_cnt = 1;
                } else if power_of_2[r] == min_power {
                    min_power_cnt += 1;
                }
                if k >= min_power_cnt {
                    ans = ans.max((r - l + 1) as i64 * gcd_val * 2);
                } else {
                    ans = ans.max((r - l + 1) as i64 * gcd_val);
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_gcd_score(vec![2, 4], 1), 8);
    assert_eq!(Solution::max_gcd_score(vec![3, 5, 7], 2), 14);
    assert_eq!(Solution::max_gcd_score(vec![5, 5, 5], 1), 15);
}
