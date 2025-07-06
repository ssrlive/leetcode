#![allow(dead_code)]

// 927. Three Equal Parts
// https://leetcode.com/problems/three-equal-parts/
// https://leetcode.cn/problems/three-equal-parts/
//
// You are given an array arr which consists of only zeros and ones, divide the array into three non-empty parts such that all of these parts represent the same binary value.
//
// If it is possible, return any [i, j] with i + 1 < j, such that:
//
// arr[0], arr[1], ..., arr[i] is the first part,
// arr[i + 1], arr[i + 2], ..., arr[j - 1] is the second part, and
// arr[j], arr[j + 1], ..., arr[arr.length - 1] is the third part.
// All three parts have equal binary values.
// If it is not possible, return [-1, -1].
//
// Note that the entire part is used when considering what binary value it represents. For example, [1,1,0] represents 6 in decimal, not 3. Also, leading zeros are allowed, so [0,1,1] and [1,1] represent the same value.
//
// Example 1:
//
// Input: arr = [1,0,1,0,1]
// Output: [0,3]
//
// Example 2:
//
// Input: arr = [1,1,0,1,1]
// Output: [-1,-1]
//
// Example 3:
//
// Input: arr = [1,1,0,0,1]
// Output: [0,2]
//
// Constraints:
//
// - 3 <= arr.length <= 3 * 10^4
// - arr[i] is 0 or 1
//

struct Solution;

impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let v = arr
            .iter()
            .enumerate()
            .filter_map(|(i, &a)| if a == 1 { Some(i) } else { None })
            .collect::<Vec<_>>();
        if arr.len() < 3 || !v.len().is_multiple_of(3) {
            return [-1, -1].to_vec();
        }
        if v.is_empty() {
            return [0, 2].to_vec();
        }
        let chunks = v.chunks(v.len() / 3).collect::<Vec<_>>();
        let i = chunks[0][v.len() / 3 - 1] + arr.len() - v[v.len() - 1] - 1;
        let j = chunks[1][v.len() / 3 - 1] + arr.len() - v[v.len() - 1];
        if arr.len() - chunks[2][0] > j - i - 1 || arr.len() - chunks[2][0] > i + 1 {
            return [-1, -1].to_vec();
        }
        for k in 0..(i + 1).min(j - i - 1).min(arr.len() - j) {
            if arr[i - k] != arr[j - 1 - k] || arr[i - k] != arr[arr.len() - 1 - k] {
                return [-1, -1].to_vec();
            }
        }
        [i as i32, j as i32].to_vec()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::three_equal_parts(vec![1, 0, 1, 0, 1]), [0, 3]);
    assert_eq!(Solution::three_equal_parts(vec![1, 1, 0, 1, 1]), [-1, -1]);
    assert_eq!(Solution::three_equal_parts(vec![1, 1, 0, 0, 1]), [0, 2]);
    assert_eq!(Solution::three_equal_parts(vec![1, 1, 0, 0, 0]), [-1, -1]);
    assert_eq!(Solution::three_equal_parts(vec![1, 1, 0, 0, 0, 0]), [-1, -1]);
    assert_eq!(Solution::three_equal_parts(vec![1, 1, 0, 0, 0, 0, 0]), [-1, -1]);
}
