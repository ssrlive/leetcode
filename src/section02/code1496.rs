#![allow(dead_code)]

/*

// 1496. Path Crossing
// https://leetcode.com/problems/path-crossing/
// https://leetcode.cn/problems/path-crossing/
//
// Easy
//
// Given a string path, where path[i] = 'N', 'S', 'E' or 'W', each representing moving one unit north, south, east, or west, respectively. You start at the origin (0, 0) on a 2D plane and walk on the path specified by path.
//
// Return true if the path crosses itself at any point, that is, if at any time you are on a location you have previously visited. Return false otherwise.
//
// Example 1:

Input: path = "NES"
Output: false
Explanation: Notice that the path doesn't cross any point more than once.

Example 2:

Input: path = "NESWW"
Output: true
Explanation: Notice that the path visits the origin twice.



Constraints:

    1 <= path.length <= 10^4
    path[i] is either 'N', 'S', 'E', or 'W'.
*/

struct Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut visited = std::collections::HashSet::new();
        visited.insert((x, y));
        for c in path.chars() {
            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => unreachable!(),
            }
            if visited.contains(&(x, y)) {
                return true;
            }
            visited.insert((x, y));
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_path_crossing("NES".to_string()), false);
    assert_eq!(Solution::is_path_crossing("NESWW".to_string()), true);
}
