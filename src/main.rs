
use std::io::BufReader;
use std::fs::File;
use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

/// Search for a pattern in a file and display the lines that contain it
#[derive(StructOpt)]
struct Cli {
	/// The pattern to look for
	pattern: String,
	/// The path to the file to read
	#[structopt(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let file = File::open(&args.path)
        .with_context(|_| format!("could not read file '{}'", args.path.display()))?;

    // Buffer reading in the case of large input files
    let mut reader = BufReader::new(file);

    // Buffer writing in the case of large numbers of matches
    let mut handle = std::io::BufWriter::new(std::io::stdout());

    grrs::find_matches(&mut reader, &args.pattern, &mut handle)
}
