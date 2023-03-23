use std::{
    collections::{BTreeSet, BinaryHeap, HashMap},
    rc::Rc,
};

use super::{
    loader::Loader, loader_gltf::LoaderGLTF, loader_off::LoaderOff, loader_rvm::LoaderRVM,
    ExtensionMap,
};

#[derive(Clone)]
struct LoaderEntry {
    pub loader: Rc<dyn Loader>,
    pub priority: u32,
}

impl LoaderEntry {
    /// Returns a new loader entry.
    ///
    /// # Arguments
    /// * `loader` - The loader to store in the loader entry.
    pub fn new(loader: Rc<dyn Loader>) -> Self {
        let priority = loader.get_priority();

        Self { loader, priority }
    }
}

impl Ord for LoaderEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for LoaderEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl PartialEq for LoaderEntry {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for LoaderEntry {}

/// A list of loaders sorted by priority
type LoaderList = BinaryHeap<LoaderEntry>;

/// A map of loaders
type LoaderMap = HashMap<String, LoaderList>;

/// The manager contains a list of loaders which can be searched by mime-types or file extensions.
pub struct Manager {
    /// The internal list of all loaders
    loader: Vec<Rc<dyn Loader>>,

    /// Map of all extensions supported by the manager
    map_ext: ExtensionMap,

    /// Map from file mime types to a list of loaders
    map_mime: LoaderMap,
}

impl Manager {
    /// Creates and returns a loader manager initialized with multiple loaders.
    pub fn new() -> Self {
        let mut result = Self::new_empty();

        // register loaders here...
        result.register_loader(Box::new(LoaderOff::new()));
        result.register_loader(Box::new(LoaderGLTF::new()));
        result.register_loader(Box::new(LoaderRVM::new()));

        result
    }

    /// Creates and returns a new empty loader manager
    pub fn new_empty() -> Self {
        Self {
            loader: Vec::new(),
            map_ext: ExtensionMap::new(),
            map_mime: HashMap::new(),
        }
    }

    /// Registers a new loader in the manager
    ///
    /// # Arguments
    /// * `loader` - The loader to register.
    pub fn register_loader(&mut self, loader: Box<dyn Loader>) {
        let mut ext_map = loader.as_ref().get_extensions_mime_type_map();
        let mime_types = loader.get_mime_types();

        // create reference counter of loader
        let loader: Rc<dyn Loader> = loader.into();
        let loader_entry = LoaderEntry::new(loader.clone());

        // register loader in the general loader list
        self.loader.push(loader);

        // update extensions map
        for (ext, new_mime_types) in ext_map.iter_mut() {
            let mime_types = self
                .map_ext
                .entry(ext.clone())
                .or_insert_with(|| BTreeSet::new());

            mime_types.append(new_mime_types);
        }

        // register loader based on its mime type
        for mim_type in mime_types.iter() {
            let loader_list = self
                .map_mime
                .entry(mim_type.clone())
                .or_insert_with(|| LoaderList::new());

            loader_list.push(loader_entry.clone());
        }
    }

    /// Tries to find mime_types associated to the given extension.
    ///
    /// # Arguments
    /// * `ext` - The extension of the loader without a preceding dot, e.g. "png".
    pub fn get_mime_types_for_extension(&self, ext: &str) -> Vec<String> {
        let ext = ext.to_lowercase();

        match self.map_ext.get(&ext) {
            Some(lst) => Vec::from_iter(lst.iter().map(|s| s.clone())),
            None => Vec::new(),
        }
    }

    /// Tries to find a loader by its mime type.
    ///
    /// # Arguments
    /// * `mime_type` - The mime type of the loader, e.g. "image/png".
    pub fn get_loader_by_mime_type(&self, mime_type: &str) -> Option<Rc<dyn Loader>> {
        let mime_type = mime_type.to_lowercase();

        match self.map_mime.get(&mime_type) {
            Some(lst) => {
                let e = match lst.peek() {
                    Some(l) => Some(l.loader.clone()),
                    None => None,
                };

                e
            }
            None => None,
        }
    }

