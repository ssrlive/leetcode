#![allow(dead_code)]

// 937. Reorder Data in Log Files
// https://leetcode.com/problems/reorder-data-in-log-files/
// https://leetcode.cn/problems/reorder-data-in-log-files/
//
// You are given an array of logs. Each log is a space-delimited string of words, where the first word is the identifier.
//
// There are two types of logs:
//
// Letter-logs: All words (except the identifier) consist of lowercase English letters.
// Digit-logs: All words (except the identifier) consist of digits.
// Reorder these logs so that:
//
// The letter-logs come before all digit-logs.
// The letter-logs are sorted lexicographically by their contents. If their contents are the same, then sort them lexicographically by their identifiers.
// The digit-logs maintain their relative ordering.
// Return the final order of the logs.
//
// Example 1:
//
// Input: logs = ["dig1 8 1 5 1","let1 art can","dig2 3 6","let2 own kit dig","let3 art zero"]
// Output: ["let1 art can","let3 art zero","let2 own kit dig","dig1 8 1 5 1","dig2 3 6"]
// Explanation:
// The letter-log contents are all different, so their ordering is "art can", "art zero", "own kit dig".
// The digit-logs have a relative order of "dig1 8 1 5 1", "dig2 3 6".
//
// Example 2:
//
// Input: logs = ["a1 9 2 3 1","g1 act car","zo4 4 7","ab1 off key dog","a8 act zoo"]
// Output: ["g1 act car","a8 act zoo","ab1 off key dog","a1 9 2 3 1","zo4 4 7"]
//
// Constraints:
//
// - 1 <= logs.length <= 100
// - 3 <= logs[i].length <= 100
// - All the tokens of logs[i] are separated by a single space.
// - logs[i] is guaranteed to have an identifier and at least one word after the identifier.
//

struct Solution;

impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        pub struct DataSort<'a> {
            pub flag_let_dig: usize,
            pub id: &'a str,
            pub content: &'a str,
            pub s: &'a String,
        }

        let mut logs_sort = logs
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let (id, content) = s.split_once(' ').unwrap();

                let flag_let_dig = match content.chars().next() {
                    Some(c) if c.is_alphabetic() => 0,
                    _ => i + 1,
                };

                DataSort {
                    flag_let_dig,
                    content,
                    id,
                    s,
                }
            })
            .collect::<Vec<_>>();
        logs_sort.sort_unstable_by(|a, b| {
            a.flag_let_dig
                .cmp(&b.flag_let_dig)
                .then(a.content.cmp(b.content))
                .then(a.id.cmp(b.id))
        });
        logs_sort.into_iter().map(|data| data.s.clone()).collect()
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!["dig1 8 1 5 1", "let1 art can", "dig2 3 6", "let2 own kit dig", "let3 art zero"],
            vec!["let1 art can", "let3 art zero", "let2 own kit dig", "dig1 8 1 5 1", "dig2 3 6"],
        ),
        (
            vec!["a1 9 2 3 1", "g1 act car", "zo4 4 7", "ab1 off key dog", "a8 act zoo"],
            vec!["g1 act car", "a8 act zoo", "ab1 off key dog", "a1 9 2 3 1", "zo4 4 7"],
        ),
    ];
    let cases = cases
        .into_iter()
        .map(|(logs, expected)| {
            (
                logs.into_iter().map(|s| s.to_string()).collect(),
                expected.into_iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            )
        })
        .collect::<Vec<_>>();

    for (logs, expected) in cases {
        assert_eq!(Solution::reorder_log_files(logs), expected);
    }
}
