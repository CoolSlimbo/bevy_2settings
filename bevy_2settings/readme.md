# bevy_2settings
[![Crates.io](https://img.shields.io/crates/v/bevy_2settings.svg)](https://crates.io/crates/bevy_2settings)
[![docs.rs](https://img.shields.io/docsrs/bevy_2settings/latest)](https://docs.rs/bevy_2settings/latest)

## Features
What `bevy_2settings` offers:
- An easy to setup, macro based settings system.
    - See the [example](#getting-started)!
- Dynamic range of configurators
- Fully documented, and panic free (minus plugin code, see [here](https://github.com/bevyengine/bevy/issues/2337).)
- TODO: UI components, that can be easily bound

## Supported Bevy Versions
| Bevy | bevy_2settings |
| ---- | -------------- |
| 0.16 | 0.0.1          |

## Getting Started
1. Add `bevy_2settings` to your `Cargo.toml` (`bevy_2settings = 0.0.1`).
2. Derive `Settings`, `Resource`, `Serialize` and `Reflect` on your settings struct.
3. Add a configurator to your plugin (or, add the SettingsPlugin raw).
```rust
use bevy::prelude::*;
use bevy_2settings::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Serialize, Deserialize, Debug, Reflect)]
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
        .add_systems(Update, (get_settings, space_pressed))
        .run();
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
```

## Macro Details
The macro can be used in the following ways:
```rust
#[derive(Settings)]
#[settings(rename = "New Internal Name", styling(...))]
struct Potato {
    #[settings(
        name = "",
        styling(...),
        default = Potatoes,
        default = || bevy_ok(Potatoes),
        default = false, // Allowed, given the field is a literal of some form,
        predicate = false, // Shares what default does.
        nested
    )]
    field: Potatoes
}

struct Potatoes;
```