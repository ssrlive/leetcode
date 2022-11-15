#![allow(dead_code)]

// 187. Repeated DNA Sequences
// https://leetcode.com/problems/repeated-dna-sequences/
//
// The DNA sequence is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T'.
// - For example, "ACGAATTCCG" is a DNA sequence.
//
// When studying DNA, it is useful to identify repeated sequences within the DNA.
//
// Given a string s that represents a DNA sequence,
// return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule.
// You may return the answer in any order.
//

struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut sequences = std::collections::HashMap::new();
        s.as_bytes().windows(10).for_each(|seq| {
            sequences.entry(seq).and_modify(|c| *c += 1).or_insert(1u32);
        });

        sequences
            .into_iter()
            .filter(|&(_seq, count)| count > 1)
            .map(|(seq, _count)| seq)
            .map(|seq| std::str::from_utf8(seq).unwrap())
            .map(|seq| seq.to_owned())
            .collect()
    }
}

#[test]
fn test_find_repeated_dna_sequences() {
    let mut l = Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_owned());
    l.sort();
    let expected = vec!["AAAAACCCCC", "CCCCCAAAAA"];
    assert_eq!(l, expected);
    assert_eq!(
        Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string()),
        vec!["AAAAAAAAAA".to_string()]
    );
}
