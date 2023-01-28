#![allow(dead_code)]

// 1386. Cinema Seat Allocation
// https://leetcode.com/problems/cinema-seat-allocation/
// https://leetcode.cn/problems/cinema-seat-allocation/
//
// Medium
//
// A cinema has n rows of seats, numbered from 1 to n and there are ten seats in each row, labelled from 1 to 10 as shown in the figure above.
//
// Given the array reservedSeats containing the numbers of seats already reserved, for example,
// reservedSeats[i] = [3,8] means the seat located in row 3 and labelled with 8 is already reserved.
//
// Return the maximum number of four-person groups you can assign on the cinema seats.
// A four-person group occupies four adjacent seats in one single row. Seats across an aisle (such as [3,3] and [3,4])
// are not considered to be adjacent, but there is an exceptional case on which an aisle split a four-person group,
// in that case, the aisle split a four-person group in the middle, which means to have two people on each side.
//
// Example 1:
//
// Input: n = 3, reservedSeats = [[1,2],[1,3],[1,8],[2,6],[3,1],[3,10]]
// Output: 4
// Explanation: The figure above shows the optimal allocation for four groups, where seats mark with blue are already
// reserved and contiguous seats mark with orange are for one group.
//
// Example 2:
//
// Input: n = 2, reservedSeats = [[2,1],[1,8],[2,6]]
// Output: 2
//
// Example 3:
//
// Input: n = 4, reservedSeats = [[4,3],[1,4],[4,6],[1,7]]
// Output: 4
//
// Constraints:
//
// -    1 <= n <= 10^9
// -    1 <= reservedSeats.length <= min(10*n, 10^4)
// -    reservedSeats[i].length == 2
// -    1 <= reservedSeats[i][0] <= n
// -    1 <= reservedSeats[i][1] <= 10
// -    All reservedSeats[i] are distinct.
//

struct Solution;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        use std::collections::*;
        fn helper(a: &str) -> i32 {
            let mut result = 0;
            let a = a.chars().collect::<Vec<char>>();
            for (i, &item) in a.iter().enumerate() {
                if item == '1' {
                    result += 1 << i;
                }
            }
            result
        }

        let mut result = 0;
        let mut map = HashMap::new();
        for arr in reserved_seats {
            map.entry(arr[0]).or_insert(vec![]).push(arr[1]);
        }

        let two = helper("0111111110");
        let ones = vec![helper("0001111000"), helper("0111100000"), helper("0000011110")];
        let mut arr = map.into_iter().collect::<Vec<(i32, Vec<i32>)>>();
        arr.sort();
        let mut last = 0;
        let base = (1 << 10) - 1;
        for (i, indexes) in arr {
            result += (i - last - 1) * 2;

            let mut temp = base;
            for j in indexes {
                temp ^= 1 << (j - 1);
            }

            if temp & two == two {
                result += 2;
            } else {
                for &v in &ones {
                    if temp & v == v {
                        result += 1;
                        break;
                    }
                }
            }

            last = i;
        }
        result += (n - last) * 2;
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            3,
            vec![vec![1, 2], vec![1, 3], vec![1, 8], vec![2, 6], vec![3, 1], vec![3, 10]],
            4,
        ),
        (2, vec![vec![2, 1], vec![1, 8], vec![2, 6]], 2),
        (4, vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]], 4),
    ];
    for (n, reserved_seats, expected) in cases {
        assert_eq!(Solution::max_number_of_families(n, reserved_seats), expected);
    }
}
