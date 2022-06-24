#![allow(dead_code)]
use std::{collections::HashMap, hash::Hash};

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

#[derive(Debug)]
struct Blob {
    sha: String,
    url: String,
    path: String,
}

async fn with_rest(term: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("MCA-Ignore-Tool")
        .build()?;

    let main_branch = get_branches(&client).await?;
    println!("{:#?}", main_branch);

    let tree = client
        .get(main_branch.url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?
        .get("tree".to_string())
        .unwrap()
        .to_owned()
        .as_array()
        .unwrap()
        .iter()
        .map(|x| Blob {
            sha: x["sha"].to_string().replace("\"", ""),
            url: x["url"].to_string().replace("\"", ""),
            path: x["path"].to_string().replace("\"", ""),
        })
        .collect::<Vec<Blob>>();

    println!("{:#?}", tree);

    return Ok(());
}

async fn get_branches(client: &reqwest::Client) -> Result<ShaUrl, Box<dyn std::error::Error>> {
    let res = client
        .get(format!("{}/repos/{}/{}/branches/main", BASE, OWNER, REPO))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    return Ok(ShaUrl {
        sha: res["commit"]["commit"]["tree"]["sha"]
            .to_string()
            .replace("\"", ""),
        url: res["commit"]["commit"]["tree"]["url"]
            .to_string()
            .replace("\"", ""),
    });
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
