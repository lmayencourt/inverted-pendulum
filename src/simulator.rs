/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 *
 * Provides the windows and physics simulation
 */

use bevy::prelude::*;

pub struct SimulatorPlugin;

/// Cart sprite size
const PENDULUM_CART_WIDTH: f32 = 40.0;
const PENDULUM_CART_HEIGHT: f32 = PENDULUM_CART_WIDTH * 1.618;

impl Plugin for SimulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_graphics);
        app.add_systems(Startup, setup_physics);
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    commands.spawn(SpriteBundle{
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(PENDULUM_CART_HEIGHT, PENDULUM_CART_WIDTH, 1.0),
            ..default()
        },
        ..default()
    });
}