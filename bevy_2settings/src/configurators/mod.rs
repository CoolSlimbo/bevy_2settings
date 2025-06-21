mod toml;

use crate::{Settings, SettingsPlugin};
use bevy::prelude::{BevyError, Resource};
use std::marker::PhantomData;
pub use toml::TomlFileConfiguration;

pub trait WithConfigurator
where
    Self: Sized + Settings + Resource,
{
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

pub trait Configurator
where
    Self: Sync + Send + Default + Resource + Clone + 'static,
{
    type Setting: Settings;
    // TODO: !
    // Has to save, and load settings
    // Save signature = fn save(&self, config: impl Serialize) -> Result<(), BevyError>;
    // Load signature = fn load(&self) -> Result<impl Deserialize, BevyError>;

    fn save(&self, config: &Self::Setting) -> Result<(), BevyError>;

    fn load(&self) -> Result<<Self::Setting as Settings>::Partial, BevyError>;
}
