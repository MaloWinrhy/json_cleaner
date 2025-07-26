use clap::Parser;
use std::fs;
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

    #[arg(long)]
    in_place: bool,
}


fn main() {
    let args = Args::parse();
    let json = infrastructure::file_io::read_json_file(&args.input);
    let cleaned = usecase::clean_json_usecase::CleanJsonUsecase::execute(json);
    let output = serde_json::to_string_pretty(&cleaned).expect("Failed to serialize cleaned JSON");
    if args.in_place {
        fs::write(&args.input, &output).expect("Failed to overwrite input file");
    } else if let Some(output_path) = args.output {
        fs::write(output_path, &output).expect("Failed to write output");
    } else {
        println!("{}", output);
    }

}

