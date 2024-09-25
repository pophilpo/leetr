use crate::{errors::ProjectGeneratorError, project_generator::metadata::Metadata};
use serde_json;

#[derive(Debug)]
pub enum ExampleType {
    VecInt(Vec<i32>),
    VecVecInt(Vec<Vec<i32>>),
    Int(i32),
    String(String),
    VecVecChar(Vec<Vec<char>>),
    VecChar(Vec<char>),
    Double(f64),
    VecDouble(Vec<f64>),
    VecString(Vec<String>),
    VecVecString(Vec<Vec<String>>),
    Char(char),
    VecBool(Vec<bool>),
    Long(i64),
    VecLong(Vec<i64>),
    VecVecLong(Vec<Vec<i64>>),
    Bool(bool),
    Void,
}

impl ExampleType {
    pub fn from_string(
        example_type_string: &str,
        value: &str,
    ) -> Result<Self, ProjectGeneratorError> {
        match example_type_string {
            "integer[]" => {
                let value: Vec<i32> = serde_json::from_str(value)?;
                Ok(ExampleType::VecInt(value))
            }
            "integer" => {
                let value: i32 = serde_json::from_str(value)?;
                Ok(ExampleType::Int(value))
            }

            "integer[][]" => {
                let value: Vec<Vec<i32>> = serde_json::from_str(value)?;
                Ok(ExampleType::VecVecInt(value))
            }

            "list<integer>" => {
                let value: Vec<i32> = serde_json::from_str(value)?;
                Ok(ExampleType::VecInt(value))
            }

            "list<list<integer>>" => {
                let value: Vec<Vec<i32>> = serde_json::from_str(value)?;
                Ok(ExampleType::VecVecInt(value))
            }

            "string" => {
                let value: String = serde_json::from_str(value)?;

                Ok(ExampleType::String(value))
            }
            "string[]" => {
                let value: Vec<String> = serde_json::from_str(value)?;
                Ok(ExampleType::VecString(value))
            }

            "list<string>" => {
                let value: Vec<String> = serde_json::from_str(value)?;
                Ok(ExampleType::VecString(value))
            }

            "list<list<string>>" => {
                let value: Vec<Vec<String>> = serde_json::from_str(value)?;
                Ok(ExampleType::VecVecString(value))
            }

            "character" => {
                let value: char = serde_json::from_str(value)?;
                Ok(ExampleType::Char(value))
            }
            "character[]" => {
                let value: Vec<char> = serde_json::from_str(value)?;
                Ok(ExampleType::VecChar(value))
            }
            "character[][]" => {
                let value: Vec<Vec<char>> = serde_json::from_str(value)?;
                Ok(ExampleType::VecVecChar(value))
            }
            "double" => {
                let value: f64 = serde_json::from_str(value)?;
                Ok(ExampleType::Double(value))
            }

            "double[]" => {
                let value: Vec<f64> = serde_json::from_str(value)?;
                Ok(ExampleType::VecDouble(value))
            }

            "list<double>" => {
                let value: Vec<f64> = serde_json::from_str(value)?;
                Ok(ExampleType::VecDouble(value))
            }

            "boolean" => {
                let value: bool = serde_json::from_str(value)?;
                Ok(ExampleType::Bool(value))
            }

            "boolean[]" => {
                let value: Vec<bool> = serde_json::from_str(value)?;
                Ok(ExampleType::VecBool(value))
            }

            "list<boolean>" => {
                let value: Vec<bool> = serde_json::from_str(value)?;
                Ok(ExampleType::VecBool(value))
            }

            "long" => {
                let value: i64 = serde_json::from_str(value)?;
                Ok(ExampleType::Long(value))
            }

            "long[]" => {
                let value: Vec<i64> = serde_json::from_str(value)?;
                Ok(ExampleType::VecLong(value))
            }

            "long[][]" => {
                let value: Vec<Vec<i64>> = serde_json::from_str(value)?;
                Ok(ExampleType::VecVecLong(value))
            }

            "list<long>" => {
                let value: Vec<i64> = serde_json::from_str(value)?;
                Ok(ExampleType::VecLong(value))
            }

            "list<list<long>>" => {
                let value: Vec<Vec<i64>> = serde_json::from_str(value)?;
                Ok(ExampleType::VecVecLong(value))
            }

            "void" => Ok(ExampleType::Void),

            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub struct Input {
    value: ExampleType,
    name: String,
}

#[derive(Debug)]
pub struct Output {
    value: ExampleType,
}

#[derive(Debug)]
pub struct Example {
    fn_name: String,
    inputs: Vec<Input>,
    output: Output,
}

impl Example {
    pub fn new(
        raw_inputs: Vec<String>,
        outputs: Vec<String>,
        metadata: Metadata,
    ) -> Result<Vec<Self>, ProjectGeneratorError> {
        assert_eq!(raw_inputs.len(), outputs.len());

        match metadata {
            Metadata::Function(metadata) => {
                let mut examples: Vec<Self> = Vec::new();
                for (raw_input, raw_output) in raw_inputs.iter().zip(outputs.iter()) {
                    let splitted_inputs: Vec<&str> = raw_input.split("\n").collect();
                    let name = metadata.name.clone().unwrap();

                    let output_type = &metadata.return_type.return_type;
                    let output_value = ExampleType::from_string(&output_type, &raw_output)?;
                    let output = Output {
                        value: output_value,
                    };

                    let params = &metadata.params;
                    assert_eq!(splitted_inputs.len(), params.len());
                    let zipped: Vec<_> = params.iter().zip(splitted_inputs.iter()).collect();
                    let mut inputs: Vec<Input> = Vec::with_capacity(splitted_inputs.len());

                    for (param, input) in zipped {
                        let param_name = param.name.clone();
                        let param_type = param.param_type.clone();
                        let example_type = ExampleType::from_string(&param_type, input)?;
                        let input = Input {
                            value: example_type,
                            name: param_name,
                        };
                        inputs.push(input);
                    }

                    let example = Example {
                        fn_name: name.to_string(),
                        inputs,
                        output,
                    };
                    examples.push(example);
                }

                Ok(examples)
            }

            Metadata::Class(metadata) => unimplemented!(),
        }
    }
}
