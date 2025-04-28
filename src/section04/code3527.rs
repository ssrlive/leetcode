#![allow(dead_code)]

// 3527. Find the Most Common Response
// https://leetcode.com/problems/find-the-most-common-response/
// https://leetcode.cn/problems/find-the-most-common-response/
//
// Medium
//
// You are given a 2D string array responses where each responses[i] is an array of strings representing survey responses from the ith day.
//
// Return the most common response across all days after removing duplicate responses within each responses[i]. If there is a tie, return the response.
//
// Example 1:
//
// Input: responses = [["good","ok","good","ok"],["ok","bad","good","ok","ok"],["good"],["bad"]]
//
// Output: "good"
//
// Explanation:
//
//     After removing duplicates within each list, responses = [["good", "ok"], ["ok", "bad", "good"], ["good"], ["bad"]].
//     "good" appears 3 times, "ok" appears 2 times, and "bad" appears 2 times.
//     Return "good" because it has the highest frequency.
//
// Example 2:
//
// Input: responses = [["good","ok","good"],["ok","bad"],["bad","notsure"],["great","good"]]
//
// Output: "bad"
//
// Explanation:
//
//     After removing duplicates within each list we have responses = [["good", "ok"], ["ok", "bad"], ["bad", "notsure"], ["great", "good"]].
//     "bad", "good", and "ok" each occur 2 times.
//     The output is "bad" because it is the lexicographically smallest amongst the words with the highest frequency.
//
// Constraints:
//
//     1 <= responses.length <= 1000
//     1 <= responses[i].length <= 1000
//     1 <= responses[i][j].length <= 10
//     responses[i][j] consists of only lowercase English letters
//

struct Solution;

impl Solution {
    pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
        let mut res = String::new();
        let mut freq = 0;
        let mut map = std::collections::HashMap::new();
        for response in responses {
            let mut s = std::collections::HashSet::new();
            for r in response {
                if s.insert(r.clone()) {
                    let count = map.entry(r.clone()).or_insert(0);
                    *count += 1;
                    if *count > freq {
                        freq = *count;
                        res = r;
                    } else if *count == freq && r < res {
                        res = r;
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let responses = vec![
        vec!["good".to_string(), "ok".to_string(), "good".to_string(), "ok".to_string()],
        vec![
            "ok".to_string(),
            "bad".to_string(),
            "good".to_string(),
            "ok".to_string(),
            "ok".to_string(),
        ],
        vec!["good".to_string()],
        vec!["bad".to_string()],
    ];
    assert_eq!(Solution::find_common_response(responses), "good");

    let responses = vec![
        vec!["good".to_string(), "ok".to_string(), "good".to_string()],
        vec!["ok".to_string(), "bad".to_string()],
        vec!["bad".to_string(), "notsure".to_string()],
        vec!["great".to_string(), "good".to_string()],
    ];
    assert_eq!(Solution::find_common_response(responses), "bad");
}
