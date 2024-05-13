const RUST_TEMPLATE: &str = r#"
fn main() {
  {solution_function_title}();
}

{leetcode_fn}

mod tests {
}
"#;
