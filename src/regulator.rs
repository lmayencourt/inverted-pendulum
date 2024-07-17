/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 *
 * Provides a PID regulator for the pendulum
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::simulator::{Pendulum, Cart, MOVING_FORCE};
// use crate::user_interface::PendulumHistory;

/// Component to keep track of pendulum erros
#[derive(Component)]
pub struct RegulatorCoefficient {
    pub proportional: f32,
    pub integral: f32,
    pub derivative: f32,
    cumulative_error: f32,
    last_error: f32,
}

/// Component tag to retrieve the tilt regulation entity
#[derive(Component)]
pub struct TiltRegulator;

/// Component tag to retrieve the position regulation entity
#[derive(Component)]
pub struct PositionRegulator;

pub struct RegulatorPlugin;

impl Plugin for RegulatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_regulation);
        app.add_systems(FixedUpdate, regulator);
    }
}

fn setup_regulation(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands
) {
    commands.spawn(RegulatorCoefficient{
        proportional: -0.8,
        integral: -0.40,
        derivative: -0.6,
        cumulative_error: 0.0,
        last_error: 0.0,
    })
    .insert(TiltRegulator);

    commands.spawn(RegulatorCoefficient{
        proportional: 0.0,
        integral: 0.0,
        derivative: 0.0,
        cumulative_error: 0.0,
        last_error: 0.0,
    })
    .insert(PositionRegulator);
}

fn regulator(
    mut tilt_regulator_query: Query<&mut RegulatorCoefficient, With<TiltRegulator>>,
    mut position_regulator_query: Query<&mut RegulatorCoefficient, (With<PositionRegulator>, Without<TiltRegulator>)>,
    pendulum_query: Query<&Pendulum>,
    mut cart_query: Query<&mut ExternalForce, With<Cart>>,
) {
    let mut tilt_regulator = tilt_regulator_query.single_mut();
    let mut position_regulator = position_regulator_query.single_mut();
    let pendulum = pendulum_query.single();
    let mut cart_force = cart_query.single_mut();

    let mut tilt_angle_bias: f32 = 0.0;

    if !pendulum.tilt_angle.is_nan() && pendulum.tilt_angle < 50.0 && pendulum.tilt_angle > -50.0 {
        let target_position = 0.0;
        let position_error = target_position - pendulum.position;
        // Cart position regulation
        // Apply P factor
        tilt_angle_bias = position_error * position_regulator.proportional;

        // Apply D factor
        tilt_angle_bias += (position_error - position_regulator.last_error) * position_regulator.derivative;
        position_regulator.last_error = position_error;

        // Apply I factor
        position_regulator.cumulative_error += position_error * 0.003;
        tilt_angle_bias += position_regulator.cumulative_error * position_regulator.integral;
    }

    if !pendulum.tilt_angle.is_nan() && pendulum.above_cart && pendulum.tilt_angle < 50.0 && pendulum.tilt_angle > -50.0{
        // Assign a bias in the target angle to correct the position of the cart
        let target_angle = 0.0 + tilt_angle_bias;
        let tilt_angle_error = target_angle - pendulum.tilt_angle;
        println!("Tilt angle error: {tilt_angle_error}");

        // Tilt angle regulation
        // Apply P factor
        cart_force.force.x = tilt_angle_error * MOVING_FORCE * tilt_regulator.proportional;

        // Apply D factor
        cart_force.force.x += (tilt_angle_error - tilt_regulator.last_error) * MOVING_FORCE * tilt_regulator.derivative;
        tilt_regulator.last_error = tilt_angle_error;

        // Apply I factor
        tilt_regulator.cumulative_error += pendulum.tilt_angle;
        cart_force.force.x += tilt_regulator.cumulative_error * tilt_regulator.integral;
    } else {
        cart_force.force.x = 0.0;
    }
}