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
    pub fn to_string_rust(&self) -> String {
        match self {
            ExampleType::VecInt(value) => {
                let inner: String = value
                    .iter()
                    .map(|num| num.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("vec![{}]", inner)
            }
            ExampleType::VecVecInt(value) => {
                let mut inner_arrays: Vec<String> = Vec::new();
                for inner_array in value {
                    let inner: String = inner_array
                        .iter()
                        .map(|num| num.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    let inner_array_string = format!("vec![{}]", inner);
                    inner_arrays.push(inner_array_string);
                }
                let inner_arrays_string = inner_arrays.join(", ");
                format!("vec![{}]", inner_arrays_string)
            }
            ExampleType::Int(value) => value.to_string(),
            ExampleType::String(value) => format!("\"{}\".to_string()", value),
            ExampleType::VecVecChar(value) => {
                let mut inner_arrays: Vec<String> = Vec::new();
                for inner_array in value {
                    let inner: String = inner_array
                        .iter()
                        .map(|c| format!("'{}'", c))
                        .collect::<Vec<String>>()
                        .join(", ");
                    let inner_array_string = format!("vec![{}]", inner);
                    inner_arrays.push(inner_array_string);
                }
                let inner_arrays_string = inner_arrays.join(", ");
                format!("vec![{}]", inner_arrays_string)
            }
            ExampleType::VecChar(value) => {
                let inner: String = value
                    .iter()
                    .map(|c| format!("'{}'", c))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("vec![{}]", inner)
            }
            ExampleType::Double(value) => value.to_string(),
            ExampleType::VecDouble(value) => {
                let inner = value
                    .iter()
                    .map(|num| num.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("vec![{}]", inner)
            }
            ExampleType::VecString(value) => {
                let inner = value
                    .iter()
                    .map(|s| format!("\"{}\".to_string()", s))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("vec![{}]", inner)
            }
            ExampleType::VecVecString(value) => {
                let mut inner_arrays: Vec<String> = Vec::new();
                for inner_array in value {
                    let inner: String = inner_array
                        .iter()
                        .map(|s| format!("\"{}\".to_string()", s))
                        .collect::<Vec<String>>()
                        .join(", ");
                    let inner_array_string = format!("vec![{}]", inner);
                    inner_arrays.push(inner_array_string);
                }
                let inner_arrays_string = inner_arrays.join(", ");
                format!("vec![{}]", inner_arrays_string)
            }

            ExampleType::Char(value) => format!("'{}'", value),
            ExampleType::VecBool(value) => {
                let inner = value
                    .iter()
                    .map(|b| b.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("vec![{}]", inner)
            }
            ExampleType::Long(value) => value.to_string(),
            ExampleType::VecLong(value) => {
                let inner = value
                    .iter()
                    .map(|l| l.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("vec![{}]", inner)
            }
            ExampleType::VecVecLong(value) => {
                let mut inner_arrays: Vec<String> = Vec::new();
                for inner_array in value {
                    let inner: String = inner_array
                        .iter()
                        .map(|l| l.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    let inner_array_string = format!("vec![{}]", inner);
                    inner_arrays.push(inner_array_string);
                }
                let inner_arrays_string = inner_arrays.join(", ");
                format!("vec![{}]", inner_arrays_string)
            }
            ExampleType::Bool(value) => value.to_string(),
            ExampleType::Void => String::from("()"),
        }
    }

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
    pub value: ExampleType,
    pub name: String,
}

#[derive(Debug)]
pub struct Output {
    pub value: ExampleType,
}

#[derive(Debug)]
pub struct Example {
    pub fn_name: String,
    pub inputs: Vec<Input>,
    pub output: Output,
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
