/**
 * This is the main driver code for the starter.
 * Run with `cargo run` or `<project_name>` to see the auto-generated help text.
 */
use clap::Parser;

mod commands;
use commands::example::*;

#[derive(Parser)]
#[clap(name = "ignore")]
#[clap(author = "Myles <myles@themapletree.io>")]
#[clap(version = "0.1.0")]
#[clap(about = "Outputs the typical gitignore for a search term")]

struct Cli {
    query: String,
}

fn main() {
    let cli = Cli::parse();
    println!("{}", cli.query);

    // get query
    // check gitignore if a file exists
    // if one exists, output to stdout
    // else output error "can't find gitignore for query"
    

}
