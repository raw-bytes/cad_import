use std::collections::HashMap;

use super::{GeneralOptions, OptionsDescriptor, OptionsGroup};

/// The overall set of options provided to a loader.
#[derive(Clone)]
pub struct Options {
    /// general options that apply to all loaders.
    general_options: GeneralOptions,

    /// Options for different loaders. Options for multiple loaders can be
    /// important as following links will be a future feature.
    /// The key is the respective options descriptor ids.
    loader_options: HashMap<u32, OptionsGroup>,
}

impl Options {
    /// Creates and returns a new options object based on the provided options.
    ///
    /// # Argument
    /// * `general_options` - Special category of options that apply to all loaders
    pub fn new(general_options: GeneralOptions) -> Self {
        Self {
            general_options,
            loader_options: HashMap::new(),
        }
    }

    /// Returns a reference onto the general options.
    pub fn get_general_options(&self) -> &GeneralOptions {
        &self.general_options
    }

    /// Adds options values for a specific loader.
    ///
    /// # Arguments
    /// * `options` - The options for the loader to be set
    pub fn add_loader_option_values(&mut self, options: OptionsGroup) {
        let id = options.get_descriptor().get_id();
        self.loader_options.insert(id, options);
    }

    /// Returns option values for the provided options descriptor.
    ///
    /// # Arguments
    /// * `descriptor` - The descriptor for which the options will be returned.
    pub fn get_loader_option_values(&self, descriptor: &OptionsDescriptor) -> OptionsGroup {
        match self.loader_options.get(&descriptor.get_id()) {
            Some(option_values) => option_values.clone(),
            None => {
                let option_values = OptionsGroup::new(descriptor.clone());
                option_values
            }
        }
    }
}
