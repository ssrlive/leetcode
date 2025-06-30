#![allow(dead_code)]

// 3598. Longest Common Prefix Between Adjacent Strings After Removals
// https://leetcode.com/problems/longest-common-prefix-between-adjacent-strings-after-removals/
// https://leetcode.cn/problems/longest-common-prefix-between-adjacent-strings-after-removals/
//
// Medium
//
// You are given an array of strings words. For each index i in the range [0, words.length - 1], perform the following steps:
//
//     Remove the element at index i from the words array.
//     Compute the length of the longest common
//
//     among all adjacent pairs in the modified array.
//
// Return an array answer, where answer[i] is the length of the longest common prefix between
// the adjacent pairs after removing the element at index i.
// If no adjacent pairs remain or if none share a common prefix, then answer[i] should be 0.
//
// Example 1:
//
// Input: words = ["jump","run","run","jump","run"]
//
// Output: [3,0,0,3,3]
//
// Explanation:
//
//     Removing index 0:
//         words becomes ["run", "run", "jump", "run"]
//         Longest adjacent pair is ["run", "run"] having a common prefix "run" (length 3)
//     Removing index 1:
//         words becomes ["jump", "run", "jump", "run"]
//         No adjacent pairs share a common prefix (length 0)
//     Removing index 2:
//         words becomes ["jump", "run", "jump", "run"]
//         No adjacent pairs share a common prefix (length 0)
//     Removing index 3:
//         words becomes ["jump", "run", "run", "run"]
//         Longest adjacent pair is ["run", "run"] having a common prefix "run" (length 3)
//     Removing index 4:
//         words becomes ["jump", "run", "run", "jump"]
//         Longest adjacent pair is ["run", "run"] having a common prefix "run" (length 3)
//
// Example 2:
//
// Input: words = ["dog","racer","car"]
//
// Output: [0,0,0]
//
// Explanation:
//
//     Removing any index results in an answer of 0.
//
// Constraints:
//
//     1 <= words.length <= 10^5
//     1 <= words[i].length <= 10^4
//     words[i] consists of lowercase English letters.
//     The sum of words[i].length is smaller than or equal 10^5.

struct Solution;

impl Solution {
    pub fn longest_common_prefix(words: Vec<String>) -> Vec<i32> {
        fn adjacent_prefix(words: &[String], i: usize) -> usize {
            let mini = words[i - 1].len().min(words[i + 1].len());
            let mut k = 0;
            while k < mini && words[i - 1].as_bytes()[k] == words[i + 1].as_bytes()[k] {
                k += 1;
            }
            k
        }

        let n = words.len();
        let mut ans = vec![0; n];
        let mut maxlen = 0;
        let mut secondmaxlen = 0;
        let mut maxindex = -1;
        for i in 0..n - 1 {
            let mini = words[i].len().min(words[i + 1].len());
            let mut k = 0;
            while k < mini && words[i].as_bytes()[k] == words[i + 1].as_bytes()[k] {
                k += 1;
            }

            if k > maxlen {
                secondmaxlen = maxlen;
                maxlen = k;
                maxindex = i as i32;
            } else if k > secondmaxlen {
                secondmaxlen = k;
            }
        }
        for (i, ans_i) in ans.iter_mut().enumerate().take(n) {
            if i == 0 || i == n - 1 {
                if i as i32 == maxindex || i as i32 == maxindex + 1 {
                    *ans_i = secondmaxlen as i32;
                } else {
                    *ans_i = maxlen as i32;
                }
            } else {
                let a = adjacent_prefix(&words, i);
                if a > maxlen {
                    *ans_i = a as i32;
                } else if i as i32 == maxindex || i as i32 == maxindex + 1 {
                    *ans_i = secondmaxlen as i32;
                } else {
                    *ans_i = maxlen as i32;
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let words = vec!["jump", "run", "run", "jump", "run"].into_iter().map(String::from).collect();
    let result = Solution::longest_common_prefix(words);
    assert_eq!(result, vec![3, 0, 0, 3, 3]);

    let words = vec!["dog", "racer", "car"].into_iter().map(String::from).collect();
    let result = Solution::longest_common_prefix(words);
    assert_eq!(result, vec![0, 0, 0]);
}
