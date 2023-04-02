#![allow(dead_code)]

/*

// 2606. Find the Substring With Maximum Cost
// https://leetcode.com/problems/find-the-substring-with-maximum-cost/
// https://leetcode.cn/problems/find-the-substring-with-maximum-cost/
//
// Medium
//
// You are given a string s, a string chars of distinct characters and an integer array vals of the same length as chars.

The cost of the substring is the sum of the values of each character in the substring. The cost of an empty string is considered 0.

The value of the character is defined in the following way:

    If the character is not in the string chars, then its value is its corresponding position (1-indexed) in the alphabet.
        For example, the value of 'a' is 1, the value of 'b' is 2, and so on. The value of 'z' is 26.
    Otherwise, assuming i is the index where the character occurs in the string chars, then its value is vals[i].

Return the maximum cost among all substrings of the string s.

Example 1:

Input: s = "adaa", chars = "d", vals = [-1000]
Output: 2
Explanation: The value of the characters "a" and "d" is 1 and -1000 respectively.
The substring with the maximum cost is "aa" and its cost is 1 + 1 = 2.
It can be proven that 2 is the maximum cost.

Example 2:

Input: s = "abc", chars = "abc", vals = [-1,-1,-1]
Output: 0
Explanation: The value of the characters "a", "b" and "c" is -1, -1, and -1 respectively.
The substring with the maximum cost is the empty substring "" and its cost is 0.
It can be proven that 0 is the maximum cost.

Constraints:

    1 <= s.length <= 10^5
    s consist of lowercase English letters.
    1 <= chars.length <= 26
    chars consist of distinct lowercase English letters.
    vals.length == chars.length
    -1000 <= vals[i] <= 1000
*/

struct Solution;

impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        fn maximum_subarray(nums: Vec<i32>) -> i32 {
            let (mut max, mut cur) = (i32::MIN, 0);
            for &num in nums.iter() {
                cur = i32::max(num, cur + num);
                max = i32::max(max, cur);
            }
            max
        }

        let mut costs = [0; 26];
        let (s, chars) = (s.into_bytes(), chars.into_bytes());
        for (i, cost) in costs.iter_mut().enumerate() {
            *cost = (i + 1) as i32;
        }
        for i in 0..chars.len() {
            costs[(chars[i] - b'a') as usize] = vals[i];
        }

        let nums = s.into_iter().map(|c| costs[(c - b'a') as usize]).collect::<Vec<i32>>();

        i32::max(0, maximum_subarray(nums))
    }
}

#[test]
fn test() {
    let cases = vec![
        ("adaa", "d", vec![-1000], 2),
        ("abc", "abc", vec![-1, -1, -1], 0),
        ("abab", "ab", vec![1, 2], 6),
    ];
    for (s, c, v, e) in cases {
        assert_eq!(Solution::maximum_cost_substring(s.to_string(), c.to_string(), v), e);
    }
}