    /// Returns reference onto the internal list of loader
    pub fn get_loader_list(&self) -> &[Rc<dyn Loader>] {
        &self.loader
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{loader::Resource, structure::CADData, Error};

    use super::*;

    struct FakeLoader {
        identifier: String,
        map_ext: ExtensionMap,
        mime_types: Vec<String>,
        priority: u32,
    }

    impl FakeLoader {
        pub fn new(
            identifier: String,
            map_ext: ExtensionMap,
            mime_types: Vec<String>,
            priority: u32,
        ) -> Self {
            Self {
                identifier,
                map_ext,
                mime_types,
                priority,
            }
        }
    }

    impl Loader for FakeLoader {
        fn get_mime_types(&self) -> Vec<String> {
            self.mime_types.clone()
        }

        fn get_extensions_mime_type_map(&self) -> ExtensionMap {
            self.map_ext.clone()
        }

        fn get_priority(&self) -> u32 {
            self.priority
        }

        fn get_loader_options(&self) -> Option<crate::loader::OptionsDescriptor> {
            None
        }

        fn read_with_options(
            &self,
            _: &dyn Resource,
            _: Option<crate::loader::Options>,
        ) -> Result<CADData, Error> {
            todo!()
        }

        fn get_name(&self) -> &str {
            &self.identifier
        }
    }

    #[test]
    fn test_loader_registry() {
        let mut m = Manager::new_empty();

        let l = FakeLoader::new(
            "loader1".to_owned(),
            BTreeMap::from([(
                "foobar".to_owned(),
                BTreeSet::from(["foobar/x-test".to_owned()]),
            )]),
            vec!["foobar/x-test".to_owned()],
            42,
        );
        m.register_loader(Box::new(l));

        assert_eq!(m.get_mime_types_for_extension("foobar"), ["foobar/x-test"]);
        assert_eq!(m.get_mime_types_for_extension("FOobar"), ["foobar/x-test"]);
        assert!(m.get_mime_types_for_extension("FOobar2").is_empty());
        assert!(m.get_mime_types_for_extension("FOob").is_empty());

        let l2 = FakeLoader::new(
            "loader2".to_owned(),
            BTreeMap::from([(
                "foobar".to_owned(),
                BTreeSet::from(["foobar/x-test".to_owned()]),
            )]),
            vec!["foobar/x-test".to_owned()],
            43,
        );

        m.register_loader(Box::new(l2));

        assert_eq!(m.get_mime_types_for_extension("foobar"), ["foobar/x-test"]);
        assert_eq!(m.get_mime_types_for_extension("FOobar"), ["foobar/x-test"]);
        assert!(m.get_mime_types_for_extension("FOobar2").is_empty());
        assert!(m.get_mime_types_for_extension("FOob").is_empty());

        assert_eq!(
            m.get_loader_by_mime_type("foobar/x-test")
                .unwrap()
                .get_priority(),
            43
        );

        assert_eq!(
            m.get_loader_by_mime_type("foobar/x-test")
                .unwrap()
                .get_name(),
            "loader2"
        );

        assert_eq!(m.get_loader_list().len(), 2);
    }

    #[test]
    fn test_if_loaders_are_registered() {
        let manager = Manager::new();

        let off_mime_types = manager.get_mime_types_for_extension("off");
        assert_eq!(off_mime_types.len(), 1);
        let loader = manager.get_loader_by_mime_type(&off_mime_types[0]).unwrap();
        assert_eq!(loader.get_name(), "Object File Format");

        let loader = manager.get_loader_by_mime_type("model/vnd.off").unwrap();
        assert_eq!(loader.get_name(), "Object File Format");

        let loaders = manager.get_loader_list();
        assert_eq!(loaders.len(), 2);
    }

    #[test]
    fn test_extension_map() {
        let manager = Manager::new();

        for loader in manager.get_loader_list() {
            // create list of all mime types based on the extension map
            let mut mime_types_set: BTreeSet<String> = BTreeSet::new();
            let mut ext_map = loader.get_extensions_mime_type_map();
            for m in ext_map.values_mut() {
                mime_types_set.append(m);
            }

            let mut mime_types: Vec<String> = loader.get_mime_types();
            mime_types.sort();

            assert_eq!(mime_types.len(), mime_types_set.len());

            for m in mime_types.iter() {
                assert!(mime_types_set.contains(m));
            }
        }
    }
}
