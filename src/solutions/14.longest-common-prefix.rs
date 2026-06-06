// ─── leetcode-local-prelude — tự thêm để biên dịch ở local; phần này KHÔNG được gửi khi Submit.
// Đừng bỏ comment ListNode/TreeNode trong phần lời giải — chúng đã được khai báo ở đây.
struct Solution;

/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].to_string();

        for word in 1..strs.len() {
            while !strs[word].starts_with(&prefix) {
                prefix.pop();

                if prefix.is_empty() {
                    return String::new()
                }
            }
        }
        prefix
    }
}
// @lc code=end
