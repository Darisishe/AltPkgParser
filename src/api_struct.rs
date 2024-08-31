use serde::Deserialize;
use crate::data;

#[derive(Deserialize, Debug)]
pub struct ArchData {
    pub arch: data::Architecture,
    count: usize,
}

#[derive(Deserialize, Debug)]
pub struct PkgsetArchs {
    length: usize,
    pub archs: Vec<ArchData>,
}
