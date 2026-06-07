// ─── leetcode-local-prelude — tự thêm để biên dịch ở local; phần này KHÔNG được gửi khi Submit.
// Đừng bỏ comment ListNode/TreeNode trong phần lời giải — chúng đã được khai báo ở đây.
struct Solution;

/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut root_number: i32 = x;
        let mut reserved_number: i32 = 0;

        while root_number > reserved_number {
            reserved_number = reserved_number * 10 + (root_number % 10);
            root_number /= 10;
        }

        reserved_number == root_number || root_number == (reserved_number / 10)
    }
}
// @lc code=end
