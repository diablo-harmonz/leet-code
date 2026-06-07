// ─── leetcode-local-prelude — tự thêm để biên dịch ở local; phần này KHÔNG được gửi khi Submit.
// Đừng bỏ comment ListNode/TreeNode trong phần lời giải — chúng đã được khai báo ở đây.
struct Solution;

/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    fn is_closing_bracket(sign: char) -> bool {
        match sign {
            '}' => true,
            ']' => true,
            ')' => true,
            _ => false,
        }
    }

    fn is_matching_bracket(sign: char) -> Option<char> {
        match sign {
            '}' => Some('{'),
            ']' => Some('['),
            ')' => Some('('),
            _ => None,
        }
    }

    pub fn is_valid(s: String) -> bool {
        if Self::is_closing_bracket(s.chars().next().unwrap()) {
            return false;
        }

        let mut seen_book: Vec<char> = Vec::new();
        for sign in s.chars() {
            if !Self::is_closing_bracket(sign) {
                seen_book.push(sign);
            }

            if Self::is_closing_bracket(sign) {
                if Self::is_matching_bracket(sign).as_ref() == seen_book.last() {
                    seen_book.pop();
                } else {
                    return false;
                }
            }
        }
        seen_book.is_empty()
    }
}
// @lc code=end

// Test local — dưới @lc code=end nên KHÔNG bị Submit.
// Chạy: cargo test --test p13_roman_to_integer -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_chars() {
        let s = String::from("()");
        let result = Solution::is_valid(s);
        assert_eq!(result, true);
    }
}
