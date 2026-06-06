// ─── leetcode-local-prelude — tự thêm để biên dịch ở local; phần này KHÔNG được gửi khi Submit.
// Đừng bỏ comment ListNode/TreeNode trong phần lời giải — chúng đã được khai báo ở đây.
    struct Solution;

/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

 // @lc code=start
use std::collections::HashMap;
 impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen_book = HashMap::new();
        for (index, num_found) in nums.iter().enumerate() {
            let complement = target - num_found;
            if let Some(&previous_num) = seen_book.get(&complement) {
                return vec![previous_num, index as i32]
            }
            seen_book.insert(num_found, index as i32);
        }
        vec![]
    }
}
// @lc code=end

// Test local — nằm dưới @lc code=end nên KHÔNG bị Submit.
// Chạy tất cả:   cargo test -- --nocapture
// Chỉ bài này:   cargo test p1_two_sum -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let out = Solution::two_sum(vec![2, 7, 11, 15], 9);
        println!("case1 -> {:?}", out);
        assert_eq!(out, vec![0, 1]);
    }

    #[test]
    fn case2() {
        let out = Solution::two_sum(vec![3, 2, 4], 6);
        println!("case2 -> {:?}", out);
        assert_eq!(out, vec![1, 2]);
    }
}
