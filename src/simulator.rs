/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 *
 * Provides the windows and physics simulation
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct SimulatorPlugin;

/// Cart sprite size
const PENDULUM_CART_HEIGHT: f32 = 40.0;
const PENDULUM_CART_WIDTH: f32 = PENDULUM_CART_HEIGHT * 1.618;

/// Pendulum size
const PENDULUM_WIDTH: f32 = 10.0;
const PENDULUM_HEIGHT: f32 = 100.0;

impl Plugin for SimulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_systems(Startup, setup_graphics);
        app.add_systems(Startup, setup_physics);
        app.add_systems(Update, bevy::window::close_on_esc);
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    let parent = commands.spawn(SpriteBundle{
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(PENDULUM_CART_WIDTH, PENDULUM_CART_HEIGHT, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(RigidBody::KinematicVelocityBased)
    .insert(Collider::cuboid(0.5, 0.5))
    .id();

    let joint = RevoluteJointBuilder::new().local_anchor2(Vec2::new(0.0, PENDULUM_HEIGHT));
    commands
            .spawn((
                SpriteBundle{
                    transform: Transform {
                        translation: Vec3::new(0.0, PENDULUM_HEIGHT, 0.0),
                        scale: Vec3::new(PENDULUM_WIDTH, PENDULUM_WIDTH, 1.0),
                        ..default()
                    },
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::cuboid(0.5, 0.5),
            ))
            .insert(ImpulseJoint::new(parent, joint));
}