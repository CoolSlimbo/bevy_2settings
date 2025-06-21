//! # Configurators
//!
//! Configurators are used to configure settings.

mod toml;

use crate::{Settings, SettingsPlugin};
use bevy::prelude::{BevyError, Resource};
use std::marker::PhantomData;

pub mod prelude {
    //! The prelude re-export

    pub use crate::configurators::WithConfigurator;
    pub use crate::configurators::toml::TomlFileConfiguration;
}

/// Trait to allow for easily creating a configurator in a more ergonomic way.
pub trait WithConfigurator
where
    Self: Sized + Settings + Resource,
{
    /// Given a setting, create a configurator for it
    fn with_configurator<C: Configurator<Setting = Self>>(
        configurator: C,
    ) -> SettingsPlugin<Self, C>;
}

impl<S: Settings + Resource> WithConfigurator for S {
    fn with_configurator<C: Configurator<Setting = S>>(configurator: C) -> SettingsPlugin<S, C> {
        SettingsPlugin {
            config: configurator,
            auto_save: true,
            _settings: PhantomData,
        }
    }
}

/// Shared trait for all configurators
///
/// Is set to a specific setting config.
pub trait Configurator
where
    Self: Sync + Send + Default + Resource + Clone + 'static,
{
    /// The setting that the configurator is for
    type Setting: Settings;

    /// Saves the config provided
    fn save(&self, config: &Self::Setting) -> Result<(), BevyError>;

    /// Loads the config, as per [`Configurator::Setting`]
    fn load(&self) -> Result<<Self::Setting as Settings>::Partial, BevyError>;
}
