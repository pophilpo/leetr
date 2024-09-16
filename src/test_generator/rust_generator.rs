use crate::errors::TestGenerationError;
use crate::test_generator::traits::TestGenerator;
use log::{debug, error, info, warn};
use proc_macro2;
use quote::quote;
use syn::{parse_file, FnArg, ImplItem, Item, Pat, ReturnType};
use syn::{PatType, Type};

pub struct RustTestGenerator;

impl TestGenerator for RustTestGenerator {
    fn generate_tests(
        &self,
        code_snippet: &str,
        test_cases: &[String],
    ) -> Result<String, TestGenerationError> {
        // Parse the code snippet using syn
        info!("Parsing code snippet for generating tests.");
        let ast = match parse_file(code_snippet) {
            Ok(ast) => ast,
            Err(err) => {
                error!("Failed to parse code snippet: {:?}", err);
                return Err(TestGenerationError::ParseError);
            }
        };

        // Extract function information
        let mut functions = Vec::new();
        for item in ast.items {
            if let Item::Impl(item_impl) = item {
                for impl_item in item_impl.items {
                    if let ImplItem::Fn(fn_item) = impl_item {
                        let fn_name = fn_item.sig.ident.to_string();
                        let inputs = fn_item.sig.inputs.clone();
                        let output = fn_item.sig.output.clone();
                        functions.push((fn_name, inputs, output));
                    }
                }
            }
        }

        if functions.is_empty() {
            error!("No functions found in the parsed code.");
            return Err(TestGenerationError::NoFunctionsFoundError);
        }

        // Use the first function found for generating tests
        let (fn_name, inputs, _output) = &functions[0];
        info!("Generating tests for function: {}", fn_name);

        // Parse test cases
        let parsed_test_cases = match parse_test_cases(test_cases) {
            Ok(test_cases) => test_cases,
            Err(err) => {
                error!("Failed to parse test cases: {:?}", err);
                return Err(err);
            }
        };

        // Generate test functions
        let test_functions = parsed_test_cases
            .iter()
            .enumerate()
            .map(|(i, (input, expected))| {
                let test_fn_name = syn::Ident::new(
                    &format!("test_{}_{}", fn_name, i + 1),
                    proc_macro2::Span::call_site(),
                );

                // Generate input arguments based on the function signature
                let input_args = match generate_input_args(inputs.clone(), input) {
                    Some(args) => args,
                    None => {
                        error!(
                            "Failed to generate input arguments for test case {}: {}",
                            i + 1,
                            input
                        );
                        vec![]
                    }
                };

                let expected_value = expected.clone();

                quote! {
                    #[test]
                    fn #test_fn_name() {
                        let result = Solution::#fn_name(#(#input_args),*);
                        assert_eq!(result, #expected_value);
                    }
                }
            })
            .collect::<Vec<_>>();

        // Generate the test module
        let test_module = quote! {
            #[cfg(test)]
            mod tests {
                use super::*;

                #(#test_functions)*
            }
        };

        info!("Test module generated successfully.");
        Ok(test_module.to_string())
    }
}

// Parse test cases, assuming they are split by newline into input/output pairs

fn parse_test_cases(test_cases: &[String]) -> Result<Vec<(String, String)>, TestGenerationError> {
    let mut parsed = Vec::new();
    for case in test_cases {
        let parts: Vec<&str> = case.split('\n').collect();
        if parts.len() != 2 {
            error!("Invalid format for test case: {}", case);
            return Err(TestGenerationError::InvalidTestCaseFormatError(
                case.clone(),
            ));
        }

        let input = parts[0].to_string();
        let expected = parts[1].to_string();
        parsed.push((input, expected));
    }
    info!("Parsed {} test cases successfully.", parsed.len());
    Ok(parsed)
}

fn generate_input_args(
    inputs: syn::punctuated::Punctuated<FnArg, syn::token::Comma>,
    input: &str,
) -> Option<Vec<syn::Expr>> {
    // Split the input string by newline, assuming arguments and output are split by '\n'
    let args: Vec<&str> = input.split('\n').map(|s| s.trim()).collect();

    // Check if the number of inputs matches the function signature
    if args.len() != inputs.len() {
        error!(
            "Input argument count mismatch. Expected {}, got {}",
            inputs.len(),
            args.len()
        );
        return None;
    }

    let mut exprs = Vec::new();

    for (arg, value) in inputs.iter().zip(args.iter()) {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            // Determine the argument type and parse the input accordingly
            let parsed_value = match **ty {
                Type::Path(ref type_path) => {
                    let segment = &type_path.path.segments.last().unwrap().ident;
                    if segment == "Vec" {
                        // Handle Vec types (e.g., Vec<i32>) -- the whole value is treated as one argument
                        if value.starts_with('[') && value.ends_with(']') {
                            let inner_values = &value[1..value.len() - 1]; // Strip the square brackets
                            format!("vec![{}]", inner_values) // Convert to Rust vec! macro format
                        } else {
                            error!("Expected vector format (e.g., [x, y, z]) for Vec type");
                            return None;
                        }
                    } else if segment == "i32"
                        || segment == "i64"
                        || segment == "u32"
                        || segment == "usize"
                    {
                        // Handle integer types
                        value.to_string()
                    } else if segment == "String" {
                        // Handle String types (remove quotes if necessary)
                        if value.starts_with('"') && value.ends_with('"') {
                            value.to_string()
                        } else {
                            format!("\"{}\"", value)
                        }
                    } else {
                        error!("Unsupported type: {:?}", segment);
                        return None;
                    }
                }
                _ => {
                    error!("Unsupported argument type");
                    return None;
                }
            };

            // Try parsing the value into a Rust expression
            match syn::parse_str::<syn::Expr>(&parsed_value) {
                Ok(expr) => exprs.push(expr),
                Err(err) => {
                    error!(
                        "Failed to parse input '{}' for argument: {:?}",
                        parsed_value, err
                    );
                    return None;
                }
            }
        }
    }

    Some(exprs)
}
