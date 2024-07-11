/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 *
 * Add some juice to the graphics.
 */

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use crate::simulator::{Pendulum, PENDULUM_RADIUS, COLOR_ORANGE};

pub struct VisualEffectsPlugin;

/// Component to manage the trails
#[derive(Component)]
struct GohstTrail {
    life: u32,
}

impl Plugin for VisualEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_ghost_trail);
    }    
}

/// Draw a trail of the past position of the pendulum
fn draw_ghost_trail(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    pendulum_query: Query<&Transform, With<Pendulum>>,
    mut trail_query: Query<(&mut GohstTrail, &mut Transform, Entity), Without<Pendulum>>,
) {
    let pendulum_transform = pendulum_query.single();

    // Handle existing trails
    for (mut trail, mut transform, entity) in &mut trail_query {
        if trail.life > 0 {
            trail.life -= 1;
            transform.scale.x *= 0.9;
            transform.scale.y *= 0.9;
        } else {
            command.entity(entity).despawn();
        }
    }

    // Spawn a new trail at the current pendulum position
    command.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle{radius: PENDULUM_RADIUS})),
        material: materials.add(COLOR_ORANGE),
        transform: pendulum_transform.clone(),
        ..default()
    })
    .insert(GohstTrail{life: 20});
}