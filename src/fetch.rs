use crate::api_struct;
use crate::data::Architecture;
use reqwest::Result;

const ARCHS_URL: &str = "https://rdb.altlinux.org/api/site/all_pkgset_archs?branch=";


/// Gets binary package archs list
///
/// # Example
///
/// ```
/// # use altpkgparser::fetch::fetch_branch_archs;
/// # use altpkgparser::data::Architecture;
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
    let branch_archs: api_struct::PkgsetArchs = reqwest::get(ARCHS_URL.to_owned() + branch_name)
        .await?
        .json()
        .await?;

    Ok(branch_archs.archs.into_iter().map(|a| a.arch).collect())
}
