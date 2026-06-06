//! Repo lời giải LeetCode.
//!
//! Mỗi bài trong `src/solutions/` được `build.rs` biến thành một **test crate
//! riêng** trong `tests/`, nên một bài lỗi không chặn việc test bài khác:
//!
//! ```text
//! cargo test --test p1_two_sum -- --nocapture   # chạy + xem print 1 bài
//! cargo test                                     # chạy tất cả bài compile được
//! ```
//!
//! Crate lib này cố tình để trống — lời giải KHÔNG nằm ở đây.
