#![allow(dead_code)]

// 646. Maximum Length of Pair Chain
// https://leetcode.com/problems/maximum-length-of-pair-chain/
//
// You are given an array of n pairs pairs where pairs[i] = [lefti, righti] and lefti < righti.
//
// A pair p2 = [c, d] follows a pair p1 = [a, b] if b < c. A chain of pairs can be formed in this fashion.
//
// Return the length longest chain which can be formed.
//
// You do not need to use up all the given intervals. You can select pairs in any order.
//
// Example 1:
//
// Input: pairs = [[1,2],[2,3],[3,4]]
// Output: 2
// Explanation: The longest chain is [1,2] -> [3,4].
//
// Example 2:
//
// Input: pairs = [[1,2],[7,8],[4,5]]
// Output: 3
// Explanation: The longest chain is [1,2] -> [4,5] -> [7,8].
//
// Constraints:
//
// - n == pairs.length
// - 1 <= n <= 1000
// - -1000 <= lefti < righti <= 1000
//

struct Solution;

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        pairs.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut count = 0;
        let mut last = i32::MIN;
        for pair in pairs {
            if pair[0] > last {
                count += 1;
                last = pair[1];
            }
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        2
    );
    assert_eq!(
        Solution::find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]),
        3
    );
}
