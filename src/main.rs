/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

mod user_interface;
mod regulator;
mod simulator;
mod visual_effects;

use user_interface::UserInterfacePlugin;
use regulator::RegulatorPlugin;
use simulator::SimulatorPlugin;
use visual_effects::VisualEffectsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimulatorPlugin)
        .add_plugins(VisualEffectsPlugin)
        .add_plugins(UserInterfacePlugin)
        .add_plugins(RegulatorPlugin)
        .run();
}
