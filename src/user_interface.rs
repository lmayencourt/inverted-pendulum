/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 *
 * Provides a graphical interface to the simulation
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui_plot::{Legend, Line, PlotPoints};

use crate::simulator::{Pendulum, COLOR_ORANGE};

pub struct UserInterfacePlugin;

/// Resource to keep track of pendulum positions
#[derive(Resource)]
struct PendulumHistory {
    tilt_angles: Vec<[f64; 2]>,
    position_errors: Vec<[f64; 2]>,
    applied_forces: Vec<[f64; 2]>,
}

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, update_ui);
        app.insert_resource(PendulumHistory{
            tilt_angles: vec![],
            position_errors: vec![],
            applied_forces: vec![],
        });
    }
}

fn update_ui (
    mut time: Res<Time>,
    mut contexts: EguiContexts,
    mut history: ResMut<PendulumHistory>,
    query: Query<&Pendulum>,
) {
    let pendulum = query.single();
    history.tilt_angles.push([time.elapsed_seconds_f64(), pendulum.tilt_angle.into()]);
    history.position_errors.push([time.elapsed_seconds_f64(), pendulum.position_error.into()]);
    history.applied_forces.push([time.elapsed_seconds_f64(), pendulum.applied_force.into()]);

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

            let position_error = PlotPoints::from(history.position_errors.clone());
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