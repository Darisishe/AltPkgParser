use crate::api_struct;
use crate::packages_handler::BranchPkgsHandler;
use reqwest::Result;

const PKGS_URL: &str = "https://rdb.altlinux.org/api/export/branch_binary_packages/";


/// Gets packages list by Branch name
///
/// # Example
///
/// ```
/// # use altpkgparser::fetch::fetch_branch_packages;
/// # use altpkgparser::packages_handler::Architecture;
/// #
/// # tokio_test::block_on(async {
/// let res = fetch_branch_packages("p10").await.unwrap();
/// let archs = res.architectures().collect::<Vec<_>>();
/// assert!(archs.contains(&&Architecture("x86_64".to_owned())));
/// assert!(archs.contains(&&Architecture("aarch64".to_owned())));
/// assert!(archs.contains(&&Architecture("noarch".to_owned())));
/// # });
/// ```
pub async fn fetch_branch_packages(branch_name: &str) -> Result<BranchPkgsHandler> {
    let branch_pkgs: api_struct::BranchPkgsResponse = reqwest::get(PKGS_URL.to_owned() + branch_name)
        .await?
        .json()
        .await?;

    Ok(BranchPkgsHandler::from_raw(branch_pkgs.packages))
}
