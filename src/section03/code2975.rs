#![allow(dead_code)]

// 2975. Maximum Square Area by Removing Fences From a Field
// https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/
// https://leetcode.cn/problems/maximum-square-area-by-removing-fences-from-a-field/
//
// Medium
//
// There is a large (m - 1) x (n - 1) rectangular field with corners at (1, 1) and (m, n) containing some horizontal and vertical fences given in arrays hFences and vFences respectively.
//
// Horizontal fences are from the coordinates (hFences[i], 1) to (hFences[i], n) and vertical fences are from the coordinates (1, vFences[i]) to (m, vFences[i]).
//
// Return the maximum area of a square field that can be formed by removing some fences (possibly none) or -1 if it is impossible to make a square field.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// Note: The field is surrounded by two horizontal fences from the coordinates (1, 1) to (1, n) and (m, 1) to (m, n) and two vertical fences from the coordinates (1, 1) to (m, 1) and (1, n) to (m, n). These fences cannot be removed.
//
// Example 1:
//
// Input: m = 4, n = 3, hFences = [2,3], vFences = [2]
// Output: 4
// Explanation: Removing the horizontal fence at 2 and the vertical fence at 2 will give a square field of area 4.
//
// Example 2:
//
// Input: m = 6, n = 7, hFences = [2], vFences = [4]
// Output: -1
// Explanation: It can be proved that there is no way to create a square field by removing fences.
//
// Constraints:
//
//     3 <= m, n <= 10^9
//     1 <= hFences.length, vFences.length <= 600
//     1 < hFences[i] < m
//     1 < vFences[i] < n
//     hFences and vFences are unique.
//

struct Solution;

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        let mut h_fences = h_fences;
        let mut v_fences = v_fences;

        h_fences.extend_from_slice(&[1, m]);
        h_fences.sort_unstable();

        let mut data = std::collections::HashSet::new();
        for i in 0..h_fences.len() - 1 {
            for j in i + 1..h_fences.len() {
                data.insert(h_fences[j] - h_fences[i]);
            }
        }

        let mut res = -1;
        v_fences.extend_from_slice(&[1, n]);
        v_fences.sort_unstable();
        for i in 0..v_fences.len() - 1 {
            for j in i + 1..v_fences.len() {
                let diff = v_fences[j] - v_fences[i];
                if data.contains(&diff) {
                    res = res.max(diff);
                }
            }
        }

        if res == -1 {
            return res;
        }

        (((res as u64) * (res as u64)) % 1000000007) as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximize_square_area(4, 3, vec![2, 3], vec![2]), 4);
    assert_eq!(Solution::maximize_square_area(6, 7, vec![2], vec![4]), -1);
    assert_eq!(Solution::maximize_square_area(5, 5, vec![3, 1], vec![1]), 16);
}
