use std::collections::{BTreeMap, BTreeSet};

use log::{debug, error};

use crate::{
    loader::{
        loader_rvm::{cad_data_creator::CADDataCreator, rvm_parser::RVMParser},
        ExtensionMap, Loader, Options, OptionsDescriptor, Resource,
    },
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

    fn read_with_options(
        &self,
        resource: &dyn Resource,
        _: Option<Options>,
    ) -> Result<CADData, Error> {
        let mut cad_creator = CADDataCreator::new();

        {
            let reader = resource.open()?;
            let mut parser = RVMParser::new(reader, &mut cad_creator);

            debug!("Start parsing {}...", resource.to_string());
            match parser.parse() {
                Ok(_) => {
                    debug!("Start parsing {}...DONE", resource.to_string());
                }
                Err(err) => {
                    error!("Start parsing {}...FAILED", resource.to_string());
                    error!("Parsing failed due to {}", err);

                    return Err(err);
                }
            }
        }

        debug!("Create cad data from the cad creator...");
        let cad_data = cad_creator.to_cad_data();
        debug!("Create cad data from the cad creator...DONE");

        Ok(cad_data)
    }

    fn get_loader_options(&self) -> Option<OptionsDescriptor> {
        todo!()
    }
}
