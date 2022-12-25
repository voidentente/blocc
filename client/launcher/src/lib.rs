use bevy::prelude::*;
use client_state::GameState;
use player_identity::{PlayerIdentities, PlayerIdentitiesHandle, PlayerIdentitySelection};
use profile::{ProfileSelection, Profiles};
use rapid_qoi::Qoi;
use text_asset::TextAsset;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Launcher)
                .with_system(setup)
                .with_system(window_settings),
        );

        app.add_system_set(
            SystemSet::on_update(GameState::Launcher)
                .with_system(draw)
                .with_system(on_news_loaded),
        );

        warn!("Unlocalized Strings");
    }
}

fn window_settings(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();

    window.set_title("Blocc Launcher".to_string());
    window.set_resolution(940., 540.);
    window.set_resizable(false);
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
    news: Option<Vec<Vec<String>>>,
}

#[derive(Resource)]
struct LauncherNews(Handle<TextAsset>);

fn setup(
    mut commands: Commands,
    mut ctx: ResMut<bevy_egui::EguiContext>,
    server: Res<AssetServer>,
) {
    // Load some News

    let news = server.load::<TextAsset, _>("http://bloc.cli.rs/news/news.txt");

    commands.insert_resource(LauncherNews(news));

    // Load Launcher State

    let ctx = ctx.ctx_mut();

    let menu = LauncherMenu::MainMenu;

    let background = {
        let (header, rgba) = Qoi::decode_alloc(embedded_assets::LAUNCHER_BACKGROUND).unwrap();

        let image = egui::ColorImage::from_rgba_unmultiplied(
            [header.width as _, header.height as _],
            &rgba,
        );

        let options = egui::TextureOptions::NEAREST;

        ctx.load_texture("launcher_background_image", image, options)
    };

    let identity_textedit = String::new();

    commands.insert_resource(LauncherState {
        menu,
        background,
        identity_textedit,
        news: None,
    });
}

fn on_news_loaded(
    mut ev_asset: EventReader<AssetEvent<TextAsset>>,
    assets: Res<Assets<TextAsset>>,
    news: Res<LauncherNews>,
    mut state: ResMut<LauncherState>,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            if *handle == news.0 {
                let news_text = assets.get(handle).unwrap();

                let mut articles = Vec::new();

                for article in news_text.0.split("\n\n").collect::<Vec<&str>>() {
                    let mut lines = Vec::new();

                    for line in article.split("\n").collect::<Vec<&str>>() {
                        lines.push(line.to_owned());
                    }

                    articles.push(lines);
                }

                state.news = Some(articles);
            }
        }
    }
}

