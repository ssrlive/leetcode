#![allow(dead_code)]

// 3445. Maximum Difference Between Even and Odd Frequency II
// https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-ii/
// https://leetcode.cn/problems/maximum-difference-between-even-and-odd-frequency-ii/
//
// Hard
//
// You are given a string s and an integer k. Your task is to find the maximum difference between the
// frequency of two characters, freq[a] - freq[b], in a substring subs of s, such that:
//
//     subs has a size of at least k.
//     Character a has an odd frequency in subs.
//     Character b has an even frequency in subs.
//
// Return the maximum difference.
//
// Note that subs can contain more than 2 distinct characters.
//
// Example 1:
//
// Input: s = "12233", k = 4
//
// Output: -1
//
// Explanation:
//
// For the substring "12233", the frequency of '1' is 1 and the frequency of '3' is 2. The difference is 1 - 2 = -1.
//
// Example 2:
//
// Input: s = "1122211", k = 3
//
// Output: 1
//
// Explanation:
//
// For the substring "11222", the frequency of '2' is 3 and the frequency of '1' is 2. The difference is 3 - 2 = 1.
//
// Example 3:
//
// Input: s = "110", k = 3
//
// Output: -1
//
// Constraints:
//
//     3 <= s.length <= 3 * 10^4
//     s consists only of digits '0' to '4'.
//     The input is generated that at least one substring has a character with an even frequency and a character with an odd frequency.
//     1 <= k <= s.length
//

struct Solution;

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        fn get(a: i32, b: i32) -> i32 {
            let mut ans = 0;
            ans |= (a % 2) << 1;
            ans |= b % 2;
            ans
        }
        fn dfs(s: &[u8], a: u8, b: u8, k: i32, n: i32) -> i32 {
            let (mut ca, mut cb) = (0, 0);
            let mut arr = vec![(0, 0); n as usize];
            let mut diff = [i32::MAX; 4];
            let mut ans = i32::MIN;
            let mut j = 0;
            for i in 0..n as usize {
                let c = s[i];
                if c == a {
                    ca += 1;
                } else if c == b {
                    cb += 1;
                }
                if i as i32 - k + 1 == 0 {
                    diff[0] = 0;
                }
                while j + k as usize <= i && arr[j].0 < ca && arr[j].1 < cb {
                    let d = arr[j].0 - arr[j].1;
                    let index = get(arr[j].0, arr[j].1) as usize;
                    diff[index] = diff[index].min(d);
                    j += 1;
                }
                let ri = get(ca + 1, cb) as usize;
                if diff[ri] != i32::MAX && ca > 0 && cb > 0 {
                    ans = ans.max(ca - cb - diff[ri]);
                }
                arr[i] = (ca, cb);
            }
            ans
        }

        let s = s.as_bytes();
        let n = s.len() as i32;
        let mut ans = i32::MIN;
        let mut mp = [0; 5];
        for c in s.iter() {
            mp[(c - b'0') as usize] += 1;
        }
        for a in b'0'..=b'4' {
            for b in b'0'..=b'4' {
                if a == b {
                    continue;
                }
                if mp[(a - b'0') as usize] == 0 {
                    continue;
                }
                if mp[(b - b'0') as usize] == 0 {
                    continue;
                }
                let cur = dfs(s, a, b, k, n);
                ans = ans.max(cur);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let s = "12233".to_string();
    let k = 4;
    let output = -1;
    assert_eq!(Solution::max_difference(s, k), output);

    let s = "1122211".to_string();
    let k = 3;
    let output = 1;
    assert_eq!(Solution::max_difference(s, k), output);

    let s = "110".to_string();
    let k = 3;
    let output = -1;
    assert_eq!(Solution::max_difference(s, k), output);
}
