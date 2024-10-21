#![allow(dead_code)]

// 3327. Check if DFS Strings Are Palindromes
// https://leetcode.com/problems/check-if-dfs-strings-are-palindromes/
// https://leetcode.cn/problems/check-if-dfs-strings-are-palindromes/
//
// Hard
//
// You are given a tree rooted at node 0, consisting of n nodes numbered from 0 to n - 1.
// The tree is represented by an array parent of size n, where parent[i] is the parent of node i.
// Since node 0 is the root, parent[0] == -1.
//
// You are also given a string s of length n, where s[i] is the character assigned to node i.
//
// Consider an empty string dfsStr, and define a recursive function dfs(int x) that takes
// a node x as a parameter and performs the following steps in order:
//
// Iterate over each child y of x in increasing order of their numbers, and call dfs(y).
// Add the character s[x] to the end of the string dfsStr.
// Note that dfsStr is shared across all recursive calls of dfs.
//
// You need to find a boolean array answer of size n, where for each index i from 0 to n - 1, you do the following:
//
// Empty the string dfsStr and call dfs(i).
// If the resulting string dfsStr is a palindrome, then set answer[i] to true. Otherwise, set answer[i] to false.
// Return the array answer.
//
// A palindrome is a string that reads the same forward and backward.
//
// Example 1:
//
// Input: parent = [-1,0,0,1,1,2], s = "aababa"
//
// Output: [true,true,false,true,true,true]
//
// Explanation:
//
// Calling dfs(0) results in the string dfsStr = "abaaba", which is a palindrome.
// Calling dfs(1) results in the string dfsStr = "aba", which is a palindrome.
// Calling dfs(2) results in the string dfsStr = "ab", which is not a palindrome.
// Calling dfs(3) results in the string dfsStr = "a", which is a palindrome.
// Calling dfs(4) results in the string dfsStr = "b", which is a palindrome.
// Calling dfs(5) results in the string dfsStr = "a", which is a palindrome.
//
// Example 2:
//
// Input: parent = [-1,0,0,0,0], s = "aabcb"
//
// Output: [true,true,true,true,true]
//
// Explanation:
//
// Every call on dfs(x) results in a palindrome string.
//
// Constraints:
//
// n == parent.length == s.length
// 1 <= n <= 10^5
// 0 <= parent[i] <= n - 1 for all i >= 1.
// parent[0] == -1
// parent represents a valid tree.
// s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = s.len();
        let mut count = 0;
        let mut answer = vec![false; n];
        let mut ch = vec![vec![]; n];
        for i in 1..parent.len() {
            ch[parent[i] as usize].push(i as i32);
        }
        let mut traversed = vec![b'#'; n * 2 + 1];
        let mut subs = vec![(0, 0); n];
        Solution::dfs(0, &ch, s.as_bytes(), &mut traversed, &mut count, &mut subs);
        let d = Solution::manacher_odd(&traversed);
        for i in 0..n {
            let (l, r) = subs[i];
            let len = r - l + 1;
            answer[i] = d[(l + r + 1) as usize] > len;
        }
        answer
    }

    fn dfs(i: i32, ch: &[Vec<i32>], s: &[u8], traversed: &mut Vec<u8>, count: &mut i32, subs: &mut Vec<(i32, i32)>) {
        let l = *count;
        for j in &ch[i as usize] {
            Solution::dfs(*j, ch, s, traversed, count, subs);
        }
        traversed[*count as usize * 2 + 1] = s[i as usize];
        subs[i as usize] = (l, *count);
        *count += 1;
    }

    fn manacher_odd(s0: &[u8]) -> Vec<i32> {
        let n = s0.len();
        let mut s = vec![b'$'];
        s.extend_from_slice(s0);
        s.push(b'^');
        let mut p = vec![0; n + 2];
        let mut l = 1;
        let mut r = 1;
        for i in 1..=n {
            p[i] = 0.max((r - i) as i32).min(p[l + r - i]);
            while s[i - p[i] as usize] == s[i + p[i] as usize] {
                p[i] += 1;
            }
            if i + p[i] as usize > r {
                l = i - p[i] as usize;
                r = i + p[i] as usize;
            }
        }
        p[1..n + 1].to_vec()
    }
}

#[test]
fn test() {
    let parent = vec![-1, 0, 0, 1, 1, 2];
    let s = "aababa".to_string();
    let output = vec![true, true, false, true, true, true];
    assert_eq!(Solution::find_answer(parent, s), output);

    let parent = vec![-1, 0, 0, 0, 0];
    let s = "aabcb".to_string();
    let output = vec![true, true, true, true, true];
    assert_eq!(Solution::find_answer(parent, s), output);
}
