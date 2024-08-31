use serde::{Deserialize, Serialize};
use reqwest::Result;
use crate::data::Architecture;
use crate::api_struct;

const ARCHS_URL: &str = "https://rdb.altlinux.org/api/site/all_pkgset_archs?branch=";

// Gets binary package archs list
pub async fn fetch_branch_archs(branch_name: &str) -> Result<Vec<Architecture>> {
    let branch_archs: api_struct::PkgsetArchs = reqwest::get(ARCHS_URL.to_owned() + branch_name)
        .await?
        .json()
        .await?;

    Ok(branch_archs.archs.into_iter().map(|a| a.arch).collect())
}
