#![allow(dead_code)]

// 1177. Can Make Palindrome from Substring
// https://leetcode.com/problems/can-make-palindrome-from-substring/
// https://leetcode.cn/problems/can-make-palindrome-from-substring/
//
// You are given a string s and array queries where queries[i] = [lefti, righti, ki]. We may rearrange the substring
// s[lefti...righti] for each query and then choose up to ki of them to replace with any lowercase English letter.
//
// If the substring is possible to be a palindrome string after the operations above,
// the result of the query is true. Otherwise, the result is false.
//
// Return a boolean array answer where answer[i] is the result of the ith query queries[i].
//
// Note that each letter is counted individually for replacement, so if, for example s[lefti...righti] = "aaa",
// and ki = 2, we can only replace two of the letters. Also, note that no query modifies the initial string s.
//
// Example :
//
// Input: s = "abcda", queries = [[3,3,0],[1,2,0],[0,3,1],[0,3,2],[0,4,1]]
// Output: [true,false,false,true,true]
// Explanation:
// queries[0]: substring = "d", is palidrome.
// queries[1]: substring = "bc", is not palidrome.
// queries[2]: substring = "abcd", is not palidrome after replacing only 1 character.
// queries[3]: substring = "abcd", could be changed to "abba" which is palidrome. Also this can be changed
//   to "baab" first rearrange it "bacd" then replace "cd" with "ab".
// queries[4]: substring = "abcda", could be changed to "abcba" which is palidrome.
//
// Example 2:
//
// Input: s = "lyb", queries = [[0,1,0],[2,2,1]]
// Output: [false,true]
//
// Constraints:
//
// - 1 <= s.length, queries.length <= 10^5
// - 0 <= lefti <= righti < s.length
// - 0 <= ki <= s.length
// - s consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let s = s.chars().map(|v| (v as u8 - b'a') as usize).collect::<Vec<usize>>();
        let n = s.len();
        let mut memo = vec![vec![0; 26]; n + 1];

        for i in 0..n {
            memo[i + 1] = memo[i].clone();
            memo[i + 1][s[i]] += 1;
        }

        let q = queries.len();
        let mut result = vec![false; q];
        for i in 0..q {
            let l = queries[i][0] as usize;
            let r = queries[i][1] as usize;
            let c = queries[i][2] as usize;

            let mut odd = 0;
            for j in 0..26 {
                odd += (memo[r + 1][j] - memo[l][j]) % 2;
            }
            odd += odd % 2;

            let padding = (r - l + 1) % 2;
            result[i] = (odd / 2) <= (c + padding);
        }

        result
    }
}

#[test]
fn test() {
    let s = "abcda".to_string();
    let queries = vec![vec![3, 3, 0], vec![1, 2, 0], vec![0, 3, 1], vec![0, 3, 2], vec![0, 4, 1]];
    let result = vec![true, false, false, true, true];
    assert_eq!(Solution::can_make_pali_queries(s, queries), result);

    let s = "lyb".to_string();
    let queries = vec![vec![0, 1, 0], vec![2, 2, 1]];
    let result = vec![false, true];
    assert_eq!(Solution::can_make_pali_queries(s, queries), result);
}
