use clap::Parser;
use report_compiler::Cli;

fn main() {
    let args = Cli::parse();

    dbg!(args);
}
