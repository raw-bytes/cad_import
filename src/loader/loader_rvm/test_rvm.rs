#[cfg(test)]
mod test {
    use crate::loader::{loader_rvm::LoaderRVM, Loader, MemoryResource};

    #[test]
    fn test_loading_rvm() {
        let rvm_data = include_bytes!("../test_data/rvm/plm-sample_11072013.rvm");

        let r = MemoryResource::new(
            rvm_data.as_ref(),
            "application/vnd.aveva.pdm.rvm".to_owned(),
        );

        let loader = LoaderRVM::new();

        let cad_data = loader.read(&r).unwrap();
    }
}
