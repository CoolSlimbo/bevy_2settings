use bevy::prelude::*;
use bevy_2settings::{
    BevyError, DynamicSetting, Settings, bevy_ok,
    configurators::{TomlFileConfiguration, WithConfigurator as _},
};
use serde::{Deserialize, Serialize};
use strum::VariantArray;

#[derive(Settings, Resource, Serialize, Debug, Reflect)]
struct Test {
    /// Testing Description!
    ///
    /// NEW LINE!!
    #[settings(name = "Big Screen", default = test_system, predicate = pred)]
    fullscreen: bool,
    #[settings(name = "Nested Test", nested)]
    nested: NestedTest,
}

#[derive(Settings, Serialize, Debug, Reflect)]
struct NestedTest {
    #[settings(name = "No Screen")]
    anti_screen: bool,
    #[settings(name = "Control Selector", default = || bevy_ok(ControlSelector::Keyboard))]
    control_selector: ControlSelector,
}

#[derive(Clone, Serialize, Deserialize, Debug, Reflect, VariantArray)]
enum ControlSelector {
    Keyboard,
    Mouse,
}

fn test_system() -> Result<bool, BevyError> {
    Ok(true)
}

fn pred() -> Result<bool, BevyError> {
    Ok(true)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Test::with_configurator(TomlFileConfiguration::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (get_settings, space_pressed))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn get_settings(settings: Res<Test>) {
    println!("{settings:?}");
}

fn space_pressed(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Test>) {
    if keys.just_pressed(KeyCode::Space) {
        settings.nested.anti_screen = !settings.nested.anti_screen;
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        settings
            .set_value("nested.control_selector", &ControlSelector::Mouse)
            .unwrap();
    }
    if keys.just_released(KeyCode::ArrowRight) {
        settings
            .set_value("nested.control_selector", &ControlSelector::Keyboard)
            .unwrap();
    }
}

/*
spawn(Checkbox<Test>::bind("fullscreen"))
spawn(Options<ControlSelector>::bind("nested.control_selector"))
*/

// UI Options:
// `bevy_ui` based autogenerator.
// Function is executed, and resultingly generates the UI structure as a component tree, to be spawned onto an entity.
// Base level version has no customisation; but automatically adds everything it needs. Requires a plugin added, that handles joint interactions
// Automatically handles changing the resource.
// Requires custom components for: hotkeys, toggles, dropdown, sliders, dual view fields, etc.
// Supports an array of different types of inputs.

// `bevy_lunex` option.
// More like a styleable UI component library, with some utilities.
