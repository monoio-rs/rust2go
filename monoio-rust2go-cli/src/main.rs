use clap::Parser;
use monoio_rust2go_cli::{generate, Args};

fn main() {
    let args = Args::parse();
    generate(&args);
}
