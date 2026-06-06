//! Tự động hoá repo LeetCode: mỗi file `src/solutions/*.rs` (do extension tạo)
//! trở thành một **test crate riêng** trong `tests/` — nên:
//!
//! - Có autocomplete / type-check đầy đủ trong rust-analyzer (file là module
//!   thật qua `#[path]`, KHÔNG dùng `include!` -> không bị "detached file").
//! - **Cô lập**: một bài chưa làm xong / lỗi compile KHÔNG chặn việc test các
//!   bài khác. `cargo test --test p1_two_sum` chỉ biên dịch đúng bài đó.
//!
//! Mỗi lần build, script còn chèn "prelude local" (`struct Solution;`, và
//! `ListNode`/`TreeNode` nếu bài có dùng) vào ĐẦU file bài giải — phần này nằm
//! NGOÀI cặp `@lc code` marker nên KHÔNG bị gửi khi Submit, nhưng giúp file tự
//! đủ để compile ở local.

use std::collections::HashSet;
use std::{env, fs, path::Path};

const SENTINEL: &str = "leetcode-local-prelude";
const GEN_HEADER: &str = "// @generated bởi build.rs — đừng sửa tay.";

fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = Path::new(&manifest);
    let sol_dir = root.join("src").join("solutions");
    let tests_dir = root.join("tests");
    fs::create_dir_all(&sol_dir).expect("không tạo được src/solutions");
    fs::create_dir_all(&tests_dir).expect("không tạo được tests");

    // Chạy lại khi có file được thêm/xóa trong thư mục bài giải.
    println!("cargo:rerun-if-changed=src/solutions");

    // Dọn tàn dư của thiết kế cũ (single-crate).
    let _ = fs::remove_file(sol_dir.join("mod.rs"));

    let mut files: Vec<_> = fs::read_dir(&sol_dir)
        .expect("không đọc được src/solutions")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "rs"))
        .filter(|p| p.file_name().is_some_and(|n| n != "mod.rs"))
        .collect();
    files.sort();

    let mut expected: HashSet<String> = HashSet::new();

    for path in &files {
        let content = fs::read_to_string(path).unwrap_or_default();

        // (1) Chèn prelude local nếu chưa có.
        if !content.contains(SENTINEL) {
            let prelude = build_prelude(&content);
            if !prelude.is_empty() {
                fs::write(path, format!("{prelude}{content}")).expect("không ghi được file bài giải");
            }
        }

        // (2) Sinh test crate riêng trong tests/<ident>.rs.
        let stem = path.file_stem().unwrap().to_string_lossy();
        let ident = sanitize_ident(&stem);
        let file_name = path.file_name().unwrap().to_string_lossy();
        let rel = format!("../src/solutions/{file_name}");
        let wrapper = format!(
            "{GEN_HEADER}\n#[path = {rel:?}]\n#[allow(unused, non_snake_case)]\nmod solution;\n"
        );
        let dst = tests_dir.join(format!("{ident}.rs"));
        let need_write = fs::read_to_string(&dst).map(|old| old != wrapper).unwrap_or(true);
        if need_write {
            fs::write(&dst, wrapper).expect("không ghi được tests wrapper");
        }
        expected.insert(format!("{ident}.rs"));
    }

    // (3) Xóa các wrapper mồ côi (bài đã bị xóa/đổi tên) — chỉ xóa file do mình sinh.
    if let Ok(rd) = fs::read_dir(&tests_dir) {
        for entry in rd.filter_map(Result::ok) {
            let p = entry.path();
            if p.extension().is_some_and(|x| x == "rs") {
                let name = p.file_name().unwrap().to_string_lossy().to_string();
                if !expected.contains(&name) {
                    if let Ok(c) = fs::read_to_string(&p) {
                        if c.starts_with(GEN_HEADER) {
                            let _ = fs::remove_file(&p);
                        }
                    }
                }
            }
        }
    }
}

/// Dựng phần prelude local-only cần chèn cho 1 file (rỗng nếu không cần gì).
fn build_prelude(content: &str) -> String {
    let mut needed = String::new();

    if !declares_struct(content, "Solution") {
        needed.push_str("struct Solution;\n");
    }
    if content.contains("ListNode") && !declares_struct(content, "ListNode") {
        needed.push_str(
            "#[derive(PartialEq, Eq, Clone, Debug)]\npub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }\n\
             impl ListNode { #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } } }\n",
        );
    }
    if content.contains("TreeNode") && !declares_struct(content, "TreeNode") {
        needed.push_str(
            "use std::cell::RefCell; use std::rc::Rc;\n\
             #[derive(Debug, PartialEq, Eq)]\npub struct TreeNode { pub val: i32, pub left: Option<Rc<RefCell<TreeNode>>>, pub right: Option<Rc<RefCell<TreeNode>>> }\n\
             impl TreeNode { #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } } }\n",
        );
    }

    if needed.is_empty() {
        return String::new();
    }
    // LƯU Ý: 2 dòng comment dưới TUYỆT ĐỐI không được chứa chuỗi marker của @lc,
    // vì CLI dò marker bằng indexOf trên cả file -> sẽ bắt nhầm và gửi cả prelude.
    format!(
        "// ─── {SENTINEL} — tự thêm để biên dịch ở local; phần này KHÔNG được gửi khi Submit.\n\
         // Đừng bỏ comment ListNode/TreeNode trong phần lời giải — chúng đã được khai báo ở đây.\n\
         {needed}\n"
    )
}

/// Có khai báo `struct <name>` thật (không tính trong comment) hay không.
fn declares_struct(content: &str, name: &str) -> bool {
    let needle = format!("struct {name}");
    content.lines().any(|line| {
        let t = line.trim_start();
        if t.starts_with("//") || t.starts_with('*') || t.starts_with("/*") {
            return false;
        }
        match t.find(&needle) {
            Some(i) => {
                let after = &t[i + needle.len()..];
                matches!(after.chars().next(), None | Some(' ' | '{' | ';' | '<'))
            }
            None => false,
        }
    })
}

/// Biến tên file thành identifier Rust hợp lệ (cũng là tên test target).
fn sanitize_ident(stem: &str) -> String {
    let mut s: String = stem
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect();
    if s.chars().next().is_none_or(|c| c.is_ascii_digit()) {
        s.insert(0, 'p');
    }
    s.to_lowercase()
}
