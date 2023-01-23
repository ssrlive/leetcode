#![allow(dead_code)]

// 1298. Maximum Candies You Can Get from Boxes
// https://leetcode.com/problems/maximum-candies-you-can-get-from-boxes/
// https://leetcode.cn/problems/maximum-candies-you-can-get-from-boxes/
//
// Hard
//
// You have n boxes labeled from 0 to n - 1. You are given four arrays: status, candies, keys, and containedBoxes where:
//
//     status[i] is 1 if the ith box is open and 0 if the ith box is closed,
//     candies[i] is the number of candies in the ith box,
//     keys[i] is a list of the labels of the boxes you can open after opening the ith box.
//     containedBoxes[i] is a list of the boxes you found inside the ith box.
//
// You are given an integer array initialBoxes that contains the labels of the boxes you initially have.
// You can take all the candies in any open box and you can use the keys in it
// to open new boxes and you also can use the boxes you find in it.
//
// Return the maximum number of candies you can get following the rules above.
//
// Example 1:
//
// Input: status = [1,0,1,0], candies = [7,5,4,100], keys = [[],[],[1],[]], containedBoxes = [[1,2],[3],[],[]], initialBoxes = [0]
// Output: 16
// Explanation: You will be initially given box 0. You will find 7 candies in it and boxes 1 and 2.
// Box 1 is closed and you do not have a key for it so you will open box 2. You will find 4 candies and a key to box 1 in box 2.
// In box 1, you will find 5 candies and box 3 but you will not find a key to box 3 so box 3 will remain closed.
// Total number of candies collected = 7 + 4 + 5 = 16 candy.
//
// Example 2:
//
// Input: status = [1,0,0,0,0,0], candies = [1,1,1,1,1,1], keys = [[1,2,3,4,5],[],[],[],[],[]],
// containedBoxes = [[1,2,3,4,5],[],[],[],[],[]], initialBoxes = [0]
// Output: 6
// Explanation: You have initially box 0. Opening it you can find boxes 1,2,3,4 and 5 and their keys.
// The total number of candies will be 6.
//
// Constraints:
//
// -    n == status.length == candies.length == keys.length == containedBoxes.length
// -    1 <= n <= 1000
// -    status[i] is either 0 or 1.
// -    1 <= candies[i] <= 1000
// -    0 <= keys[i].length <= n
// -    0 <= keys[i][j] < n
// -    All values of keys[i] are unique.
// -    0 <= containedBoxes[i].length <= n
// -    0 <= containedBoxes[i][j] < n
// -    All values of containedBoxes[i] are unique.
// -    Each box is contained in one box at most.
// -    0 <= initialBoxes.length <= n
// -    0 <= initialBoxes[i] < n
//

struct Solution;

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut status = status;
        let mut q = std::collections::VecDeque::new();
        let mut result = 0;
        let mut reachable_closed_boxes = vec![false; status.len()];
        for i in initial_boxes {
            if status[i as usize] == 1 {
                q.push_back(i);
            } else {
                reachable_closed_boxes[i as usize] = true;
            }
        }
        while !q.is_empty() {
            let i = q.pop_front().unwrap();
            result += candies[i as usize];
            for &j in &keys[i as usize] {
                if status[j as usize] == 0 && reachable_closed_boxes[j as usize] {
                    q.push_back(j);
                }
                status[j as usize] = 1;
            }
            for &j in &contained_boxes[i as usize] {
                if status[j as usize] == 1 {
                    q.push_back(j);
                } else {
                    reachable_closed_boxes[j as usize] = true;
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![1, 0, 1, 0],
            vec![7, 5, 4, 100],
            vec![vec![], vec![], vec![1], vec![]],
            vec![vec![1, 2], vec![3], vec![], vec![]],
            vec![0],
            16,
        ),
        (
            vec![1, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1],
            vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
            vec![vec![1, 2, 3, 4, 5], vec![], vec![], vec![], vec![], vec![]],
            vec![0],
            6,
        ),
    ];
    for (status, candies, keys, contained_boxes, initial_boxes, expect) in cases {
        assert_eq!(
            Solution::max_candies(status, candies, keys, contained_boxes, initial_boxes),
            expect
        );
    }
}
