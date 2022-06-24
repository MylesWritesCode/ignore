use git2::Repository;

pub fn with_libgit(remote: &str) -> Result<(), git2::Error> {
    // run query against repo

    let repo = Repository::open("./tmp")?;
    let mut remote = repo.remote_anonymous(remote)?;
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
