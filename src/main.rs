use std::arch;

use altpkgparser::fetch;

#[tokio::main]
async fn main() {
    if let Ok(archs) = fetch::fetch_branch_archs("p10").await {
        for arch in archs {
            println!("{}", arch.0);
        }
    } else {
        dbg!(fetch::fetch_branch_archs("p10").await);
    }
}
