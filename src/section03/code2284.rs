#![allow(dead_code)]

/*

// 2284. Sender With Largest Word Count
// https://leetcode.com/problems/sender-with-largest-word-count/
// https://leetcode.cn/problems/sender-with-largest-word-count/
//
// Medium
//
// You have a chat log of n messages. You are given two string arrays messages and senders where messages[i] is a message sent by senders[i].

A message is list of words that are separated by a single space with no leading or trailing spaces. The word count of a sender is the total number of words sent by the sender. Note that a sender may send more than one message.

Return the sender with the largest word count. If there is more than one sender with the largest word count, return the one with the lexicographically largest name.

Note:

    Uppercase letters come before lowercase letters in lexicographical order.
    "Alice" and "alice" are distinct.

Example 1:

Input: messages = ["Hello userTwooo","Hi userThree","Wonderful day Alice","Nice day userThree"], senders = ["Alice","userTwo","userThree","Alice"]
Output: "Alice"
Explanation: Alice sends a total of 2 + 3 = 5 words.
userTwo sends a total of 2 words.
userThree sends a total of 3 words.
Since Alice has the largest word count, we return "Alice".

Example 2:

Input: messages = ["How is leetcode for everyone","Leetcode is useful for practice"], senders = ["Bob","Charlie"]
Output: "Charlie"
Explanation: Bob sends a total of 5 words.
Charlie sends a total of 5 words.
Since there is a tie for the largest word count, we return the sender with the lexicographically larger name, Charlie.

Constraints:

    n == messages.length == senders.length
    1 <= n <= 10^4
    1 <= messages[i].length <= 100
    1 <= senders[i].length <= 10
    messages[i] consists of uppercase and lowercase English letters and ' '.
    All the words in messages[i] are separated by a single space.
    messages[i] does not have leading or trailing spaces.
    senders[i] consists of uppercase and lowercase English letters only.
*/

struct Solution;

impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        use std::collections::HashMap;
        let mut xs = HashMap::new();
        let mut hi = 0;
        let mut ans = String::new();
        for (i, x) in senders.into_iter().enumerate() {
            let cnt = messages[i].split(' ').collect::<Vec<&str>>().len();
            *xs.entry(x.clone()).or_insert(0) += cnt;
            if xs[&x] > hi || (xs[&x] == hi && x > ans) {
                hi = xs[&x];
                ans = x;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![
                "Hello userTwooo",
                "Hi userThree",
                "Wonderful day Alice",
                "Nice day userThree",
            ],
            vec!["Alice", "userTwo", "userThree", "Alice"],
            "Alice",
        ),
        (
            vec!["How is leetcode for everyone", "Leetcode is useful for practice"],
            vec!["Bob", "Charlie"],
            "Charlie",
        ),
    ];
    for (messages, senders, expect) in cases {
        let messages = messages.into_iter().map(|x| x.to_string()).collect();
        let senders = senders.into_iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::largest_word_count(messages, senders), expect);
    }
}
