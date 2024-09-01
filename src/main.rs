use altpkgparser::fetch;

#[tokio::main]
async fn main() {
    if let Ok(pkgs) = fetch::fetch_branch_packages("p10").await {
        for arch in pkgs.architectures() {
            println!("{}", arch.0);
        }
    } else {
        dbg!(fetch::fetch_branch_packages("p10").await);
    }
}
