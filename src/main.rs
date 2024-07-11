/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

use bevy::prelude::*;

mod simulator;
mod visual_effects;

use simulator::SimulatorPlugin;
use visual_effects::VisualEffectsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimulatorPlugin)
        .add_plugins(VisualEffectsPlugin)
        .run();
}
