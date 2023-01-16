#![allow(dead_code)]

// 2468. Split Message Based on Limit
// https://leetcode.com/problems/split-message-based-on-limit/
// https://leetcode.cn/problems/split-message-based-on-limit/
//
// You are given a string, message, and a positive integer, limit.
//
// You must split message into one or more parts based on limit. Each resulting part should have the suffix "<a/b>",
// where "b" is to be replaced with the total number of parts and "a" is to be replaced with the index of the part,
// starting from 1 and going up to b. Additionally, the length of each resulting part (including its suffix)
// should be equal to limit, except for the last part whose length can be at most limit.
//
// The resulting parts should be formed such that when their suffixes are removed and they are all concatenated in order,
// they should be equal to message. Also, the result should contain as few parts as possible.
//
// Return the parts message would be split into as an array of strings. If it is impossible to split message as required, return an empty array.
//
// Example 1:
//
// Input: message = "this is really a very awesome message", limit = 9
// Output: ["thi<1/14>","s i<2/14>","s r<3/14>","eal<4/14>","ly <5/14>","a v<6/14>","ery<7/14>"," aw<8/14>","eso<9/14>","me<10/14>"," m<11/14>","es<12/14>","sa<13/14>","ge<14/14>"]
// Explanation:
// The first 9 parts take 3 characters each from the beginning of message.
// The next 5 parts take 2 characters each to finish splitting message.
// In this example, each part, including the last, has length 9.
// It can be shown it is not possible to split message into less than 14 parts.
//
// Example 2:
//
// Input: message = "short message", limit = 15
// Output: ["short mess<1/2>","age<2/2>"]
// Explanation:
// Under the given constraints, the string can be split into two parts:
// - The first part comprises of the first 10 characters, and has a length 15.
// - The next part comprises of the last 3 characters, and has a length 8.
//
// Constraints:
//
// - 1 <= message.length <= 10^4
// - message consists only of lowercase English letters and ' '.
// - 1 <= limit <= 10^4
//

struct Solution;

impl Solution {
    pub fn split_message(message: String, limit: i32) -> Vec<String> {
        let len = message.len();
        if limit <= 5 {
            return Vec::new();
        }

        let mut msglen = (limit - 5) as usize;
        let mut splits = Vec::<String>::new();
        let mut pow10 = 10;
        let mut pow = 1;
        let mut pow10local = 10;
        let mut powlocal = 1;
        let mut nmsg = 0;
        let mut stridx = 0;
        loop {
            nmsg += 1;
            if nmsg == pow10 {
                pow10 *= 10;
                pow += 1;
                pow10local = 10;
                powlocal = 1;
                nmsg = 0;
                stridx = 0;
                if limit <= (3 + pow + powlocal) {
                    return Vec::new();
                }
                msglen = limit as usize - (3 + pow + powlocal) as usize;
                splits.clear();
                continue;
            }

            if nmsg == pow10local {
                pow10local *= 10;
                powlocal += 1;
                if limit <= (3 + pow + powlocal) {
                    return Vec::new();
                }
                msglen = limit as usize - (3 + pow + powlocal) as usize;
            }

            if msglen == 0 {
                return Vec::new();
            }

            let mut finish = false;
            if (msglen + stridx) >= len {
                finish = true;
                if limit <= (3 + pow + powlocal) {
                    return Vec::new();
                }
                msglen = len - stridx;
            }
            splits.push(String::from(&message[stridx..(stridx + msglen)]));
            stridx += msglen;
            if finish {
                break;
            }
        }
        let f = |x: (usize, &String)| format!("{}<{}/{}>", x.1, x.0 + 1, nmsg);
        splits.iter().enumerate().map(f).collect::<Vec<String>>()
    }
}

#[test]
fn test() {
    let message = "this is really a very awesome message".to_string();
    let expected = vec![
        "thi<1/14>",
        "s i<2/14>",
        "s r<3/14>",
        "eal<4/14>",
        "ly <5/14>",
        "a v<6/14>",
        "ery<7/14>",
        " aw<8/14>",
        "eso<9/14>",
        "me<10/14>",
        " m<11/14>",
        "es<12/14>",
        "sa<13/14>",
        "ge<14/14>",
    ];
    assert_eq!(Solution::split_message(message, 9), expected);

    let message = "short message".to_string();
    let expected = vec!["short mess<1/2>", "age<2/2>"];
    assert_eq!(Solution::split_message(message, 15), expected);
}
