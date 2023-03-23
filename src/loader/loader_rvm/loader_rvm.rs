use std::collections::{BTreeMap, BTreeSet};

use crate::{
    loader::{ExtensionMap, Loader},
    structure::CADData,
    Error,
};

/// Loader for the AVEVA PDMS binary RVM format. (see https://en.wikipedia.org/wiki/PDMS_(software))
pub struct LoaderRVM {}

impl LoaderRVM {
    pub fn new() -> Self {
        Self {}
    }
}

impl Loader for LoaderRVM {
    fn get_name(&self) -> &str {
        "AVEVA PDMS binary RVM"
    }

    fn get_mime_types(&self) -> Vec<String> {
        vec!["application/vnd.aveva.pdm.rvm".to_owned()]
    }

    fn get_extensions_mime_type_map(&self) -> ExtensionMap {
        let mut ext_map = BTreeMap::new();

        ext_map.insert(
            "rvm".to_owned(),
            BTreeSet::from(["application/vnd.aveva.pdm.rvm".to_owned()]),
        );

        ext_map
    }

    fn get_priority(&self) -> u32 {
        1000
    }

    fn read(&self, resource: &dyn crate::loader::Resource) -> Result<CADData, Error> {
        todo!()
    }
}
