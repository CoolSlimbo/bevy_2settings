#![deny(clippy::unwrap_used)]
#![forbid(missing_docs)]
#![doc = include_str!("../../readme.md")]

mod configurators;

use bevy::ecs::{error::BevyError, world::World};
use bevy::{
    app::{Plugin, PreUpdate},
    ecs::{
        event::{Event, EventReader},
        resource::Resource,
        schedule::{
            Condition, IntoScheduleConfigs,
            common_conditions::{not, resource_added, resource_changed},
        },
        system::Res,
    },
    log::tracing,
    reflect::{GetPath, PartialReflect},
};
use bevy_2settings_macro::Settings;
use configurators::Configurator;
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;

pub mod prelude {
    //! The prelude re-export

    // pub use crate::Settings;
    pub use crate::DynamicSetting;
    pub use crate::bevy_ok;
    pub use crate::{Field, Meta, Styling};
    pub use bevy_2settings_macro::Settings;

    pub use bevy::ecs::{error::BevyError, system::RunSystemOnce, world::World};

    pub use crate::configurators::prelude::*;

    pub use serde::Deserialize;
}

/// Simple wrapper function for bevy's `Ok`
pub fn bevy_ok<T>(t: T) -> Result<T, BevyError> {
    Ok(t)
}

/// The main trait for settings
///
/// Automatically implemented with the [`Settings`] macro, users should never need to manually implement this.
/// See trait documentation if you're insane enough to manually implement this.
pub trait Settings
where
    Self: Serialize + Sized + Sync + Send + 'static,
{
    /// Metadata for the settings
    ///
    /// See [`Meta`]
    const META: Meta;
    /// The internal name of the settings
    ///
    /// Can either be renamed, or is just the name of the struct
    const INTERNAL_NAME: &'static str;

    /// The partial struct for the settings
    ///
    /// This partial version has every type wrapped in an option
    type Partial: DeserializeOwned;
    /// Controls how the settings are displayed for UI uses.
    type ViewNode;

    /// Takes a partial version of the settings and returns the full version
    ///
    /// This works by turning all Some values into actual values, and all None values into either [`Default::default()] or [`RunSystemOnce::run_system_once`], using the world.
    fn from_partial(partial: Self::Partial, world: &mut World) -> Result<Self, BevyError>;

    /// Takes a config, and generates a view tree for it
    ///
    /// Utilises [`RunSystemOnce::run_system_once`] to run the systems
    fn get_view_tree(&self, world: &mut World) -> Result<Self::ViewNode, BevyError>;
}

/// Trait to allow for dynamic settings
pub trait DynamicSetting {
    /// Sets the value of a field
    fn set_value(
        &mut self,
        name: &'static str,
        value: &dyn PartialReflect,
    ) -> Result<(), BevyError>;

    /// Gets the value of a field
    fn get_value(&self, name: impl AsRef<str>) -> Option<&dyn PartialReflect>;
}

impl<S> DynamicSetting for S
where
    S: Settings + GetPath,
{
    fn get_value(&self, name: impl AsRef<str>) -> Option<&dyn PartialReflect> {
        let val = self.reflect_path(name.as_ref());
        val.ok()
    }

    fn set_value(
        &mut self,
        name: &'static str,
        value: &dyn PartialReflect,
    ) -> Result<(), BevyError> {
        let val = self.reflect_path_mut(name)?;
        val.try_apply(value)?;
        Ok(())
    }
}

/// The metadata for the settings
#[derive(Debug, Serialize)]
pub struct Meta {
    /// This is the stylings implemented for a settings itself.
    pub self_stylings: &'static [Styling],
    /// This is the fields of the settings
    pub fields: &'static [Field],
}

/// The metadata for a field
#[derive(Debug, Serialize)]
pub struct Field {
    /// The display name.
    ///
    /// Defaults to the field name, but can be renamed.
    pub name: &'static str,
    /// A shortened name, used for internal references.
    ///
    /// This is how the actual field is.
    pub internal_name: &'static str,
    /// The description of the field
    pub description: &'static str,
    /// The stylings of the field
    pub styling: &'static [Styling],
    /// If the field is nested
    pub nested: bool,
}

/// List of stylings
///
/// TODO: Change this to something dynamic
#[derive(Debug, Serialize)]
pub enum Styling {}

/// Event triggered to allow saving a config
#[derive(Event)]
pub struct SaveConfigEvent<C>(PhantomData<C>);

/// The main plugin used for settings
///
/// Ideally added using [`configurators::WithConfigurator`]
#[derive(Debug)]
pub struct SettingsPlugin<S: Settings, C: Configurator<Setting = S>> {
    config: C,
    auto_save: bool,
    _settings: PhantomData<S>,
}

impl<S: Settings + Resource, C: Configurator<Setting = S>> SettingsPlugin<S, C> {
    /// Enables or disables auto-saving
    pub fn using_autosave(self, auto_save: bool) -> Self {
        SettingsPlugin {
            config: self.config,
            auto_save,
            _settings: PhantomData,
        }
    }

    /// Saves the config, based on the event
    fn save_config(
        mut ev: EventReader<SaveConfigEvent<C>>,
        settings: Res<C::Setting>,
        configurator: Res<C>,
    ) -> Result<(), BevyError> {
        for _ in ev.read() {
            configurator.save(&settings)?;
        }
        Ok(())
    }

    /// Saves the config automatically when the settings change
    fn save_config_on_auto(
        settings: Res<C::Setting>,
        configurator: Res<C>,
    ) -> Result<(), BevyError> {
        configurator.save(&settings)?;
        tracing::info!("Saving config for {}", S::INTERNAL_NAME);
        Ok(())
    }
}

impl<S: Settings, C: Configurator<Setting = S>> Default for SettingsPlugin<S, C>
where
    C: Default,
{
    fn default() -> Self {
        SettingsPlugin {
            config: C::default(),
            auto_save: true,
            _settings: PhantomData,
        }
    }
}

impl<S: Settings + Resource, C: Configurator<Setting = S>> Plugin for SettingsPlugin<S, C> {
    #[allow(clippy::unwrap_used)]
    fn build(&self, app: &mut bevy::app::App) {
        let partial_conf = self.config.load().unwrap();
        // Convert this partial config to a full config
        let config = <S as Settings>::from_partial(partial_conf, app.world_mut()).unwrap();

        // Insert the config, as a resource
        app.insert_resource(config);
        // Insert the configurator as a resource
        app.insert_resource(self.config.clone());

        // TODO: Introduce systems triggered on modification, that allow controlling saving, etc

        // Save event handler
        app.add_event::<SaveConfigEvent<C>>();
        app.add_systems(PreUpdate, Self::save_config);

        // If auto-save is enabled, add a system to save on change
        if self.auto_save {
            app.add_systems(
                PreUpdate,
                Self::save_config_on_auto
                    .run_if(resource_changed::<S>.and(not(resource_added::<S>))),
            );
        }
    }
}
