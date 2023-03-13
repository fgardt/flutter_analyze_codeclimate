use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

use clap::Parser;

mod codeclimate_issue;
mod flutter_issue;

use codeclimate_issue::CodeClimateIssue;
use flutter_issue::FlutterIssue;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The file to read in (output of `flutter analyze`)
    #[clap(short, long, value_parser)]
    input: PathBuf,

    /// The file to write the codeclimate report to
    #[clap(short, long, value_parser)]
    output: PathBuf,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let input_file = File::open(cli.input)?;
    let input_reader = BufReader::new(input_file);

    let mut result = Vec::new();

    for line in input_reader.lines() {
        let source_issue = FlutterIssue::try_from(line?).unwrap();
        let target_issue = CodeClimateIssue::from(source_issue);

        result.push(target_issue);
    }

    let mut output_file = File::create(cli.output)?;
    write!(output_file, "{}", serde_json::to_string(&result).unwrap())
}
