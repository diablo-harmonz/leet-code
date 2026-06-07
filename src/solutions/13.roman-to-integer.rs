// ─── leetcode-local-prelude — tự thêm để biên dịch ở local; phần này KHÔNG được gửi khi Submit.
// Đừng bỏ comment ListNode/TreeNode trong phần lời giải — chúng đã được khai báo ở đây.
struct Solution;

/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    fn roman_dictionary(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
    pub fn roman_to_int(s: String) -> i32 {
        let roman_char: Vec<char> = s.chars().collect();
        let mut total: i32 = 0;

        for i in 0..roman_char.len() {
            let current: i32 = Self::roman_dictionary(roman_char[i]);
            let next: i32 = if i + 1 < roman_char.len() {
                Self::roman_dictionary(roman_char[i + 1])
            } else {
                0
            };

            if current < next {
                total -= current;
            } else {
                total += current;
            }
        }

        total
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
        let s = String::from("MCMXCIV");
        let chars: Vec<char> = s.chars().collect();
        println!("input = {:?}", s);
        println!("chars = {:?}", chars);
        let result = Solution::roman_to_int(s);
        println!("roman_to_int = {}", result);
        assert_eq!(result, 1994);
    }
}
