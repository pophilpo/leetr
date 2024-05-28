pub const RUST_TEMPLATE: &str = r#"// This is a dummy struct, so that we can build the project
struct Solution {}

{solution code}

fn main() {
    let solution = Solution {};
}

mod tests {
}
"#;

// NOTE: using import * here since leetcode return code with something like "List" which is
// not recognized by python wihtout an extra import
pub const PYTHON_TEMPLATE: &str = r#"from typing import *


{solution code}pass


def main():
    solution = Solution()


if __name__ == "__main__":
    main()
"#;