fn draw(
    mut ctx: ResMut<bevy_egui::EguiContext>,
    mut state: ResMut<LauncherState>,
    identities_handle: Res<PlayerIdentitiesHandle>,
    mut identities_assets: ResMut<Assets<PlayerIdentities>>,
    mut identity_selection: ResMut<PlayerIdentitySelection>,
    profiles: Res<Profiles>,
    mut profile_selection: ResMut<ProfileSelection>,
) {
    egui::Area::new("launcher_background_area")
        .order(egui::Order::Background)
        .show(ctx.ctx_mut(), |ui| {
            ui.image(&state.background, egui::vec2(940., 540.));
        });

    egui::Area::new("launcher_titleoutline_area")
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0., 8.))
        .show(ctx.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new("blocc")
                        .heading()
                        .size(96.)
                        .color(egui::Color32::BROWN),
                );
            });
        });

    egui::Area::new("launcher_title_area")
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0., 8.))
        .order(egui::Order::Foreground)
        .show(ctx.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new("blocc")
                        .heading()
                        .size(92.)
                        .color(egui::Color32::GOLD),
                );
            });
        });

    egui::Area::new("launcher_subtitle_area")
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0., 132.))
        .show(ctx.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new("[ l a u n c h e r ]")
                        .font(egui::FontId::new(
                            16.,
                            egui::FontFamily::Name("kongtext".into()),
                        ))
                        .color(egui::Color32::WHITE),
                );
            });
        });

    match state.menu {
        LauncherMenu::MainMenu => {
            egui::Area::new("launcher_news_area")
                .anchor(egui::Align2::CENTER_TOP, egui::vec2(0., 192.))
                .show(ctx.ctx_mut(), |ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(300., 540.),
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            ui.hyperlink_to(
                                egui::RichText::new("News").font(egui::FontId::new(
                                    32.,
                                    egui::FontFamily::Name("november".into()),
                                )),
                                "https://bloc.cli.rs/news/",
                            );

                            ui.separator();

                            if let Some(news) = &state.news {
                                let scroll_area = egui::ScrollArea::vertical()
                                    .auto_shrink([false; 2])
                                    .id_source("launcher_news_scrollarea")
                                    .max_width(300.)
                                    .max_height(200.);

                                scroll_area.show(ui, |ui| {
                                    for (i, article) in news.iter().enumerate() {
                                        for (j, line) in article.iter().enumerate() {
                                            let label = match j {
                                                0 => egui::Label::new(
                                                    egui::RichText::new(line).font(
                                                        egui::FontId::new(
                                                            12.,
                                                            egui::FontFamily::Name(
                                                                "kongtext".into(),
                                                            ),
                                                        ),
                                                    ),
                                                )
                                                .wrap(true),
                                                _ => egui::Label::new(
                                                    egui::RichText::new(line).font(
                                                        egui::FontId::new(
                                                            8.,
                                                            egui::FontFamily::Name(
                                                                "kongtext".into(),
                                                            ),
                                                        ),
                                                    ),
                                                )
                                                .wrap(true),
                                            };

                                            ui.add(label);
                                        }

                                        if i + 1 != news.len() {
                                            ui.separator();
                                        }
                                    }
                                });
                            } else {
                                ui.spinner();
                            }

                            ui.separator();
                        },
                    );
                });

            egui::Area::new("launcher_identities_area")
                .anchor(egui::Align2::LEFT_TOP, egui::vec2(0., 192.))
                .show(ctx.ctx_mut(), |ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(235., 540.),
                        egui::Layout::top_down(egui::Align::RIGHT),
                        |ui| {
                            let button = egui::Button::new("Identities").frame(false);

                            if ui.add(button).clicked() {
                                profile_selection.0 = None;
                                state.menu = LauncherMenu::ManageIdentities;
                            }

                            ui.separator();

                            let scroll_area = egui::ScrollArea::vertical()
                                .auto_shrink([false; 2])
                                .id_source("launcher_identityscrollarea");

                            scroll_area.show(ui, |ui| {
                                if let Some(identities) =
                                    identities_assets.get_mut(&identities_handle.0)
                                {
                                    for i in 0..identities.0.len() {
                                        let (name, _) = &identities.0[i];
                                        let name = name.to_owned();
                                        let selection = identity_selection.0.unwrap_or(usize::MAX);
                                        if ui.selectable_label(selection == i, name).clicked() {
                                            identity_selection.0 = Some(i);
                                        }
                                    }
                                }
                            });
                        },
                    );
                });

            egui::Area::new("launcher_profiles_area")
                .anchor(egui::Align2::RIGHT_TOP, egui::vec2(0., 192.))
                .show(ctx.ctx_mut(), |ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(235., 540.),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            let button = egui::Button::new("Profiles").frame(false);

                            if ui.add(button).clicked() {
                                info!("Unimplemented.");
                            }

                            ui.separator();

                            let scroll_area = egui::ScrollArea::vertical()
                                .auto_shrink([false; 2])
                                .id_source("launcher_profilescrollarea");

                            scroll_area.show(ui, |ui| {
                                for (i, name) in profiles.0.iter().enumerate() {
                                    let selection = profile_selection.0.unwrap_or(usize::MAX);
                                    if ui.selectable_label(selection == i, name).clicked() {
                                        profile_selection.0 = Some(i);
                                    }
                                }
                            });
                        },
                    );
                });

            egui::Area::new("launcher_startbutton_area")
                .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0., -40.))
                .show(ctx.ctx_mut(), |ui| {
                    ui.vertical_centered(|ui| {
                        let button_text = egui::RichText::new("Launch");

                        let button = egui::Button::new(button_text).frame(false);

                        if ui.add(button).clicked() {
                            info!("Unimplemented.");
                        }
                    });
                });
        }

        LauncherMenu::ManageIdentities => {
            egui::Area::new("launcher_manageidentities_area")
                .anchor(egui::Align2::CENTER_TOP, egui::vec2(0., 192.))
                .show(ctx.ctx_mut(), |ui| {
                    if let Some(identities) = identities_assets.get_mut(&identities_handle.0) {
                        ui.horizontal_top(|ui| {
                            ui.allocate_ui_with_layout(
                                egui::vec2(470., 256.),
                                egui::Layout::top_down(egui::Align::RIGHT),
                                |ui| {
                                    let scroll_area = egui::ScrollArea::vertical()
                                        .auto_shrink([false; 2])
                                        .id_source("launcher_manageidentities_scrollarea");

                                    scroll_area.show(ui, |ui| {
                                        let mut to_be_removed = None;

                                        for i in 0..identities.0.len() {
                                            let (name, _) = &identities.0[i];
                                            let name = name.to_owned();

                                            ui.horizontal(|ui| {
                                                if ui.button("-").clicked() {
                                                    to_be_removed = Some(i);
                                                }
                                                ui.label(name);
                                            });

                                            if identities.0.len() != i + 1 {
                                                ui.separator();
                                            }
                                        }

                                        if let Some(i) = to_be_removed {
                                            identities.0.remove(i);
                                        }
                                    });
                                },
                            );

                            ui.allocate_ui_with_layout(
                                egui::vec2(470., 256.),
                                egui::Layout::top_down(egui::Align::LEFT),
                                |ui| {
                                    ui.horizontal(|ui| {
                                        if ui.button("+").clicked() {
                                            identities
                                                .add_identity(state.identity_textedit.clone());
                                            state.identity_textedit.clear();
                                        }

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
                                            state.identity_textedit.clear();
                                        }
                                    });

                                    ui.separator();

                                    ui.horizontal(|ui| {
                                        if ui.button("Save").clicked() {
                                            identities.write();
                                        }
                                        if ui.button("Return").clicked() {
                                            state.identity_textedit.clear();
                                            state.menu = LauncherMenu::MainMenu;
                                        }
                                    });
                                },
                            );
                        });
                    }
                });
        }
    }
}
