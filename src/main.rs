use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{fs, io::Write, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    filepath: PathBuf,

    #[structopt(short, long)]
    json: bool,

    #[structopt(short, long)]
    yaml: bool,

    #[structopt(long, short)]
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
        let yaml_data: JsonValue = serde_yaml::from_str(&file_content)?;
        let json_output = serde_json::to_string_pretty(&yaml_data)?;
        if let Some(output_file) = args.output {
            let mut file = fs::File::create(output_file)?;
            file.write_all(json_output.as_bytes())?;
        } else {
            println!("{:?}", json_output);
        }
    } else if args.yaml {
        let json_data: JsonValue = serde_json::from_str(&file_content)?;
        let yaml_output = serde_yaml::to_string(&json_data)?;
        if let Some(output_file) = args.output {
            let mut file = fs::File::create(output_file)?;
            file.write_all(yaml_output.as_bytes())?;
        } else {
            println!("{:?}", yaml_output);
        }
    } else {
        eprintln!("Invalid arguments!");
    }

    Ok(())
}
