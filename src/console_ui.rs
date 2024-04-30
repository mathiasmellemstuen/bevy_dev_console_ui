use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_egui::egui::Pos2;
use crate::console_command_registry::{CommandInput, NextCommandInput};
use crate::console_command_registry::CommandRegistry;
use crate::register_command_system;
use crate::console_command_registry::purge_command;

pub struct ConsoleUIPlugin;

impl Plugin for ConsoleUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.init_resource::<ConsoleUIRes>();
        app.init_resource::<NextCommandInput>();
        app.init_resource::<CommandRegistry>();

        app.add_systems(Update, ui_system);

        // Registering commands to control the ui with
        register_command_system!("clear", app, Update, clear_command);
        register_command_system!("help", app, Update, help_command);
    }
}

fn help_command(command_registry: Res<CommandRegistry>) {
    println!("--- ALL COMMANDS ---");
    for s in &command_registry.0 {
        println!("{}", s);
    }
}
fn clear_command(mut console_ui_r : ResMut<ConsoleUIRes>) {

    for i in 0..console_ui_r.previous_outputs.len() {
        console_ui_r.previous_outputs[i] = String::from("");
    }
}

#[derive(Default, Resource)]
struct ConsoleUIRes {
    previous_outputs : [String; 5],
    text_input : String,
    toggled : bool
}

macro_rules! shift_vec_forward {
    ($new:tt, $x:expr) => {
        {
            let mut n_x : Vec<String> = vec![];

            for i in 0..$x.len() {
                n_x.push($x[i].clone());
            }
            for i in (0..$x.len() - 1).rev() {
                $x[i] = n_x[i + 1].clone();
            }

            let l = $x.len() - 1;

            $x[l] = String::from($new);
        }
    };
}
fn ui_system(
    mut contexts : EguiContexts,
    mut console_ui_r : ResMut<ConsoleUIRes>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    command_registry : Res<CommandRegistry>,
    mut next_command_input: ResMut<NextCommandInput>,
    window_q : Query<&Window>) {

    let window = window_q.get_single().unwrap();

    let size = bevy_egui::egui::Vec2::new(300.0, 100.0);
    let pos = Pos2{x: (window.resolution.width() as f32 / 2.0) - size.x / 2.0, y: 0.1 * ((window.resolution.height() as f32 / 2.0) - size.y / 2.0)};


    if keyboard_input.just_pressed(KeyCode::Escape) {
        console_ui_r.toggled = false;
    }
    if keyboard_input.just_pressed(KeyCode::Tab) {
        console_ui_r.toggled = !console_ui_r.toggled;
    }

    if console_ui_r.toggled {
        egui::Window::new("Dev Console").title_bar(false).current_pos(pos).min_size(size).max_size(size).show(contexts.ctx_mut(), |ui| {
            for i in 0..console_ui_r.previous_outputs.len() {
                ui.label(&console_ui_r.previous_outputs[i]);
            }
            let text_input_ui = ui.text_edit_singleline(&mut console_ui_r.text_input);

            // Requesting focus so we can press enter without unfocusing
            text_input_ui.request_focus();

            if keyboard_input.just_pressed(KeyCode::Enter) {
                match &console_ui_r.text_input.parse::<CommandInput>() {
                    Ok(ci) => {
                        if command_registry.0.contains(&ci.command) {
                            shift_vec_forward!("Command executed!", console_ui_r.previous_outputs);
                            next_command_input.0 = Some(ci.clone());
                        } else {
                            error!("The command '{}' does not exist in the command registry!", &console_ui_r.text_input);
                            shift_vec_forward!("Command does not exist in the command registry!", console_ui_r.previous_outputs);
                        }
                    }
                    Err(_e) => {
                        shift_vec_forward!("Failed to parse command!", console_ui_r.previous_outputs);
                    }
                };

                console_ui_r.text_input = String::from("");
            }
        });
    }
}