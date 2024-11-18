use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{fs, io::Write, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    filepath: PathBuf,

    #[structopt(short, long, about = "Convert yaml to json")]
    json: bool,

    #[structopt(short, long, about = "Convert json to yaml")]
    yaml: bool,

    #[structopt(long, short, about = "Save output in provided file")]
    output: Option<PathBuf>,
}

#[derive(Deserialize, Serialize)]
struct Data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let file_content = fs::read_to_string(&args.filepath)?;

    //convert based on specified argument
    if args.json {
        //convert yaml to json
        let json_output = convert_to_json(&file_content)?;
        if let Some(ref output_file) = args.output {
            match write_to_file(&json_output, &output_file) {
                Ok(_) => {
                    println!("Output written to file: {:?}", output_file);
                }
                Err(e) => {
                    eprintln!("Failed to write to output file: {:?}\n{:?}", output_file, e);
                }
            }
        } else {
            println!("{:?}", json_output);
        }
    } else if args.yaml {
        let yaml_output = convert_to_yaml(&file_content)?;
        if let Some(ref output_file) = args.output {
            match write_to_file(&yaml_output, &output_file) {
                Ok(_) => {
                    println!("Output written to file: {:?}", output_file);
                }
                Err(e) => {
                    eprintln!("Failed to write to output file: {:?}\n{:?}", output_file, e);
                }
            }
        } else {
            println!("{:?}", yaml_output);
        }
    } else {
        eprintln!("Invalid arguments!");
    }

    Ok(())
}

fn convert_to_json(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let yaml_content: JsonValue = serde_yaml::from_str(&content)?;
    let json_output = serde_json::to_string_pretty(&yaml_content)?;
    Ok(json_output)
}

fn convert_to_yaml(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let json_content: JsonValue = serde_json::from_str(&content)?;
    let yaml_output = serde_yaml::to_string(&json_content)?;
    Ok(yaml_output)
}

fn write_to_file(content: &str, output_file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&output_file)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
//tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_convert_yaml_to_json() {
        let yaml_content = r#"
name: John Doe
age: 30
skills:
  - Rust
  - Testing
"#;
        let result = convert_to_json(yaml_content);
        assert!(result.is_ok());
        let json_output = result.unwrap();
        assert!(json_output.contains("John Doe"));
        assert!(json_output.contains("30"));
        assert!(json_output.contains("Rust"));
    }

    #[test]
    fn test_convert_json_to_yaml() {
        let json_content = r#"{
    "name": "John Doe",
    "age": 30,
    "skills": ["Rust", "Testing"]
}"#;
        let result = convert_to_yaml(json_content);
        assert!(result.is_ok());
        let yaml_output = result.unwrap();
        assert!(yaml_output.contains("John Doe"));
        assert!(yaml_output.contains("age: 30"));
        assert!(yaml_output.contains("- Rust"));
    }

    #[test]
    fn test_write_to_file() {
        let content = "test content";
        let temp_file = NamedTempFile::new().unwrap();
        let result = write_to_file(content, &temp_file.path().to_path_buf());
        assert!(result.is_ok());

        let saved_content = fs::read_to_string(temp_file.path()).unwrap();
        assert_eq!(saved_content, content);
    }

    #[test]
    fn test_invalid_yaml_conversion() {
        let invalid_yaml = "invalid: : yaml";
        let result = convert_to_json(invalid_yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_json_conversion() {
        let invalid_json = "{invalid: json}";
        let result = convert_to_yaml(invalid_json);
        assert!(result.is_err());
    }
}
