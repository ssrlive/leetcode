#![allow(dead_code)]

/*

// 1528. Shuffle String
// https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array/
// https://leetcode.cn/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array/
//
// Easy
//
// You are given a string s and an integer array indices of the same length. The string s will be shuffled such that the character at the ith position moves to indices[i] in the shuffled string.

Return the shuffled string.

Example 1:

Input: s = "codeleet", indices = [4,5,6,7,0,2,1,3]
Output: "leetcode"
Explanation: As shown, "codeleet" becomes "leetcode" after shuffling.

Example 2:

Input: s = "abc", indices = [0,1,2]
Output: "abc"
Explanation: After shuffling, each character remains in its position.

Constraints:

    s.length == indices.length == n
    1 <= n <= 100
    s consists of only lowercase English letters.
    0 <= indices[i] < n
    All values of indices are unique.
*/

struct Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut ans = vec![' '; s.len()];
        for (i, c) in s.chars().enumerate() {
            ans[indices[i] as usize] = c;
        }
        ans.into_iter().collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("codeleet", vec![4, 5, 6, 7, 0, 2, 1, 3], "leetcode"),
        ("abc", vec![0, 1, 2], "abc"),
    ];
    for (s, indices, expect) in cases {
        assert_eq!(Solution::restore_string(s.to_string(), indices), expect);
    }
}
