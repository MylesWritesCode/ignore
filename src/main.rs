/**
 * This is the main driver code for the starter.
 * Run with `cargo run` or `<project_name>` to see the auto-generated help text.
 */
use std::path::Path;

use clap::Parser;
use git2::Repository;

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

const REMOTE: &str = "https://github.com/github/gitignore";

fn main() {
    let cli = Cli::parse();

    // get query
    println!("{}", cli.query);

    // run query against repo

    let repo = match Repository::clone("https://github.com/github/gitignore", Path::new("./tmp")) {
        Ok(repo) => {
            let tree = repo.revparse_single("main");

            println!("{:?}", tree);
        }
        Err(e) => println!("Repository not found. {}", e),
    };

    // @note If I clone, then I'm just gonna use fs and treewalking to find the
    //       relevant gitignore, then output that. But that's not what I want

    // check gitignore if a file exists
    // if one exists, output to stdout
    // else output error "can't find gitignore for query"
}
