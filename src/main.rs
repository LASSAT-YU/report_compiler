use clap::Parser;
use report_compiler::Cli;

fn main() {
    let args = Cli::parse();

    dbg!(args);

    // TODO Load input files into memory

    // TODO Generate output from input files

    // TODO Save output to disk and report where file was saved
}
