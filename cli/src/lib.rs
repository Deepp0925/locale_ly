use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputFormats {
    Json,
    Yaml,
    Strings,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// The input file to use, this can either be yaml/yml or json
    #[arg(short, long)]
    input: String,

    /// the relative path to the output directory
    #[arg(short, long)]
    output: String,

    /// the formats in which the output files should be generated
    /// Currently supported formats are: json, yaml/ yml, strings(ios), more to come soon
    #[arg(short, long)]
    formats: Vec<String>,

    /// Allow inaccurate language translations to be generated.
    /// This should only be used if you are confident in the quality of the translations
    #[arg(short, long)]
    allow_inaccurate: bool,
}

pub fn run() {
    let args = CliArgs::parse();

    println!("args: {:?}", args);
}
