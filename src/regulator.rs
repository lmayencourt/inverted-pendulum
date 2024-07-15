/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 *
 * Provides a PID regulator for the pendulum
 */

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::simulator::{Pendulum, Cart, MOVING_FORCE};
// use crate::user_interface::PendulumHistory;

/// Resource to keep track of pendulum erros
#[derive(Resource)]
pub struct RegulatorCoefficient {
    proportional: f32,
    integral: f32,
    derivative: f32,
    cumulative_error: f32,
    last_error: f32,
}

pub struct RegulatorPlugin;

impl Plugin for RegulatorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RegulatorCoefficient{
            proportional: 0.100,
            integral: 0.4,
            derivative: 0.3,
            cumulative_error: 0.0,
            last_error: 0.0,
        });
        app.add_systems(FixedUpdate, regulator);
    }
}

fn regulator(
    mut regulator: ResMut<RegulatorCoefficient>,
    pendulum_query: Query<&Pendulum>,
    mut cart_query: Query<&mut ExternalForce, With<Cart>>,
) {
    let pendulum = pendulum_query.single();
    let mut cart_force = cart_query.single_mut();

    if !pendulum.tilt_angle.is_nan() && pendulum.above_cart && pendulum.tilt_angle < 60.0 && pendulum.tilt_angle > -60.0{
        // Apply P factor
        cart_force.force.x = pendulum.tilt_angle * MOVING_FORCE * regulator.proportional;

        // Apply D factor
        cart_force.force.x += (pendulum.tilt_angle - regulator.last_error) * MOVING_FORCE * regulator.derivative;
        regulator.last_error = pendulum.tilt_angle;

        // Apply I factor
        regulator.cumulative_error += pendulum.position_error;
        cart_force.force.x += regulator.cumulative_error * regulator.integral;
    } else {
        cart_force.force.x = 0.0;
    }
}