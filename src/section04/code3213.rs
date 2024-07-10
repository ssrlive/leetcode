#![allow(dead_code)]

// 3213. Construct String with Minimum Cost
// https://leetcode.com/problems/construct-string-with-minimum-cost/
// https://leetcode.cn/problems/construct-string-with-minimum-cost/
//
// Hard
//
// You are given a string target, an array of strings words, and an integer array costs, both arrays of the same length.
//
// Imagine an empty string s.
//
// You can perform the following operation any number of times (including zero):
//
// - Choose an index i in the range [0, words.length - 1].
// - Append words[i] to s.
// - The cost of operation is costs[i].
//
// Return the minimum cost to make s equal to target. If it's not possible, return -1.
//
// Example 1:
//
// Input: target = "abcdef", words = ["abdef","abc","d","def","ef"], costs = [100,1,1,10,5]
//
// Output: 7
//
// Explanation:
//
// The minimum cost can be achieved by performing the following operations:
//
// - Select index 1 and append "abc" to s at a cost of 1, resulting in s = "abc".
// - Select index 2 and append "d" to s at a cost of 1, resulting in s = "abcd".
// - Select index 4 and append "ef" to s at a cost of 5, resulting in s = "abcdef".
//
// Example 2:
//
// Input: target = "aaaa", words = ["z","zz","zzz"], costs = [1,10,100]
//
// Output: -1
//
// Explanation:
//
// It is impossible to make s equal to target, so we return -1.
//
// Constraints:
//
//     1 <= target.length <= 5 * 10^4
//     1 <= words.length == costs.length <= 5 * 10^4
//     1 <= words[i].length <= target.length
//     The total sum of words[i].length is less than or equal to 5 * 10^4.
//     target and words[i] consist only of lowercase English letters.
//     1 <= costs[i] <= 10^4
//

struct Solution;

#[derive(Default, Debug)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    cost: Option<i32>,
}

impl Solution {
    pub fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
        let mut root = Trie::default();

        for (i, word) in words.into_iter().enumerate() {
            let mut curr = &mut root;

            for c in word.chars() {
                let c = c as usize - 'a' as usize;
                curr = curr.children[c].get_or_insert_with(Default::default);
            }

            if let Some(cost) = curr.cost {
                curr.cost = Some(costs[i].min(cost));
            } else {
                curr.cost = Some(costs[i]);
            }
        }

        fn dfs(t: &Vec<usize>, root: &Trie, i: usize, dp: &mut Vec<i32>) -> Option<i32> {
            if i == t.len() {
                return Some(0);
            }

            if i > t.len() {
                return None;
            }

            if dp[i] != -1 {
                return Some(dp[i]);
            }

            let mut res = i32::MAX;
            let mut curr = root;
            let mut traversed = 0;

            for j in i..t.len() {
                let Some(next) = curr.children[t[j]].as_ref() else {
                    break;
                };

                traversed += 1;

                if let Some(cost) = next.cost.as_ref() {
                    if let Some(total) = dfs(t, root, i + traversed, dp) {
                        res = res.min(total + cost);
                    }
                }

                curr = next;
            }

            if res == i32::MAX {
                return None;
            }

            dp[i] = res;
            Some(res)
        }

        let t: Vec<usize> = target.chars().map(|c| c as usize - 'a' as usize).collect();
        let mut dp: Vec<i32> = vec![-1; t.len()];

        dfs(&t, &root, 0, &mut dp).unwrap_or(-1)
    }
}

#[test]
fn test() {
    let target = "abcdef".to_string();
    let words = vec![
        "abdef".to_string(),
        "abc".to_string(),
        "d".to_string(),
        "def".to_string(),
        "ef".to_string(),
    ];
    let costs = vec![100, 1, 1, 10, 5];
    let res = 7;
    assert_eq!(Solution::minimum_cost(target, words, costs), res);

    let target = "aaaa".to_string();
    let words = vec!["z".to_string(), "zz".to_string(), "zzz".to_string()];
    let costs = vec![1, 10, 100];
    let res = -1;
    assert_eq!(Solution::minimum_cost(target, words, costs), res);
}
