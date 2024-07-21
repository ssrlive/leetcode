#![allow(dead_code)]

// 3228. Maximum Number of Operations to Move Ones to the End
// https://leetcode.com/problems/maximum-number-of-operations-to-move-ones-to-the-end/
// https://leetcode.cn/problems/maximum-number-of-operations-to-move-ones-to-the-end/
//
// Medium
//
// You are given a binary string s.
//
// You can perform the following operation on the string any number of times:
//
// - Choose any index i from the string where i + 1 < s.length such that s[i] == '1' and s[i + 1] == '0'.
// - Move the character s[i] to the right until it reaches the end of the string or another '1'.
//   For example, for s = "010010", if we choose i = 1, the resulting string will be s = "000110".
//
// Return the maximum number of operations that you can perform.
//
// Example 1:
//
// Input: s = "1001101"
//
// Output: 4
//
// Explanation:
//
// We can perform the following operations:
//
//     Choose index i = 0. The resulting string is s = "0011101".
//     Choose index i = 4. The resulting string is s = "0011011".
//     Choose index i = 3. The resulting string is s = "0010111".
//     Choose index i = 2. The resulting string is s = "0001111".
//
// Example 2:
//
// Input: s = "00111"
//
// Output: 0
//
// Constraints:
//
//     1 <= s.length <= 10^5
//     s[i] is either '0' or '1'.
//

struct Solution;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let n = s.len();
        let mut cnt = 0;
        let mut ans = 0;
        let mut i = 0;
        let s = s.as_bytes();
        while i < n {
            if s[i] == b'0' {
                ans += cnt;
                while i < n && s[i] != b'1' {
                    i += 1;
                }
            }
            cnt += 1;
            i += 1;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_operations("1001101".to_string()), 4);
    assert_eq!(Solution::max_operations("00111".to_string()), 0);
}
