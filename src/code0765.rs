#![allow(dead_code)]

// 765. Couples Holding Hands
// https://leetcode.com/problems/couples-holding-hands/
//
// There are n couples sitting in 2n seats arranged in a row and want to hold hands.
//
// The people and seats are represented by an integer array row where row[i] is the ID of the person sitting in the ith seat. The couples are numbered in order,
// the first couple being (0, 1), the second couple being (2, 3), and so on with the last couple being (2n - 2, 2n - 1).
//
// Return the minimum number of swaps so that every couple is sitting side by side. A swap consists of choosing any two people, then they stand up and switch seats.
//
// Example 1:
//
// Input: row = [0,2,1,3]
// Output: 1
// Explanation: We only need to swap the second (row[1]) and third (row[2]) person.
//
// Example 2:
//
// Input: row = [3,2,0,1]
// Output: 0
// Explanation: All couples are already seated side by side.
//
// Constraints:
//
// - 2n == row.length
// - 2 <= n <= 30
// - n is even.
// - 0 <= row[i] < 2n
// - All the elements of row are unique.
//

struct Solution;

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        fn couple(x: usize) -> usize {
            if x & 1 == 1 {
                x - 1
            } else {
                x + 1
            }
        }

        fn _min_swaps_couples(row: Vec<i32>) -> Option<i32> {
            let mut row = row.iter().map(|x| *x as usize).collect::<Vec<usize>>();
            let mut index = std::collections::HashMap::new();
            let m = row.len();
            for (i, &item) in row.iter().enumerate().take(m) {
                index.insert(item, i);
            }
            let mut swaps = 0;
            for i in (0..m).step_by(2) {
                let desired_partner = couple(row[i + 1]);
                while row[i] != desired_partner {
                    let couple_index = couple(*index.get(&couple(row[i]))?);
                    row.swap(i, couple_index);
                    swaps += 1;
                }
            }

            Some(swaps)
        }
        
        _min_swaps_couples(row).unwrap_or_default()
    }
}

#[test]
fn test() {
    let row = vec![0, 2, 1, 3];
    assert_eq!(Solution::min_swaps_couples(row), 1);

    let row = vec![3, 2, 0, 1];
    assert_eq!(Solution::min_swaps_couples(row), 0);

    let row = vec![0, 2, 4, 6, 7, 1, 3, 5];
    assert_eq!(Solution::min_swaps_couples(row), 3);
}
