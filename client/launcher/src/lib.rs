use bevy::prelude::*;
use client_state::GameState;
use player_identity::{PlayerIdentities, PlayerIdentitiesHandle, PlayerIdentitySelection};
use rapid_qoi::Qoi;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Launcher)
                .with_system(window)
                .with_system(setup),
        );

        app.add_system_set(SystemSet::on_update(GameState::Launcher).with_system(draw));

        warn!("Unlocalized Strings");
    }
}

fn window(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();

    window.set_title("launcher".to_string());
    window.set_resizable(false);
    window.set_resolution(512., 512.);
    window.center_window(bevy::window::MonitorSelection::Current);
    window.set_present_mode(bevy::window::PresentMode::AutoNoVsync);

    commands.insert_resource(bevy_framepace::FramepaceSettings {
        limiter: bevy_framepace::Limiter::from_framerate(60.0),
    });
}

enum LauncherMenu {
    MainMenu,
    ManageIdentities,
}

#[derive(Resource)]
struct LauncherState {
    menu: LauncherMenu,
    background: egui::TextureHandle,
    identity_textedit: String,
    identity_status_message: egui::RichText,
}

fn setup(mut commands: Commands, mut ctx: ResMut<bevy_egui::EguiContext>) {
    let ctx = ctx.ctx_mut();

    let menu = LauncherMenu::MainMenu;

    let background = {
        let (header, rgba) = Qoi::decode_alloc(assets::LAUNCHER_BACKGROUND).unwrap();

        let image = egui::ColorImage::from_rgba_unmultiplied(
            [header.width as _, header.height as _],
            &rgba,
        );

        let options = egui::TextureOptions::NEAREST;

        ctx.load_texture("launcher_background_image", image, options)
    };

    let identity_textedit = String::new();
    let identity_status_message = egui::RichText::new("");

    commands.insert_resource(LauncherState {
        menu,
        background,
        identity_textedit,
        identity_status_message,
    });
}

fn draw(
    mut ctx: ResMut<bevy_egui::EguiContext>,
    mut state: ResMut<LauncherState>,
    identities_handle: Res<PlayerIdentitiesHandle>,
    mut identities_assets: ResMut<Assets<PlayerIdentities>>,
    mut identity_selection: ResMut<PlayerIdentitySelection>,
) {
    egui::Area::new("launcher_background_area")
        .order(egui::Order::Background)
        .show(ctx.ctx_mut(), |ui| {
            ui.image(&state.background, egui::vec2(512., 512.));
        });

    egui::Area::new("launcher_title_area")
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0., 16.))
        .show(ctx.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new("blocc")
                        .heading()
                        .size(72.)
                        .color(egui::Color32::GOLD),
                );
            });
        });

    match state.menu {
        LauncherMenu::MainMenu => {
            egui::Area::new("launcher_mainmenu_area")
                .anchor(egui::Align2::LEFT_TOP, egui::vec2(0., 192.))
                .show(ctx.ctx_mut(), |ui| {
                    egui::Grid::new("launcher_mainmenu_grid")
                        .num_columns(2)
                        .max_col_width(256.)
                        .min_col_width(256.)
                        .min_row_height(256.)
                        .show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                                let scroll_area =
                                    egui::ScrollArea::vertical().auto_shrink([false; 2]);

                                let button = egui::Button::new("Identities").frame(false);

                                if ui.add(button).clicked() {
                                    state.menu = LauncherMenu::ManageIdentities;
                                }

                                ui.separator();

                                scroll_area.show(ui, |ui| {
                                    if let Some(identities) =
                                        identities_assets.get_mut(&identities_handle.0)
                                    {
                                        for i in 0..identities.0.len() {
                                            let (name, _) = &identities.0[i];
                                            let name = name.to_owned();
                                            let selection =
                                                identity_selection.0.unwrap_or(usize::MAX);
                                            if ui.selectable_label(selection == i, name).clicked() {
                                                identity_selection.0 = Some(i);
                                            }
                                        }
                                    }
                                });
                            });

                            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                let button = egui::Button::new("Profiles").frame(false);

                                if ui.add(button).clicked() {
                                    info!("Unimplemented");
                                }

                                ui.separator();
                            });
                        });
                });

            egui::Area::new("launcher_startbutton_area")
                .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0., -32.))
                .show(ctx.ctx_mut(), |ui| {
                    ui.vertical_centered_justified(|ui| {
                        let button_text = egui::RichText::new("Launch");

                        let button = egui::Button::new(button_text).frame(false);

                        if ui.add(button).clicked() {
                            info!("Starting...");
                        }
                    });
                });
        }

        LauncherMenu::ManageIdentities => {
            egui::Area::new("launcher_manageidentities_area")
                .anchor(egui::Align2::LEFT_TOP, egui::vec2(0., 192.))
                .show(ctx.ctx_mut(), |ui| {
                    if let Some(identities) = identities_assets.get_mut(&identities_handle.0) {
                        egui::Grid::new("launcher_manageidentities_grid")
                            .num_columns(2)
                            .max_col_width(256.)
                            .min_col_width(256.)
                            .min_row_height(256.)
                            .show(ui, |ui| {
                                ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                                    let scroll_area =
                                        egui::ScrollArea::vertical().auto_shrink([false; 2]);

                                    scroll_area.show(ui, |ui| {
                                        let mut to_be_removed = None;

                                        for i in 0..identities.0.len() {
                                            let (name, _) = &identities.0[i];
                                            let name = name.to_owned();
                                            ui.horizontal(|ui| {
                                                if ui.button("-").clicked() {
                                                    state.identity_status_message =
                                                        egui::RichText::new(format!(
                                                            "Removed '{}'",
                                                            name
                                                        ));
                                                    to_be_removed = Some(i);
                                                }
                                                ui.label(name);
                                            });
                                        }

                                        if let Some(i) = to_be_removed {
                                            identities.0.remove(i);
                                        }
                                    });
                                });

                                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                    ui.horizontal(|ui| {
                                        let response = ui.add(
                                            egui::TextEdit::singleline(
                                                &mut state.identity_textedit,
                                            )
                                            .hint_text("Profile Name"),
                                        );

                                        if response.lost_focus()
                                            && ui.input().key_pressed(egui::Key::Enter)
                                        {
                                            identities
                                                .add_identity(state.identity_textedit.clone());
                                            state.identity_status_message = egui::RichText::new(
                                                format!("Added '{}'", state.identity_textedit),
                                            );
                                            state.identity_textedit.clear();
                                        }
                                    });

                                    ui.separator();

                                    ui.horizontal(|ui| {
                                        if ui.button("+").clicked() {
                                            identities
                                                .add_identity(state.identity_textedit.clone());
                                            state.identity_status_message = egui::RichText::new(
                                                format!("Added '{}'", state.identity_textedit),
                                            );
                                            state.identity_textedit.clear();
                                        }
                                        if ui.button("Save").clicked() {
                                            identities.write();
                                            state.identity_status_message =
                                                egui::RichText::new("Saved");
                                        }
                                        if ui.button("Return").clicked() {
                                            state.identity_textedit.clear();
                                            state.identity_status_message = egui::RichText::new("");
                                            state.menu = LauncherMenu::MainMenu;
                                        }
                                    });

                                    ui.add_space(32.);

                                    ui.label(state.identity_status_message.to_owned());
                                });
                            });
                    }
                });
        }
    }
}
