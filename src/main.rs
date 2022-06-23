#![allow(dead_code)]
use clap::Parser;
use git2::Repository;
use serde::Deserialize;

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
const BASE: &str = "https://api.github.com";
const OWNER: &str = "github";
const REPO: &str = "gitignore";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // get query
    println!("{}", cli.query);

    // @note I mean, there's also the github public API that's much, much easier
    //       to use :)

    with_rest(&cli.query).await?;

    // check gitignore if a file exists
    // if one exists, output to stdout
    // else output error "can't find gitignore for query"
    return Ok(());
}

#[derive(Clone, Debug, Deserialize)]
struct ShaUrl {
    sha: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Branch {
    commit: ShaUrl,
    name: String,
    protected: bool,
}

async fn with_rest(term: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("MCA-Ignore-Tool")
        .build()?;

    let branch = get_branches(&client).await?;

    let res = client
        .get(&branch.url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let commit_url = serde_json::to_string_pretty(&res["commit"]["tree"]["url"])?;

    println!("{}", commit_url);
    // println!("{:#?}", &res);
    return Ok(());
}

async fn get_branches(client: &reqwest::Client) -> Result<ShaUrl, Box<dyn std::error::Error>> {
    let res = client
        .get(format!("{}/repos/{}/{}/branches", BASE, OWNER, REPO))
        .send()
        .await?
        // .json::<serde_json::Value>()
        .json::<Vec<Branch>>()
        .await?;

    let branch = res
        .iter()
        .filter(|&branch| branch.name == "main")
        .collect::<Vec<&Branch>>()
        .get(0)
        .expect("Main branch not found")
        .commit
        .to_owned();

    return Ok(branch);
}

fn get_files() -> Vec<String> {
    todo!()
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
