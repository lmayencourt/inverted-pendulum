/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

mod user_interface;
mod simulator;
mod visual_effects;

use user_interface::UserInterfacePlugin;
use simulator::SimulatorPlugin;
use visual_effects::VisualEffectsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimulatorPlugin)
        .add_plugins(VisualEffectsPlugin)
        .add_plugins(UserInterfacePlugin)
        .run();
}
