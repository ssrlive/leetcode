#![allow(dead_code)]

// 752. Open the Lock
// https://leetcode.com/problems/open-the-lock/
//
// You have a lock in front of you with 4 circular wheels. Each wheel has 10 slots: '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'.
// The wheels can rotate freely and wrap around: for example we can turn '9' to be '0', or '0' to be '9'.
// Each move consists of turning one wheel one slot.
//
// The lock initially starts at '0000', a string representing the state of the 4 wheels.
//
// You are given a list of deadends dead ends, meaning if the lock displays any of these codes,
// the wheels of the lock will stop turning and you will be unable to open it.
//
// Given a target representing the value of the wheels that will unlock the lock,
// return the minimum total number of turns required to open the lock, or -1 if it is impossible.
//
// Example 1:
//
// Input: deadends = ["0201","0101","0102","1212","2002"], target = "0202"
// Output: 6
// Explanation:
// A sequence of valid moves would be "0000" -> "1000" -> "1100" -> "1200" -> "1201" -> "1202" -> "0202".
// Note that a sequence like "0000" -> "0001" -> "0002" -> "0102" -> "0202" would be invalid,
// because the wheels of the lock become stuck after the display becomes the dead end "0102".
//
// Example 2:
//
// Input: deadends = ["8888"], target = "0009"
// Output: 1
// Explanation: We can turn the last wheel in reverse to move from "0000" -> "0009".
//
// Example 3:
//
// Input: deadends = ["8887","8889","8878","8898","8788","8988","7888","9888"], target = "8888"
// Output: -1
// Explanation: We cannot reach the target without getting stuck.
//
// Constraints:
//
// - 1 <= deadends.length <= 500
// - deadends[i].length == 4
// - target.length == 4
// - target will not be in the list deadends.
// - target and deadends[i] consist of digits only.
//

struct Solution;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        fn _open_lock(deadends: Vec<String>, target: String) -> Option<i32> {
            let mut dead = vec![false; 10_000];
            deadends
                .iter()
                .filter_map(|d| d.parse::<usize>().ok())
                .for_each(|i| dead[i] = true);
            let target = target.parse::<usize>().ok()?;
            let mut visited = vec![false; 10_000];
            let mut vd = std::collections::VecDeque::new();
            if !dead[0] {
                vd.push_back((0, 0));
            }
            while let Some((state, moves)) = vd.pop_front() {
                if state == target {
                    return Some(moves);
                }
                for i in &[1, 10, 100, 1000] {
                    let w = (state / i) % 10;
                    for &j in &[
                        (state - i * w) + i * ((w + 1) % 10),
                        (state - i * w) + i * ((w + 9) % 10),
                    ] {
                        if !dead[j] && !visited[j] {
                            visited[j] = true;
                            vd.push_back((j, moves + 1));
                        }
                    }
                }
            }
            Some(-1)
        }

        _open_lock(deadends, target).unwrap_or(-1)
    }
}

#[test]
fn test() {
    let deadends = vec!["0201", "0101", "0102", "1212", "2002"];
    let deadends = deadends.into_iter().map(|s| s.to_string()).collect();
    let target = "0202".to_string();
    assert_eq!(Solution::open_lock(deadends, target), 6);

    let deadends = vec!["8888"].into_iter().map(|s| s.to_string()).collect();
    let target = "0009".to_string();
    assert_eq!(Solution::open_lock(deadends, target), 1);

    let deadends = vec!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"];
    let deadends = deadends.into_iter().map(|s| s.to_string()).collect();
    let target = "8888".to_string();
    assert_eq!(Solution::open_lock(deadends, target), -1);
}
