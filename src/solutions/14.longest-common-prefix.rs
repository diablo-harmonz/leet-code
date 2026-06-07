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
        let mut first_word_prefix: String = strs[0].to_string();

        for word in 1..strs.len() {
            if first_word_prefix.is_empty() {
                return String::new();
            }

            while !strs[word].starts_with(&first_word_prefix) {
                println!(
                    "word: {} | prefix: {} | không khớp, pop...",
                    strs[word], first_word_prefix
                );
                first_word_prefix.pop();
            }
            println!(
                "word: {} | prefix: {} | khớp!",
                strs[word], first_word_prefix
            );
        }

        first_word_prefix
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
        let s = vec!["flower".to_owned(), "flow".to_owned(), "flight".to_owned()];
        let result = Solution::longest_common_prefix(s);
        // let chars: Vec<char> = s.chars().collect();
        // println!("input = {:?}", s);
        // println!("chars = {:?}", chars);
        // let result = Solution::roman_to_int(s);
        // println!("roman_to_int = {}", result);
        // assert_eq!(result, 1994);
    }
}
