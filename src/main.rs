/**
 * This is the main driver code for the starter.
 * Run with `cargo run` or `<project_name>` to see the auto-generated help text.
 */
use clap::Parser;
use git2::Repository;

mod commands;

#[derive(Parser)]
#[clap(name = "ignore")]
#[clap(author = "Myles <myles@themapletree.io>")]
#[clap(version = "0.1.0")]
#[clap(about = "Outputs the typical gitignore for a search term")]

struct Cli {
    query: String,
}

const REMOTE: &str = "https://github.com/github/gitignore";

fn main() -> Result<(), git2::Error> {
    let cli = Cli::parse();

    // get query
    println!("{}", cli.query);
    
    // @note I mean, there's also the github public API that's much, much easier
    //       to use :)

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

    // check gitignore if a file exists
    // if one exists, output to stdout
    // else output error "can't find gitignore for query"
    return Ok(());
}
