use crate::api_struct;
use crate::packages_handler::{Architecture, BranchPkgsHandler};
use reqwest::Result;

const PKGS_URL: &str = "https://rdb.altlinux.org/api/export/branch_binary_packages/";
const ARCHS_URL: &str = "https://rdb.altlinux.org/api/site/all_pkgset_archs?branch=";

/// Gets packages list by Branch name (and specific arch, if present)
///
/// # Example
///
/// ```
/// # use altpkgparser::fetch::fetch_branch_packages;
/// # use altpkgparser::packages_handler::Architecture;
/// #
/// # tokio_test::block_on(async {
/// let res = fetch_branch_packages("p10", Some(Architecture::from("x86_64")).as_ref()).await.unwrap();
/// let archs = res.architectures().collect::<Vec<_>>();
/// assert!(archs.contains(&&Architecture("x86_64".to_owned())));
/// assert!(!archs.contains(&&Architecture("aarch64".to_owned())));
/// assert!(!archs.contains(&&Architecture("noarch".to_owned())));
/// # });
/// ```
pub async fn fetch_branch_packages(
    branch_name: &str,
    arch: Option<&Architecture>,
) -> Result<BranchPkgsHandler> {
    let mut request_url = PKGS_URL.to_owned() + branch_name;
    if let Some(arch) = arch {
        // concretize for specific architecture
        request_url = request_url + "?arch=" + &arch.0;
    }

    let branch_pkgs: api_struct::BranchPkgsResponse =
        reqwest::get(request_url).await?.json().await?;

    Ok(BranchPkgsHandler::from_raw(branch_pkgs.packages))
}

/// Gets binary package archs list
///
/// # Example
///
/// ```
/// # use altpkgparser::fetch::fetch_branch_archs;
/// # use altpkgparser::packages_handler::Architecture;
/// #
/// # tokio_test::block_on(async {
/// let res = fetch_branch_archs("p10").await.unwrap();
///
/// assert!(res.contains(&Architecture("x86_64".to_owned())));
/// assert!(res.contains(&Architecture("aarch64".to_owned())));
/// assert!(res.contains(&Architecture("noarch".to_owned())));
/// # });
/// ```
pub async fn fetch_branch_archs(branch_name: &str) -> Result<Vec<Architecture>> {
    let branch_archs: api_struct::BranchArchs = reqwest::get(ARCHS_URL.to_owned() + branch_name)
        .await?
        .json()
        .await?;
    Ok(branch_archs.archs.into_iter().map(|a| a.arch).collect())
}
