pub mod configurators;

pub use bevy::ecs::{error::BevyError, system::RunSystemOnce, world::World};
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
pub use bevy_2settings_macro::Settings;
pub use configurators::Configurator;
pub use serde::Deserialize;
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;

pub fn bevy_ok<T>(t: T) -> Result<T, BevyError> {
    Ok(t)
}

pub trait Settings
where
    Self: Serialize + Sized + Sync + Send + 'static,
{
    const META: Meta;
    const INTERNAL_NAME: &'static str;

    type Partial: DeserializeOwned;
    type ViewNode;

    fn from_partial(partial: Self::Partial, world: &mut World) -> Result<Self, BevyError>;

    fn get_view_tree(&self, world: &mut World) -> Result<Self::ViewNode, BevyError>;
}

pub trait DynamicSetting {
    fn set_value(
        &mut self,
        name: &'static str,
        value: &dyn PartialReflect,
    ) -> Result<(), BevyError>;

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

#[derive(Debug, Serialize)]
pub struct Meta {
    pub self_stylings: &'static [Styling],
    pub fields: &'static [Field],
}

#[derive(Debug, Serialize)]
pub struct Field {
    pub name: &'static str,
    pub internal_name: &'static str,
    pub description: &'static str,
    pub styling: &'static [Styling],
    pub nested: bool,
}

#[derive(Debug, Serialize)]
pub enum Styling {}

#[derive(Event)]
pub struct SaveConfigEvent<C>(PhantomData<C>);

// Plugin Shit
#[derive(Debug)]
pub struct SettingsPlugin<S: Settings, C: Configurator<Setting = S>> {
    config: C,
    auto_save: bool,
    _settings: PhantomData<S>,
}

impl<S: Settings + Resource, C: Configurator<Setting = S>> SettingsPlugin<S, C> {
    pub fn using_autosave(self, auto_save: bool) -> Self {
        SettingsPlugin {
            config: self.config,
            auto_save,
            _settings: PhantomData,
        }
    }

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
