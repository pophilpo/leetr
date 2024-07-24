use prettyplease::unparse;
use syn::{File, ImplItem, Item};

use crate::errors::CodeParserError;
use crate::example_parser::{ExampleParser, InputType};
use crate::project_generator::ProjectType;

pub struct CodeParser<'a> {
    project_type: ProjectType,
    input_string: &'a str,
    example: ExampleParser,
}

impl<'a> CodeParser<'a> {
    pub fn new(project_type: ProjectType, input_string: &'a str, example: ExampleParser) -> Self {
        CodeParser {
            project_type,
            input_string,
            example,
        }
    }

    pub fn parse(&self) -> String {
        match &self.project_type {
            ProjectType::Rust(_) => self.rust().unwrap(),
            _ => unreachable!(),
        }
    }

    fn rust(&self) -> Result<String, CodeParserError> {
        let mut syntax_tree: File = syn::parse_str(self.input_string)?;
        for item in &mut syntax_tree.items {
            if let Item::Impl(ref mut impl_block) = item {
                for item in &mut impl_block.items {
                    if let ImplItem::Fn(ref mut function) = item {
                        if function.block.stmts.is_empty() {
                            function.block.stmts.push(syn::parse_quote! {
                                todo!("Implement solution!");
                            });
                        }
                    }
                }
            }
        }

        let no_test_code = unparse(&syntax_tree);

        let syntax_tree: File = syn::parse_str(&no_test_code)?;

        for item in &syntax_tree.items {
            if let Item::Impl(_impl_block) = item {
                for item in &syntax_tree.items {
                    if let Item::Impl(impl_block) = item {
                        for item in &impl_block.items {
                            if let ImplItem::Fn(function) = item {
                                let fn_name = &function.sig.ident;
                                let mut asserts = Vec::new();
                                for example in &self.example.examples {
                                    let inputs = &example.inputs;
                                    let output = &example.output;

                                    let mut key_value_pairs: Vec<(&String, &InputType)> =
                                        inputs.iter().collect();
                                    key_value_pairs.sort_by(|a, b| a.0.cmp(b.0));

                                    let inputs_string: Vec<String> = key_value_pairs
                                        .iter()
                                        .map(|(key, value)| {
                                            format!("{}={}", key, value.to_string_rust())
                                        })
                                        .collect();

                                    // TODO: After very thorough testing is done, remove the sorting of the hashmap
                                    // TODO: It is needed so that the output is consistent for testing
                                    let inputs_string = inputs_string.join(", ");
                                    // let inputs_string: Vec<String> = inputs.iter().map(|(key, value)| format!("{}={}", key, value.to_string())).collect();
                                    // let inputs_string = inputs_string.join(" ,");

                                    // TODO: In need of adding .into() for String/str types
                                    let fn_input_string =
                                        format!("Solution::{}({})", fn_name, inputs_string);
                                    let assert_code = format!(
                                        "assert_eq!({}, {});",
                                        output.to_string_rust(),
                                        fn_input_string
                                    );
                                    asserts.push(assert_code);
                                }

                                let test_code = asserts.join("\n        ");
                                let code = format!(
                                    r#"
#[cfg(test)]
mod tests {{
    use super::*;
    #[test]
    fn test_{}() {{
        {}
    }}
}}"#,
                                    fn_name, test_code
                                );
                                let final_code = format!("{}{}\n", no_test_code, code);
                                return Ok(final_code);
                            }
                        }
                    }
                }
            }
        }

        Ok(no_test_code)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_rust_parsing() {
        let test_string = r#"impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        }
    }"#;
        const HTML_CONTENT_WITH_VEC: &str = r#"
        <p>Given an array of integers <code>nums</code>&nbsp;and an integer <code>target</code>, return <em>indices of the two numbers such that they add up to <code>target</code></em>.</p>

        <p>You may assume that each input would have <strong><em>exactly</em> one solution</strong>, and you may not use the <em>same</em> element twice.</p>

        <p>You can return the answer in any order.</p>

        <p>&nbsp;</p>
        <p><strong class="example">Example 1:</strong></p>

        <pre>
        <strong>Input:</strong> nums = [2,7,11,15], target = 9
        <strong>Output:</strong> [0,1]
        <strong>Explanation:</strong> Because nums[0] + nums[1] == 9, we return [0, 1].
        </pre>

        <p><strong class="example">Example 2:</strong></p>

        <pre>
        <strong>Input:</strong> nums = [3,2,4], target = 6
        <strong>Output:</strong> [1,2]
        </pre>

        <p><strong class="example">Example 3:</strong></p>

        <pre>
        <strong>Input:</strong> nums = [3,3], target = 6
        <strong>Output:</strong> [0,1]
        </pre>

        <p>&nbsp;</p>
        <p><strong>Constraints:</strong></p>

        <ul>
            <li><code>2 &lt;= nums.length &lt;= 10<sup>4</sup></code></li>
            <li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
            <li><code>-10<sup>9</sup> &lt;= target &lt;= 10<sup>9</sup></code></li>
            <li><strong>Only one valid answer exists.</strong></li>
        </ul>

        <p>&nbsp;</p>
        <strong>Follow-up:&nbsp;</strong>Can you come up with an algorithm that is less than <code>O(n<sup>2</sup>)</code> time complexity?
    "#;

        let expected_code = r#"impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        todo!("Implement solution!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum() {
        assert_eq!(vec![0, 1], Solution::two_sum(nums=vec![2, 7, 11, 15], target=9));
        assert_eq!(vec![1, 2], Solution::two_sum(nums=vec![3, 2, 4], target=6));
        assert_eq!(vec![0, 1], Solution::two_sum(nums=vec![3, 3], target=6));
    }
}
"#;
        let project_type = ProjectType::Rust("rust".into());
        let mut example = ExampleParser::new(HTML_CONTENT_WITH_VEC);
        example.parse().unwrap();
        let parser = CodeParser::new(project_type, test_string, example);
        assert_eq!(parser.parse(), expected_code);

        const HTML_CONTENT_WITH_COMBINATIONS: &str = r#"
        <p>Given a string containing digits from <code>2-9</code> inclusive, return all possible letter combinations that the number could represent. Return the answer in <strong>any order</strong>.</p>

        <p>A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.</p>
        <img alt="Telephone keypad" src="https://assets.leetcode.com/uploads/2022/03/15/1200px-telephone-keypad2svg.png" style="width: 300px; height: 243px;" />
        <p>&nbsp;</p>
        <p><strong class="example">Example 1:</strong></p>

        <pre>
        <strong>Input:</strong> digits = "23"
        <strong>Output:</strong> ["ad","ae","af","bd","be","bf","cd","ce","cf"]
        </pre>

        <p><strong class="example">Example 2:</strong></p>

        <pre>
        <strong>Input:</strong> digits = ""
        <strong>Output:</strong> []
        </pre>

        <p><strong class="example">Example 3:</strong></p>

        <pre>
        <strong>Input:</strong> digits = "2"
        <strong>Output:</strong> ["a","b","c"]
        </pre>

        <p>&nbsp;</p>
        <p><strong>Constraints:</strong></p>

        <ul>
            <li><code>0 &lt;= digits.length &lt;= 4</code></li>
            <li><code>digits[i]</code> is a digit in the range <code>['2', '9']</code>.</li>
        </ul>
    "#;
        let test_string = r#"impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {

    }
}"#;
        let expected_code = r#"impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        todo!("Implement solution!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_letter_combinations() {
        assert_eq!(vec![String::from("ad"), String::from("ae"), String::from("af"), String::from("bd"), String::from("be"), String::from("bf"), String::from("cd"), String::from("ce"), String::from("cf")], Solution::letter_combinations(digits=String::from("23")));
        assert_eq!(vec![], Solution::letter_combinations(digits=String::from("")));
        assert_eq!(vec![String::from("a"), String::from("b"), String::from("c")], Solution::letter_combinations(digits=String::from("2")));
    }
}
"#;
        let project_type = ProjectType::Rust("rust".into());
        let mut example = ExampleParser::new(HTML_CONTENT_WITH_COMBINATIONS);
        example.parse().unwrap();
        let parser = CodeParser::new(project_type, test_string, example);
        assert_eq!(parser.parse(), expected_code);
        const HTML_CONTENT: &str = r#"
        <p>Given a string <code>s</code>, find the length of the <strong>longest</strong>
        <span data-keyword="substring-nonempty"><strong>substring</strong></span> without repeating characters.</p>

        <p>&nbsp;</p>
        <p><strong class="example">Example 1:</strong></p>

        <pre>
        <strong>Input:</strong> s = "abcabcbb"
        <strong>Output:</strong> 3
        <strong>Explanation:</strong> The answer is "abc", with the length of 3.
        </pre>

        <p><strong class="example">Example 2:</strong></p>

        <pre>
        <strong>Input:</strong> s = "bbbbb"
        <strong>Output:</strong> 1
        <strong>Explanation:</strong> The answer is "b", with the length of 1.
        </pre>

        <p><strong class="example">Example 3:</strong></p>

        <pre>
        <strong>Input:</strong> s = "pwwkew"
        <strong>Output:</strong> 3
        <strong>Explanation:</strong> The answer is "wke", with the length of 3.
        Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
        </pre>

        <p>&nbsp;</p>
        <p><strong>Constraints:</strong></p>

        <ul>
            <li><code>0 &lt;= s.length &lt;= 5 * 10<sup>4</sup></code></li>
            <li><code>s</code> consists of English letters, digits, symbols and spaces.</li>
        </ul>
    "#;

        let test_string = r#"impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {

    }
}"#;
        let expected_code = r#"impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        todo!("Implement solution!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(3, Solution::length_of_longest_substring(s=String::from("abcabcbb")));
        assert_eq!(1, Solution::length_of_longest_substring(s=String::from("bbbbb")));
        assert_eq!(3, Solution::length_of_longest_substring(s=String::from("pwwkew")));
    }
}
"#;
        let project_type = ProjectType::Rust("rust".into());
        let mut example = ExampleParser::new(HTML_CONTENT);
        example.parse().unwrap();
        let parser = CodeParser::new(project_type, test_string, example);
        assert_eq!(parser.parse(), expected_code);
    }
}

