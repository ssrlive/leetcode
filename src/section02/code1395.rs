#![allow(dead_code)]

// 1395. Count Number of Teams
// https://leetcode.com/problems/count-number-of-teams/
// https://leetcode.cn/problems/count-number-of-teams/
//
// Medium
//
// There are n soldiers standing in a line. Each soldier is assigned a unique rating value.
//
// You have to form a team of 3 soldiers amongst them under the following rules:
//
//     Choose 3 soldiers with index (i, j, k) with rating (rating[i], rating[j], rating[k]).
//     A team is valid if: (rating[i] < rating[j] < rating[k]) or (rating[i] > rating[j] > rating[k]) where (0 <= i < j < k < n).
//
// Return the number of teams you can form given the conditions. (soldiers can be part of multiple teams).
//
// Example 1:
//
// Input: rating = [2,5,3,4,1]
// Output: 3
// Explanation: We can form three teams given the conditions. (2,3,4), (5,4,1), (5,3,1).
//
// Example 2:
//
// Input: rating = [2,1,3]
// Output: 0
// Explanation: We can't form any team given the conditions.
//
// Example 3:
//
// Input: rating = [1,2,3,4]
// Output: 4
//
// Constraints:
//
// -    n == rating.length
// -    3 <= n <= 1000
// -    1 <= rating[i] <= 10^5
// -    All the integers in rating are unique.
//

struct Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 1..rating.len() {
            let mut rightless = 0;
            let mut rightmore = 0;
            let mut leftless = 0;
            let mut leftmore = 0;
            for j in 0..i {
                match rating[j].cmp(&rating[i]) {
                    std::cmp::Ordering::Greater => leftmore += 1,
                    std::cmp::Ordering::Less => leftless += 1,
                    _ => (),
                }
            }
            for j in (i + 1)..rating.len() {
                match rating[j].cmp(&rating[i]) {
                    std::cmp::Ordering::Greater => rightmore += 1,
                    std::cmp::Ordering::Less => rightless += 1,
                    _ => (),
                }
            }
            ans += rightmore * leftless + leftmore * rightless;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3);
    assert_eq!(Solution::num_teams(vec![2, 1, 3]), 0);
    assert_eq!(Solution::num_teams(vec![1, 2, 3, 4]), 4);
}
