use crate::packages_handler;
use serde::Deserialize;

/////////////////////////////////////////////////////////////////////////////////////////
/// describes the response of "/export/branch_binary_packages/"

#[derive(Deserialize, Debug)]
pub struct BranchPkgsResponse {
    pub packages: Vec<PkgRaw>,

    // unused
    request_args: serde_json::Value,
    length: usize,
}

/// Contains all data responsed by API
#[derive(Deserialize, Debug)]
pub struct PkgRaw {
    pub arch: packages_handler::Architecture,
    pub name: String,
    pub epoch: i32,
    pub version: String,
    pub release: String,

    // unused
    disttag: String,
    buildtime: usize,
    source: String,
}
