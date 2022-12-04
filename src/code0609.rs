#![allow(dead_code)]

// 609. Find Duplicate File in System
// https://leetcode.com/problems/find-duplicate-file-in-system/
//
// Given a list paths of directory info, including the directory path, and all the files with contents in this directory, return all the duplicate files in the file system in terms of their paths. You may return the answer in any order.
//
// A group of duplicate files consists of at least two files that have the same content.
//
// A single directory info string in the input list has the following format:
//
// - "root/d1/d2/.../dm f1.txt(f1_content) f2.txt(f2_content) ... fn.txt(fn_content)"
//
// It means there are n files (f1.txt, f2.txt ... fn.txt) with content (f1_content, f2_content ... fn_content) respectively in the directory "root/d1/d2/.../dm". Note that n >= 1 and m >= 0. If m = 0, it means the directory is just the root directory.
//
// The output is a list of groups of duplicate file paths. For each group, it contains all the file paths of the files that have the same content. A file path is a string that has the following format:
//
// - "directory_path/file_name.txt"
//
// Example 1:
//
// Input: paths = ["root/a 1.txt(abcd) 2.txt(efgh)","root/c 3.txt(abcd)","root/c/d 4.txt(efgh)","root 4.txt(efgh)"]
// Output: [["root/a/2.txt","root/c/d/4.txt","root/4.txt"],["root/a/1.txt","root/c/3.txt"]]
//
// Example 2:
//
// Input: paths = ["root/a 1.txt(abcd) 2.txt(efgh)","root/c 3.txt(abcd)","root/c/d 4.txt(efgh)"]
// Output: [["root/a/2.txt","root/c/d/4.txt"],["root/a/1.txt","root/c/3.txt"]]
//
// Constraints:
//
// - 1 <= paths.length <= 2 * 10^4
// - 1 <= paths[i].length <= 3000
// - 1 <= sum(paths[i].length) <= 5 * 10^5
// - paths[i] consist of English letters, digits, '/', '.', '(', ')', and ' '.
// - You may assume no files or directories share the same name in the same directory.
// - You may assume each given directory info represents a unique directory. A single blank space separates the directory path and file info.
//
// Follow up:
//
// - Imagine you are given a real file system, how will you search files? DFS or BFS?
// - If the file content is very large (GB level), how will you modify your solution?
// - If you can only read the file by 1kb each time, how will you modify your solution?
// - What is the time complexity of your modified solution? What is the most time-consuming part and memory-consuming part of it? How to optimize?
// - How to make sure the duplicated files you find are not false positive?
//

struct Solution;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();
        for path in paths {
            let mut iter = path.split_whitespace();
            let dir = iter.next().unwrap();
            for file in iter {
                let mut iter = file.split('(');
                let file_name = iter.next().unwrap();
                let content = iter.next().unwrap();
                let content = &content[..content.len() - 1];
                map.entry(content.to_string())
                    .or_insert_with(Vec::new)
                    .push(format!("{}/{}", dir, file_name));
            }
        }
        map.into_iter().filter(|(_, v)| v.len() > 1).map(|(_, v)| v).collect()
    }
}

#[test]
fn test() {
    let paths = vec![
        "root/a 1.txt(abcd) 2.txt(efgh)".to_string(),
        "root/c 3.txt(abcd)".to_string(),
        "root/c/d 4.txt(efgh)".to_string(),
        "root 4.txt(efgh)".to_string(),
    ];
    let mut result = Solution::find_duplicate(paths);
    result.sort();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].len(), 2);
    assert_eq!(result[1].len(), 3);
}
