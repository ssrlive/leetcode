#![allow(dead_code)]

// 777. Swap Adjacent in LR String
// https://leetcode.com/problems/swap-adjacent-in-lr-string/
// https://leetcode.cn/problems/swap-adjacent-in-lr-string/
//
// In a string composed of 'L', 'R', and 'X' characters, like "RXXLRXRXL", a move consists of either replacing one occurrence of "XL" with "LX",
// or replacing one occurrence of "RX" with "XR". Given the starting string start and the ending string end,
// return True if and only if there exists a sequence of moves to transform one string to the other.
//
// Example 1:
//
// Input: start = "RXXLRXRXL", end = "XRLXXRRLX"
// Output: true
// Explanation: We can transform start to end following these steps:
// RXXLRXRXL ->
// XRXLRXRXL ->
// XRLXRXRXL ->
// XRLXXRRXL ->
// XRLXXRRLX
//
// Example 2:
//
// Input: start = "X", end = "L"
// Output: false
//
// Constraints:
//
// - 1 <= start.length <= 10^4
// - start.length == end.length
// - Both start and end will only consist of characters in 'L', 'R', and 'X'.
//

struct Solution;

impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        let start = start.chars().collect::<Vec<_>>();
        let end = end.chars().collect::<Vec<_>>();
        let mut i = 0;
        let mut j = 0;
        while i < start.len() && j < end.len() {
            while i < start.len() && start[i] == 'X' {
                i += 1;
            }
            while j < end.len() && end[j] == 'X' {
                j += 1;
            }
            if i == start.len() || j == end.len() {
                break;
            }
            if start[i] != end[j] {
                return false;
            }
            if start[i] == 'L' && i < j {
                return false;
            }
            if start[i] == 'R' && i > j {
                return false;
            }
            i += 1;
            j += 1;
        }
        while i < start.len() {
            if start[i] != 'X' {
                return false;
            }
            i += 1;
        }
        while j < end.len() {
            if end[j] != 'X' {
                return false;
            }
            j += 1;
        }
        true
    }
}

#[test]
fn test() {
    let start = "RXXLRXRXL".to_string();
    let end = "XRLXXRRLX".to_string();
    assert_eq!(Solution::can_transform(start, end), true);

    assert_eq!(Solution::can_transform("X".to_string(), "L".to_string()), false);
}
