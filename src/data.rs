use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use serde::{Deserialize, Serialize};

use crate::api_struct::{BranchPkgsRaw, PkgRaw};

#[derive(Debug)]
pub struct BranchPkgsHandler {
    arch_packages: HashMap<Architecture, HashSet<PkgEntry>>,
}

impl BranchPkgsHandler {
    pub(crate) fn from_raw(packages: Vec<PkgRaw>) -> BranchPkgsHandler {
        let mut arch_packages: HashMap<_, HashSet<_>> = HashMap::new();
        for pkg in packages {
            arch_packages.entry(pkg.arch).or_default().insert(PkgEntry {
                name: pkg.name,
                version: pkg.version,
                release: pkg.release,
            });
        }

        BranchPkgsHandler { arch_packages }
    }

    pub fn architectures(&self) -> impl Iterator<Item = &Architecture> {
        self.arch_packages.keys()
    }
}


#[derive(Debug)]
struct PkgEntry {
    name: String,
    version: String,
    release: String,
}

impl PartialEq for PkgEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for PkgEntry {}

impl Hash for PkgEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// aliases Arch as String
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Architecture(pub String);
