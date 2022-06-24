#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{ArgEnum, Parser};

mod commands;
mod with;
use with::{libgit::*, rest_direct::*, rest_tree_walking::*};

#[derive(Parser)]
#[clap(name = "ignore")]
#[clap(author = "Myles <myles@themapletree.io>")]
#[clap(version = "0.1.0")]
#[clap(about = "Outputs the typical gitignore for a search term")]
struct Cli {
    query: String,
    #[clap(short = 'i')]
    #[clap(arg_enum)]
    implementation: Option<Implementation>,
}

#[derive(Clone, ArgEnum)]
enum Implementation {
    RestTreeWalking,
    RestDirect,
    Libgit,
}

const REMOTE: &str = "https://github.com/github/gitignore";
const BASE: &str = "https://api.github.com";
const OWNER: &str = "github";
const REPO: &str = "gitignore";
const BRANCH: &str = "main";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(implementation) = cli.implementation {
        match implementation {
            Implementation::RestTreeWalking => {
                // With a reqwest client going through the tree. Kinda gross but it works.
                with_rest_tree_walking(&cli.query, BASE, OWNER, REPO).await?
            }
            Implementation::RestDirect => {
                // With a reqwest client going directly to the query name
                with_rest_direct(&cli.query, BASE, OWNER, REPO, BRANCH).await?
            }
            Implementation::Libgit => todo!(),
        }
    } else {
        // Default implementation
        with_rest_tree_walking(&cli.query, BASE, OWNER, REPO).await?
    }

    return Ok(());
}
