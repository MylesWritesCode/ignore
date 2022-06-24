use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ShaUrl {
    sha: String,
    url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Blob {
    sha: String,
    url: String,
    path: String,
}

pub async fn with_rest_tree_walking(
    term: &str,
    base: &str,
    owner: &str,
    repo: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("MCA-Ignore-Tool")
        .build()?;

    let main_branch = get_branches(&client, base, owner, repo).await?;

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

    let blob = get_file(&tree, term);

    let res = client
        .get(blob.url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    if let Some(content) = res["content"].as_str() {
        let content = base64::decode(&content.replace("\n", ""))?;
        let decoded = std::str::from_utf8(&content)?.to_string();

        println!("{}", decoded);
    }

    return Ok(());
}

async fn get_branches(
    client: &reqwest::Client,
    base: &str,
    owner: &str,
    repo: &str,
) -> Result<ShaUrl, Box<dyn std::error::Error>> {
    let res = client
        .get(format!("{}/repos/{}/{}/branches/main", base, owner, repo))
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

fn get_file(tree: &Vec<Blob>, term: &str) -> Blob {
    return tree
        .iter()
        .find(|&blob| blob.path.to_lowercase().contains(&term.to_lowercase()))
        .unwrap()
        .to_owned();
}
