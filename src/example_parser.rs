use std::collections::HashMap;

use scraper::{Html, Selector};

use errors::ExampleParsingError;

use crate::errors;

#[derive(Debug, Eq, PartialEq)]
pub enum InputType {
    Int(i32),
    VecInt(Vec<i32>),
    String(String),
    VecString(Vec<String>),
    // TODO: Well, run the big test and fix 1 by 1
}


#[derive(Debug)]
pub struct Example {
    pub inputs: HashMap<String, InputType>,
    pub output: InputType,
}
#[derive(Debug)]
pub struct ExampleParser {
    // The LeetCode Markdown text
    description: String,
    pub examples: Vec<Example>,
}

impl ExampleParser {
    fn html_to_text(html_content: &str) -> String {
        let document = Html::parse_document(html_content);
        // TODO fix this unwrap
        let selector = Selector::parse("body").unwrap();
        let mut text_content = String::new();

        for element in document.select(&selector) {
            text_content.push_str(&element.text().collect::<Vec<_>>().join(" "));
        }

        text_content
    }
    pub fn new(html: &str) -> Self {
        let description = Self::html_to_text(html);
        ExampleParser {
            description,
            examples: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), ExampleParsingError> {
        let example_sections = self.get_example_sections()?;
        let example_sections = self.split_examples(example_sections);

        let examples = example_sections.iter().filter_map(|example| self.parse_example(example)).collect::<Vec<Example>>();

        self.examples = examples;
        Ok(())
    }

    fn get_example_sections(&self) -> Result<&str, ExampleParsingError> {
        let start = self.description.find("Example 1:").ok_or(ExampleParsingError::CouldNotFindExample)?;
        let end = self.description.rfind("Constraints:").ok_or(ExampleParsingError::CouldNotFindConstraints)?;

        Ok(&self.description[start..end])
    }

    fn split_examples(&self, examples_section: &str) -> Vec<String> {
        examples_section.split("Example ").filter_map(|s| {
            let cleaned: String = s.replace("\n", " ").split_whitespace().collect::<Vec<&str>>().join(" ");
            if cleaned.is_empty() {
                None
            } else {
                Some(format!("Example {}", cleaned))
            }
        }).collect()
    }

    fn parse_example(&self, example: &str) -> Option<Example> {
        let input_start = example.find("Input:")?;
        let output_start = example.find("Output")?;
        let remainder = &example[output_start + 7..];
        let end_of_line = remainder.find(" Explanation").unwrap_or(remainder.len());


        // Adding the lengths of the actual words of Input:/Output:
        let inputs_str = example[input_start + 6..output_start].trim();
        let outputs_str = remainder[..end_of_line].trim();

        let inputs = self.parse_inputs(inputs_str);
        let output = self.parse_output(outputs_str);
        Some(Example { inputs, output })
    }

    fn parse_inputs(&self, input_str: &str) -> HashMap<String, InputType> {
        let mut inputs = HashMap::new();

        for input in input_str.split(',') {
            if let Some((key, value)) = input.split_once('=') {
                let parsed_value = Self::parse_value(value.trim());
                inputs.insert(key.trim().to_string(), parsed_value);
            }
        }
        inputs
    }

    fn parse_output(&self, output_str: &str) -> InputType {
        Self::parse_value(output_str)
    }
    fn parse_value(value: &str) -> InputType {
        if value.starts_with('[') && value.ends_with(']') {
            let trimmed = &value[1..value.len() - 1];
            if trimmed.contains('"') {
                let vec_str: Vec<String> = trimmed.split(',').map(|s| s.trim().replace('"', "")).collect();
                InputType::VecString(vec_str)
            } else {
                let vec_int: Vec<i32> = trimmed.split(',').filter_map(|s| s.trim().parse().ok()).collect();
                InputType::VecInt(vec_int)
            }
        } else if value.starts_with('"') && value.ends_with('"') {
            InputType::String(value.trim_matches('"').to_string())
        } else {
            match value.parse::<i32>() {
                Ok(int_val) => InputType::Int(int_val),
                Err(_) => InputType::String(value.to_string()),  // Default to String if parsing fails
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_get_example_sections() {
        let parser = ExampleParser::new(HTML_CONTENT);
        let section = parser.get_example_sections().unwrap();
        assert!(section.contains("Example 1:"));
        assert!(section.contains("Example 2:"));
        assert!(section.contains("Example 3:"));

        let parser_with_vec = ExampleParser::new(HTML_CONTENT_WITH_VEC);
        let section_with_vec = parser_with_vec.get_example_sections().unwrap();
        assert!(section_with_vec.contains("Example 1:"));
        assert!(section_with_vec.contains("Example 2:"));
        assert!(section_with_vec.contains("Example 3:"));
    }

    #[test]
    fn test_parse_example() {
        let parser = ExampleParser::new(HTML_CONTENT);
        let section = parser.get_example_sections().unwrap();
        let examples = parser.split_examples(section);
        assert_eq!(examples.len(), 3);
        assert!(examples[0].contains("Input: s = \"abcabcbb\""));
        assert!(examples[0].contains("Output: 3"));

        let parser_with_vec = ExampleParser::new(HTML_CONTENT_WITH_VEC);
        let section_with_vec = parser_with_vec.get_example_sections().unwrap();
        let examples_with_vec = parser_with_vec.split_examples(section_with_vec);
        assert_eq!(examples_with_vec.len(), 3);
        assert!(examples_with_vec[0].contains("Input: nums = [2,7,11,15], target = 9"));
        assert!(examples_with_vec[0].contains("Output: [0,1]"));
    }

    #[test]
    fn test_parse_output() {
        let mut parser = ExampleParser::new(HTML_CONTENT);
        parser.parse().unwrap();
        let output = &parser.examples[0].output;
        assert_eq!(output, &InputType::Int(3));

        let mut parser_with_vec = ExampleParser::new(HTML_CONTENT_WITH_VEC);
        parser_with_vec.parse().unwrap();
        let output_with_vec = &parser_with_vec.examples[0].output;
        assert_eq!(output_with_vec, &InputType::VecInt(vec![0, 1]));
    }

    #[test]
    fn test_empty() {
        let parser = ExampleParser::new("");
        let section = parser.get_example_sections();
        assert!(section.is_err());
    }
}