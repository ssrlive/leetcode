#![allow(dead_code)]

// 1054. Distant Barcodes
// https://leetcode.com/problems/distant-barcodes/
// https://leetcode.cn/problems/distant-barcodes/
//
// In a warehouse, there is a row of barcodes, where the ith barcode is barcodes[i].
//
// Rearrange the barcodes so that no two adjacent barcodes are equal. You may return any answer, and it is guaranteed an answer exists.
//
// Example 1:
//
// Input: barcodes = [1,1,1,2,2,2]
// Output: [2,1,2,1,2,1]
//
// Example 2:
//
// Input: barcodes = [1,1,1,1,2,2,3,3]
// Output: [1,3,1,3,1,2,1,2]
//
// Constraints:
//
// - 1 <= barcodes.length <= 10000
// - 1 <= barcodes[i] <= 10000
//

struct Solution;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut barcodes = barcodes;
        let mut pos = 0;
        let mut m = std::collections::HashMap::new();
        let mut s = std::collections::BTreeSet::new();
        for n in barcodes.iter() {
            *m.entry(*n).or_insert(0) += 1;
        }
        for (k, v) in m.iter() {
            s.insert((v, k));
        }
        for &(&v, &k) in s.iter().rev() {
            for _ in 0..v {
                if pos >= barcodes.len() {
                    pos = 1;
                }
                barcodes[pos] = k;
                pos += 2;
            }
        }
        barcodes
    }
}

#[test]
fn test() {
    let barcodes = vec![1, 1, 1, 2, 2, 2];
    let res = vec![2, 1, 2, 1, 2, 1];
    assert_eq!(Solution::rearrange_barcodes(barcodes), res);

    let barcodes = vec![1, 1, 1, 1, 2, 2, 3, 3];
    let res = vec![1, 3, 1, 3, 1, 2, 1, 2];
    assert_eq!(Solution::rearrange_barcodes(barcodes), res);
}
