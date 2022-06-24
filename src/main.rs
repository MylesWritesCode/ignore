#![allow(dead_code)]
#![allow(unused_variables)]

use clap::{ArgEnum, Parser};
use git2::Repository;

mod commands;
mod with;
use with::{rest_direct::*, rest_tree_walking::*};

#[derive(Parser)]
#[clap(name = "ignore")]
#[clap(author = "Myles <myles@themapletree.io>")]
#[clap(version = "0.1.0")]
#[clap(about = "Outputs the typical gitignore for a search term")]
struct Cli {
    query: String,
    #[clap(short = 'i')]
    #[clap(arg_enum)]
    implementation: Implementation,
}

#[derive(Clone, ArgEnum)]
enum Implementation {
    RestTreeWalking,
    RestDirect,
}

const REMOTE: &str = "https://github.com/github/gitignore";
const BASE: &str = "https://api.github.com";
const OWNER: &str = "github";
const REPO: &str = "gitignore";
const BRANCH: &str = "main";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Implementation::RestTreeWalking = cli.implementation {
        // With a reqwest client going through the tree. Kinda gross but it works.
        with_rest_tree_walking(&cli.query, BASE, OWNER, REPO).await?
    } else {
        with_rest_direct(&cli.query, BASE, OWNER, REPO, BRANCH).await?
    }

    return Ok(());
}

fn with_libgit() -> Result<(), git2::Error> {
    // run query against repo

    // let repo = match Repository::open(REMOTE) {
    let repo = Repository::open("./tmp")?;
    let mut remote = repo.remote_anonymous(REMOTE)?;
    let mut connection = remote.connect_auth(git2::Direction::Fetch, None, None)?;
    println!("{}", connection.remote().url().unwrap());

    let remote_head = connection.list()?.first().unwrap();

    // let ac = repo.find_annotated_commit(head.oid())?;

    // @notes Need to somehow simulate `git ls-tree --full-tree <sha at head>`

    // let tree = repo.revparse_single("HEAD^{tree}")?;
    // let tree = repo.find_commit(head.oid())?;

    // println!("{:?}", tree);
    // println!("commit: {:?}", head.oid());

    // @note If I clone, then I'm just gonna use fs and treewalking to find the
    //       relevant gitignore, then output that. But that's not what I want
    //
    return Ok(());
}
