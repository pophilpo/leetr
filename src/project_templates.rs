pub const RUST_TEMPLATE: &str = r#"

// This is a dummy struct, so that we can build the project
struct Solution {}

{solution code}

fn main() {
    let solution = Solution {};
}

mod tests {
}
"#;
