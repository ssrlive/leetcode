#![allow(dead_code)]

// 401. Binary Watch
// https://leetcode.com/problems/binary-watch/
//
// A binary watch has 4 LEDs on the top to represent the hours (0-11), and 6 LEDs on the bottom to represent the minutes (0-59).
// Each LED represents a zero or one, with the least significant bit on the right.
//
// For example, the below binary watch reads "4:51".
//
// Given an integer turnedOn which represents the number of LEDs that are currently on (ignoring the PM),
// return all possible times the watch could represent. You may return the answer in any order.
//
// The hour must not contain a leading zero.
//
// For example, "01:00" is not valid. It should be "1:00".
// The minute must be consist of two digits and may contain a leading zero.
//
// For example, "10:2" is not valid. It should be "10:02".
//
// Example 1:
//
// Input: turnedOn = 1
// Output: ["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]
//
// Example 2:
//
// Input: turnedOn = 9
// Output: []
//
// Constraints:
//
// - 0 <= turnedOn <= 10
//

struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let nu = turned_on as u32;
        let mut result = Vec::new();
        for i in 0i32..12 {
            for j in 0i32..60 {
                if (i.count_ones() + j.count_ones()) == nu {
                    result.push(format!("{}:{:02}", i, j));
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let result = Solution::read_binary_watch(1);
    assert_eq!(result.len(), 10);
    assert!(result.contains(&"0:01".to_string()));
    assert!(result.contains(&"0:02".to_string()));
    assert!(result.contains(&"0:04".to_string()));
    assert!(result.contains(&"0:08".to_string()));
    assert!(result.contains(&"0:16".to_string()));
    assert!(result.contains(&"0:32".to_string()));
    assert!(result.contains(&"1:00".to_string()));
    assert!(result.contains(&"2:00".to_string()));
    assert!(result.contains(&"4:00".to_string()));
    assert!(result.contains(&"8:00".to_string()));

    let result = Solution::read_binary_watch(2);
    assert_eq!(result.len(), 44);
    println!("{:?}", result);
    assert!(result.contains(&"0:03".to_string()));
    assert!(result.contains(&"0:05".to_string()));
    assert!(result.contains(&"0:06".to_string()));
    assert!(result.contains(&"0:09".to_string()));
    assert!(result.contains(&"0:10".to_string()));
    assert!(result.contains(&"0:12".to_string()));
    assert!(result.contains(&"0:17".to_string()));
    assert!(result.contains(&"0:48".to_string()));
    assert!(result.contains(&"2:01".to_string()));
    assert!(result.contains(&"8:01".to_string()));
}
