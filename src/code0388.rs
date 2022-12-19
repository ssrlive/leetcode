#![allow(dead_code)]

// 388. Longest Absolute File Path
// https://leetcode.com/problems/longest-absolute-file-path/
// https://leetcode.cn/problems/longest-absolute-file-path/
//
// Suppose we abstract our file system by a string in the following manner:
//
// The string "dir
//     file.txt" represents:
//
// dir
//     file.txt
//
// The string "dir
//     subdir1
//     subdir2
//         file.ext" represents:
//
// dir
//     subdir1
//     subdir2
//         file.ext
//
// The string "dir
//     subdir1
//         file1.ext
//         subsubdir1
//     subdir2
//         subsubdir2
//             file2.ext" represents:
//
// dir
//     subdir1
//         file1.ext
//         subsubdir1
//     subdir2
//         subsubdir2
//             file2.ext
//
// We are interested in finding the longest (number of characters) absolute path
// to a file within our file system. For example, in the second example above,
// the longest absolute path is "dir/subdir2/subsubdir2/file2.ext", and its
// length is 32 (not including the double quotes).
//
// Given a string representing the file system in the above format, return the
// length of the longest absolute path to file in the abstracted file system. If
// there is no file in the system, return 0.
//
// Note:
//
// The name of a file contains at least a . and an extension.
// The name of a directory or sub-directory will not contain a ..
//
// Time complexity required: O(n) where n is the size of the input string.
//
// Notice that a/aa/aaa/file1.txt is not the longest file path, if there is
// another path aaaaaaaaaaaaaaaaaaaaa/sth.png.
//
// Constraints:
//
// - 1 <= input.length <= 10^4
// - input may contain lower-case or upper-case English letters, a new line
// - character '\', a tab character '\t', a dot '.', a space ' ', and digits.
//

struct Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut max = 0;
        let mut stack = vec![];
        for line in input.lines() {
            let level = line.chars().take_while(|&c| c == '\t').count();
            let len = line.len() - level;
            while stack.len() > level {
                stack.pop();
            }
            if line.contains('.') {
                max = std::cmp::max(max, stack.iter().sum::<usize>() + len);
            } else {
                stack.push(len + 1);
            }
        }
        max as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::length_longest_path("dir\n\tfile.txt".to_string()), 12);
    assert_eq!(
        Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string()),
        20
    );
    assert_eq!(
        Solution::length_longest_path(
            "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_string()
        ),
        32
    );
}
