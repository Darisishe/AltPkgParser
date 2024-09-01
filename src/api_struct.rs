use serde::Deserialize;
use crate::packages_handler;

/////////////////////////////////////////////////////////////////////////////////////////
// describes the response of "/export/branch_binary_packages/"

#[derive(Deserialize, Debug)]
pub struct BranchPkgsResponse {
    pub packages: Vec<PkgRaw>,

    // unused
    request_args: serde_json::Value,
    length: usize,
}

#[derive(Deserialize, Debug)]
pub struct PkgRaw {
    pub arch: packages_handler::Architecture,
    pub name: String,
    pub version: String,
    pub release: String,

    // unused
    epoch: i32,
    disttag: String,
    buildtime: usize,
    source: String,
}

