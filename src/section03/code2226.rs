#![allow(dead_code)]

/*

// 2226. Maximum Candies Allocated to K Children
// https://leetcode.com/problems/maximum-candies-allocated-to-k-children/
// https://leetcode.cn/problems/maximum-candies-allocated-to-k-children/
//
// Medium
//
// You are given a 0-indexed integer array candies. Each element in the array denotes a pile of candies of size candies[i]. You can divide each pile into any number of sub piles, but you cannot merge two piles together.

You are also given an integer k. You should allocate piles of candies to k children such that each child gets the same number of candies. Each child can take at most one pile of candies and some piles of candies may go unused.

Return the maximum number of candies each child can get.

Example 1:

Input: candies = [5,8,6], k = 3
Output: 5
Explanation: We can divide candies[1] into 2 piles of size 5 and 3, and candies[2] into 2 piles of size 5 and 1. We now have five piles of candies of sizes 5, 5, 3, 5, and 1. We can allocate the 3 piles of size 5 to 3 children. It can be proven that each child cannot receive more than 5 candies.

Example 2:

Input: candies = [2,5], k = 11
Output: 0
Explanation: There are 11 children but only 7 candies in total, so it is impossible to ensure each child receives at least one candy. Thus, each child gets no candy and the answer is 0.

Constraints:

    1 <= candies.length <= 10^5
    1 <= candies[i] <= 10^7
    1 <= k <= 10^12
*/

struct Solution;

impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut candies = candies;
        candies.sort_unstable_by(|a, b| b.cmp(a));
        let max_possible = candies.iter().map(|&i| i as i64).sum::<i64>() / k;
        if max_possible <= 1 {
            max_possible as i32
        } else {
            let mut end = (max_possible + 1) as i32;
            let mut start = 1_i32;
            while end - start > 1 {
                let try_give_candies = (end + start) / 2;
                let available_piles = candies
                    .get(0..candies.partition_point(|c| c >= &try_give_candies))
                    .unwrap()
                    .iter()
                    .fold(0_i64, |accm, c| accm + (c / try_give_candies) as i64);
                if available_piles < k {
                    end = try_give_candies;
                } else {
                    start = try_give_candies;
                }
            }
            start
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![5, 8, 6], 3, 5),
        (vec![2, 5], 11, 0),
        (vec![2, 5], 10, 0),
        (vec![2, 5], 9, 0),
    ];
    for (candies, k, expected) in cases {
        assert_eq!(Solution::maximum_candies(candies, k), expected);
    }
}
