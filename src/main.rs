use clap::Parser;
use serde_json::Value;
mod domain;
mod usecase;
mod infrastructure;


#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let json = infrastructure::file_io::read_json_file(&args.input);
    let cleaned = usecase::clean_json_usecase::CleanJsonUsecase::execute(json);
    if let Some(output_path) = args.output {
        infrastructure::file_io::write_json_file(&output_path, &cleaned);
    } else {
        println!("{}", serde_json::to_string_pretty(&cleaned).unwrap());
    }
}

