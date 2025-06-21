use crate::{Configurator, Settings};
use bevy::prelude::{BevyError, Resource};
use std::{
    fs::File,
    io::{Read, Write},
    marker::PhantomData,
    path::PathBuf,
};

#[derive(Debug, Resource)]
pub struct TomlFileConfiguration<C>(PathBuf, PhantomData<C>);

impl<C> Clone for TomlFileConfiguration<C> {
    fn clone(&self) -> Self {
        TomlFileConfiguration(self.0.clone(), PhantomData)
    }
}

impl<C> Default for TomlFileConfiguration<C> {
    fn default() -> Self {
        TomlFileConfiguration(PathBuf::from("settings.toml"), PhantomData)
    }
}

impl<C> Configurator for TomlFileConfiguration<C>
where
    C: Settings,
{
    type Setting = C;

    fn save(&self, config: &C) -> Result<(), BevyError> {
        let mut f = File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(self.0.clone())?;
        let conf = toml::to_string_pretty(&config)?;
        f.write_all(conf.as_bytes())?;
        Ok(())
    }

    fn load(&self) -> Result<C::Partial, BevyError> {
        let mut f = File::options()
            .read(true)
            .append(true) // Despite not needing reading, this is required to prevent errors
            .create(true)
            .truncate(false)
            .open(self.0.clone())?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents)?)
    }
}
