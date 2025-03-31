#![allow(dead_code)]

// 3501. Maximize Active Section with Trade II
// https://leetcode.com/problems/maximize-active-section-with-trade-ii/
// https://leetcode.cn/problems/maximize-active-section-with-trade-ii/
//
// Hard
//
// You are given a binary string s of length n, where:
//
//     '1' represents an active section.
//     '0' represents an inactive section.
//
// You can perform at most one trade to maximize the number of active sections in s. In a trade, you:
//
//     Convert a contiguous block of '1's that is surrounded by '0's to all '0's.
//     Afterward, convert a contiguous block of '0's that is surrounded by '1's to all '1's.
//
// Additionally, you are given a 2D array queries, where queries[i] = [li, ri] represents a s[li...ri].
//
// For each query, determine the maximum possible number of active sections in s after making the optimal trade on the substring s[li...ri].
//
// Return an array answer, where answer[i] is the result for queries[i].
//
// Note
//
// - For each query, treat s[li...ri] as if it is augmented with a '1' at both ends, forming t = '1' + s[li...ri] + '1'.
//   The augmented '1's do not contribute to the final count.
// - The queries are independent of each other.
//
// Example 1:
//
// Input: s = "01", queries = [[0,1]]
//
// Output: [1]
//
// Explanation:
//
// Because there is no block of '1's surrounded by '0's, no valid trade is possible. The maximum number of active sections is 1.
//
// Example 2:
//
// Input: s = "0100", queries = [[0,3],[0,2],[1,3],[2,3]]
//
// Output: [4,3,1,1]
//
// Explanation:
//
//     Query [0, 3] → Substring "0100" → Augmented to "101001"
//     Choose "0100", convert "0100" → "0000" → "1111".
//     The final string without augmentation is "1111". The maximum number of active sections is 4.
//
//     Query [0, 2] → Substring "010" → Augmented to "10101"
//     Choose "010", convert "010" → "000" → "111".
//     The final string without augmentation is "1110". The maximum number of active sections is 3.
//
//     Query [1, 3] → Substring "100" → Augmented to "11001"
//     Because there is no block of '1's surrounded by '0's, no valid trade is possible. The maximum number of active sections is 1.
//
//     Query [2, 3] → Substring "00" → Augmented to "1001"
//     Because there is no block of '1's surrounded by '0's, no valid trade is possible. The maximum number of active sections is 1.
//
// Example 3:
//
// Input: s = "1000100", queries = [[1,5],[0,6],[0,4]]
//
// Output: [6,7,2]
//
// Explanation:
//
//     Query [1, 5] → Substring "00010" → Augmented to "1000101"
//     Choose "00010", convert "00010" → "00000" → "11111".
//     The final string without augmentation is "1111110". The maximum number of active sections is 6.
//
//     Query [0, 6] → Substring "1000100" → Augmented to "110001001"
//     Choose "000100", convert "000100" → "000000" → "111111".
//     The final string without augmentation is "1111111". The maximum number of active sections is 7.
//
//     Query [0, 4] → Substring "10001" → Augmented to "1100011"
//     Because there is no block of '1's surrounded by '0's, no valid trade is possible. The maximum number of active sections is 2.
//
// Example 4:
//
// Input: s = "01010", queries = [[0,3],[1,4],[1,3]]
//
// Output: [4,4,2]
//
// Explanation:
//
//     Query [0, 3] → Substring "0101" → Augmented to "101011"
//     Choose "010", convert "010" → "000" → "111".
//     The final string without augmentation is "11110". The maximum number of active sections is 4.
//
//     Query [1, 4] → Substring "1010" → Augmented to "110101"
//     Choose "010", convert "010" → "000" → "111".
//     The final string without augmentation is "01111". The maximum number of active sections is 4.
//
//     Query [1, 3] → Substring "101" → Augmented to "11011"
//     Because there is no block of '1's surrounded by '0's, no valid trade is possible. The maximum number of active sections is 2.
//
// Constraints:
//
//     1 <= n == s.length <= 10^5
//     1 <= queries.length <= 10^5
//     s[i] is either '0' or '1'.
//     queries[i] = [li, ri]
//     0 <= li <= ri < n
//

struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const K: usize = 25;
        const MAXN: usize = 1e5 as usize;
        let mut st = vec![vec![0; MAXN]; K + 1];

        fn build(array: &[i32], st: &mut [Vec<i32>]) {
            let n = array.len();
            st[0][..n].copy_from_slice(array);
            for i in 1..=K {
                let mut j = 0;
                while j + (1 << i) <= n {
                    st[i][j] = st[i - 1][j].max(st[i - 1][j + (1 << (i - 1))]);
                    j += 1;
                }
            }
        }

        fn query(ll: usize, rr: usize, st: &[Vec<i32>]) -> i32 {
            let len = rr + 1 - ll;
            let i = if len == 1 {
                0
            } else {
                (len as u32).next_power_of_two().trailing_zeros() as usize - 1
            };
            st[i][ll].max(st[i][rr + 1 - (1 << i)])
        }

        let s = s.as_bytes();
        let n = s.len();
        let mut active = 0;
        let mut zero: Vec<(i32, i32)> = vec![];
        let mut index = vec![0; n];
        for i in 0..n {
            if s[i] == b'0' {
                if i > 0 && s[i - 1] == b'0' {
                    zero.last_mut().unwrap().1 += 1;
                } else {
                    zero.push((i as i32, 1));
                }
            } else {
                active += 1;
            }
            index[i] = zero.len() as i32 - 1;
        }
        if zero.is_empty() {
            return vec![active; queries.len()];
        }
        let mut gains = vec![0; zero.len() - 1];
        for i in (0..zero.len() - 1).rev() {
            gains[i] = zero[i].1 + zero[i + 1].1;
        }
        build(&gains, &mut st);
        let mut res = vec![active; queries.len()];
        for i in 0..queries.len() {
            let l = queries[i][0] as usize;
            let r = queries[i][1] as usize;
            let start = index[l] + 1;
            let end = index[r] - (s[r] == b'0') as i32;
            let cnt_left = if index[l] == -1 {
                -1
            } else {
                zero[index[l] as usize].1 - (l as i32 - zero[index[l] as usize].0)
            };
            let cnt_right = if index[r] == -1 {
                -1
            } else {
                r as i32 - zero[index[r] as usize].0 + 1
            };

            if start < end {
                res[i] = res[i].max(active + query(start as usize, end as usize - 1, &st));
            }
            if s[l] == b'0' && s[r] == b'0' && index[l] + 1 == index[r] {
                res[i] = res[i].max(active + cnt_left + cnt_right);
            }
            if s[l] == b'0' && index[l] + 1 < index[r] + (s[r] == b'1') as i32 {
                res[i] = res[i].max(active + cnt_left + zero[index[l] as usize + 1].1);
            }
            if s[r] == b'0' && index[l] < index[r] - 1 {
                res[i] = res[i].max(active + cnt_right + zero[index[r] as usize - 1].1);
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "1000100".to_string();
    let queries = vec![vec![1, 5], vec![0, 6], vec![0, 4]];
    let result = Solution::max_active_sections_after_trade(s, queries);
    assert_eq!(result, vec![6, 7, 2]);

    let s = "01010".to_string();
    let queries = vec![vec![0, 3], vec![1, 4], vec![1, 3]];
    let result = Solution::max_active_sections_after_trade(s, queries);
    assert_eq!(result, vec![4, 4, 2]);

    let s = "10110111".to_string();
    let queries = vec![vec![3, 7], vec![4, 6], vec![0, 6]];
    let result = Solution::max_active_sections_after_trade(s, queries);
    assert_eq!(result, [6, 6, 8]);
}
