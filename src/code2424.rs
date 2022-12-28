#![allow(dead_code)]

// 2424. Longest Uploaded Prefix
// https://leetcode.com/problems/longest-uploaded-prefix/
// https://leetcode.cn/problems/longest-uploaded-prefix/
//
// You are given a stream of n videos, each represented by a distinct number from 1 to n that you need to "upload" to a server. You need to implement a data structure that calculates the length of the longest uploaded prefix at various points in the upload process.
//
// We consider i to be an uploaded prefix if all videos in the range 1 to i (inclusive) have been uploaded to the server. The longest uploaded prefix is the maximum value of i that satisfies this definition.
//
// Implement the LUPrefix class:
//
// LUPrefix(int n) Initializes the object for a stream of n videos.
// void upload(int video) Uploads video to the server.
// int longest() Returns the length of the longest uploaded prefix defined above.
//
// Example 1:
//
// Input
// ["LUPrefix", "upload", "longest", "upload", "longest", "upload", "longest"]
// [[4], [3], [], [1], [], [2], []]
// Output
// [null, null, 0, null, 1, null, 3]
//
// Explanation
// LUPrefix server = new LUPrefix(4);   // Initialize a stream of 4 videos.
// server.upload(3);                    // Upload video 3.
// server.longest();                    // Since video 1 has not been uploaded yet, there is no prefix.
//                                      // So, we return 0.
// server.upload(1);                    // Upload video 1.
// server.longest();                    // The prefix [1] is the longest uploaded prefix, so we return 1.
// server.upload(2);                    // Upload video 2.
// server.longest();                    // The prefix [1,2,3] is the longest uploaded prefix, so we return 3.
//
// Constraints:
//
// - 1 <= n <= 10^5
// - 1 <= video <= n
// - All values of video are distinct.
// - At most 2 * 105 calls in total will be made to upload and longest.
// - At least one call will be made to longest.
//

struct LUPrefix {
    n: usize,
    longest: usize,
    uploaded: Vec<bool>,
}

impl LUPrefix {
    fn new(n: i32) -> Self {
        Self {
            n: n as usize,
            longest: 0,
            uploaded: vec![false; n as usize],
        }
    }

    fn upload(&mut self, video: i32) {
        let video = video as usize;
        self.uploaded[video - 1] = true;
        if video == self.longest + 1 {
            self.longest = video;
            while self.longest < self.n && self.uploaded[self.longest] {
                self.longest += 1;
            }
        }
    }

    fn longest(&self) -> i32 {
        self.longest as i32
    }
}

#[test]
fn test() {
    let mut server = LUPrefix::new(4);
    server.upload(3);
    assert_eq!(server.longest(), 0);
    server.upload(1);
    assert_eq!(server.longest(), 1);
    server.upload(2);
    assert_eq!(server.longest(), 3);
}
