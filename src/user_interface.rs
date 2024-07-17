/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 *
 * Provides a graphical interface to the simulation
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui_plot::{Legend, Line, PlotPoints};

use crate::simulator::{Pendulum, COLOR_ORANGE};
use crate::regulator::{RegulatorCoefficient, TiltRegulator, PositionRegulator};

pub struct UserInterfacePlugin;

/// Resource to keep track of pendulum positions
#[derive(Resource)]
struct PendulumHistory {
    tilt_angles: Vec<[f64; 2]>,
    positions: Vec<[f64; 2]>,
    applied_forces: Vec<[f64; 2]>,
}

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, draw_bottom_plots);
        app.add_systems(Update, draw_control_panel);
        app.insert_resource(PendulumHistory{
            tilt_angles: vec![],
            positions: vec![],
            applied_forces: vec![],
        });
    }
}

fn draw_bottom_plots (
    mut time: Res<Time>,
    mut contexts: EguiContexts,
    mut history: ResMut<PendulumHistory>,
    query: Query<&Pendulum>,
) {
    let pendulum = query.single();
    history.tilt_angles.push([time.elapsed_seconds_f64(), pendulum.tilt_angle.into()]);
    history.positions.push([time.elapsed_seconds_f64(), pendulum.position.into()]);
    history.applied_forces.push([time.elapsed_seconds_f64(), pendulum.applied_force.into()]);

    limit_vector_size(&mut history.tilt_angles, 800);
    limit_vector_size(&mut history.positions, 800);
    limit_vector_size(&mut history.applied_forces, 800);

    egui::TopBottomPanel::bottom("Cart speed").show(contexts.ctx_mut(), |ui| {
        egui_plot::Plot::new("State")
        .height(200.0)
        .show_x(true)
        .allow_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .legend(Legend::default())
        .height(100.0)
        .show(ui, |plot_ui| {
            let tilt_angle = PlotPoints::from(history.tilt_angles.clone());
            plot_ui.line(Line::new(tilt_angle).name("tilt angle"));

            let position_error = PlotPoints::from(history.positions.clone());
            plot_ui.line(Line::new(position_error).name("position error"));
        });

        egui_plot::Plot::new("Control")
        .height(200.0)
        .show_x(true)
        .allow_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .legend(Legend::default())
        .height(100.0)
        .show(ui, |plot_ui| {
            let applied_force = PlotPoints::from(history.applied_forces.clone());
            plot_ui.line(Line::new(applied_force).name("force applied"));
        });
    });
}

fn limit_vector_size(vector:&mut Vec<[f64; 2]>, size: usize) {
    if vector.len() > size {
        vector.remove(0);
    }
}

fn draw_control_panel(
    mut contexts: EguiContexts,
    mut tilt_regulator_query: Query<&mut RegulatorCoefficient, With<TiltRegulator>>,
    mut position_regulator_query: Query<&mut RegulatorCoefficient, (Without<TiltRegulator>, With<PositionRegulator>)>,
) {
    let mut tilt_regulator = tilt_regulator_query.single_mut();
    let mut position_regulator = position_regulator_query.single_mut();

    egui::Window::new("PID Controller").show(contexts.ctx_mut(), |ui| {
        ui.label("Tilt angle regulation");
        ui.add(egui::Slider::new(&mut tilt_regulator.proportional, -2.0..=0.0));
        ui.add(egui::Slider::new(&mut tilt_regulator.integral, -2.0..=0.0));
        ui.add(egui::Slider::new(&mut tilt_regulator.derivative, -2.0..=0.0));

        ui.label("Position regulation");
        ui.add(egui::Slider::new(&mut position_regulator.proportional, 0.2..=0.0));
        ui.add(egui::Slider::new(&mut position_regulator.integral, 0.2..=0.0));
        ui.add(egui::Slider::new(&mut position_regulator.derivative, 0.2..=0.0));
    });
}