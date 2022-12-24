#![allow(dead_code)]

// 71. Simplify Path
// https://leetcode.com/problems/simplify-path/
// https://leetcode.cn/problems/simplify-path/
//
// Given a string path, which is an absolute path (starting with a slash '/') to a file or directory
// in a Unix-style file system, convert it to the simplified canonical path.
//
// In a Unix-style file system, a period '.' refers to the current directory, a double period '..' refers
// to the directory up a level, and any multiple consecutive slashes (i.e. '//') are treated as a single slash '/'.
// For this problem, any other format of periods such as '...' are treated as file/directory names.
//
// The canonical path should have the following format:
//
// The path starts with a single slash '/'.
// Any two directories are separated by a single slash '/'.
// The path does not end with a trailing '/'.
// The path only contains the directories on the path from the root directory to the target file or directory (i.e., no period '.' or double period '..')
// Return the simplified canonical path.
//
// Example 1:
//
// Input: path = "/home/"
// Output: "/home"
// Explanation: Note that there is no trailing slash after the last directory name.
//
// Example 2:
//
// Input: path = "/../"
// Output: "/"
// Explanation: Going one level up from the root directory is a no-op, as the root level is the highest level you can go.
//
// Example 3:
//
// Input: path = "/home//foo/"
// Output: "/home/foo"
// Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
//
// Constraints:
//
// - 1 <= path.length <= 3000
// - path consists of English letters, digits, period '.', slash '/' or '_'.
// - path is a valid absolute Unix path.
//

struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut entries = vec![];
        for entry in path.split('/') {
            match entry {
                "" | "." => continue,
                ".." => {
                    entries.pop();
                }
                _ => {
                    entries.push(entry);
                }
            };
        }
        match entries.len() {
            0 => "/".to_string(),
            _ => entries.iter().fold(String::new(), |mut a, &r| {
                a.push('/');
                a.push_str(r);
                a
            }),
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::simplify_path("/home/".to_string()), "/home");
    assert_eq!(Solution::simplify_path("/../".to_string()), "/");
    assert_eq!(Solution::simplify_path("/home//foo/".to_string()), "/home/foo");
    assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_string()), "/c");
    assert_eq!(Solution::simplify_path("/a/../../b/../c//.//".to_string()), "/c");
    assert_eq!(Solution::simplify_path("/a//b////c/d//././/..".to_string()), "/a/b/c");
}
