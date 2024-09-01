use serde::Deserialize;
use crate::data;

/////////////////////////////////////////////////////////////////////////////////////////
// describes the response of "/export/branch_binary_packages/"

#[derive(Deserialize, Debug)]
pub struct BranchPkgsRaw {
    pub packages: Vec<PkgRaw>,

    // unused
    request_args: serde_json::Value,
    length: usize,
}

#[derive(Deserialize, Debug)]
pub struct PkgRaw {
    pub arch: data::Architecture,
    pub name: String,
    pub version: String,
    pub release: String,

    // unused
    epoch: i32,
    disttag: String,
    buildtime: usize,
    source: String,
}

