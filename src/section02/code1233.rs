#![allow(dead_code)]

// 1233. Remove Sub-Folders from the Filesystem
// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/
// https://leetcode.cn/problems/remove-sub-folders-from-the-filesystem/
//
// Medium
//
// Given a list of folders folder, return the folders after removing all sub-folders in those folders. You may return the answer in any order.
//
// If a folder[i] is located within another folder[j], it is called a sub-folder of it.
//
// The format of a path is one or more concatenated strings of the form: '/' followed by one or more lowercase English letters.
//
//     For example, "/leetcode" and "/leetcode/problems" are valid paths while an empty string and "/" are not.
//
// Example 1:
//
// Input: folder = ["/a","/a/b","/c/d","/c/d/e","/c/f"]
// Output: ["/a","/c/d","/c/f"]
// Explanation: Folders "/a/b" is a subfolder of "/a" and "/c/d/e" is inside of folder "/c/d" in our filesystem.
//
// Example 2:
//
// Input: folder = ["/a","/a/b/c","/a/b/d"]
// Output: ["/a"]
// Explanation: Folders "/a/b/c" and "/a/b/d" will be removed because they are subfolders of "/a".
//
// Example 3:
//
// Input: folder = ["/a/b/c","/a/b/ca","/a/b/d"]
// Output: ["/a/b/c","/a/b/ca","/a/b/d"]
//
// Constraints:
//
// -    1 <= folder.length <= 4 * 10^4
// -    2 <= folder[i].length <= 100
// -    folder[i] contains only lowercase letters and '/'.
// -    folder[i] always starts with the character '/'.
// -    Each folder name is unique.
//

struct Solution;

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort();

        let mut result = vec![];
        for f in folder {
            if result.is_empty() || !f.starts_with(format!("{}{}", result.last().unwrap(), "/").as_str()) {
                result.push(f);
            }
        }

        result
    }
}

#[test]
fn test() {
    let folder = vec![
        "/a".to_string(),
        "/a/b".to_string(),
        "/c/d".to_string(),
        "/c/d/e".to_string(),
        "/c/f".to_string(),
    ];
    let ans = vec!["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()];
    assert_eq!(Solution::remove_subfolders(folder), ans);

    let folder = vec!["/a".to_string(), "/a/b/c".to_string(), "/a/b/d".to_string()];
    let ans = vec!["/a".to_string()];
    assert_eq!(Solution::remove_subfolders(folder), ans);
}
