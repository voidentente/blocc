use bevy::prelude::*;
use player_identity::PlayerIdentities;
use profile_selection::Profiles;
use text_asset::TextAsset;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(LauncherState::MainMenu);

        app.add_system_set(
            SystemSet::on_enter("launcher")
                .with_system(setup_window)
                .with_system(setup_resources),
        );

        app.add_system_set(
            SystemSet::on_update("launcher")
                .with_system(on_news_loaded)
                .with_system(draw),
        );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum LauncherState {
    MainMenu,
    ManageIdentities,
}

#[derive(Resource)]
struct LauncherResources {
    background: egui::TextureHandle,
    identity_textedit: String,
    news: Option<Vec<Vec<String>>>,
}

#[derive(Resource)]
struct LauncherNews(Handle<TextAsset>);

fn setup_window(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();

    window.set_title("Blocc Launcher".to_string());
    window.set_resolution(940., 540.);
    window.set_resizable(false);
    window.center_window(bevy::window::MonitorSelection::Current);
    window.set_present_mode(bevy::window::PresentMode::AutoNoVsync);
}

fn setup_resources(
    mut commands: Commands,
    mut ctx: ResMut<bevy_egui::EguiContext>,
    server: Res<AssetServer>,
) {
    // Load some News

    commands.insert_resource(LauncherNews(
        server.load::<TextAsset, _>("http://bloc.cli.rs/news/news.txt"),
    ));

    // Load Launcher State

    let background = {
        let image = image::load_from_memory_with_format(
            embedded_assets::LAUNCHER_BACKGROUND,
            image::ImageFormat::Png,
        )
        .unwrap()
        .into_rgba8();

        let (width, height) = image.dimensions();

        let image =
            egui::ColorImage::from_rgba_unmultiplied([width as _, height as _], &image.into_raw());

        ctx.ctx_mut().load_texture(
            "launcher_background_image",
            image,
            egui::TextureOptions::NEAREST,
        )
    };

    let identity_textedit = String::new();

    commands.insert_resource(LauncherResources {
        background,
        identity_textedit,
        news: None,
    });
}

fn on_news_loaded(
    mut launcher_resources: ResMut<LauncherResources>,
    mut ev_asset: EventReader<AssetEvent<TextAsset>>,
    assets: Res<Assets<TextAsset>>,
    news: Res<LauncherNews>,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            if *handle == news.0 {
                let news_text = assets.get(handle).unwrap();

                let mut articles = Vec::new();

                for article in news_text.0.split("\n\n").collect::<Vec<&str>>() {
                    let mut lines = Vec::new();

                    for line in article.split('\n').collect::<Vec<&str>>() {
                        lines.push(line.to_owned());
                    }

                    articles.push(lines);
                }

                launcher_resources.news = Some(articles);
            }
        }
    }
}

fn draw(
    mut ctx: ResMut<bevy_egui::EguiContext>,
    mut global_state: ResMut<State<&'static str>>,
    mut launcher_resources: ResMut<LauncherResources>,
    mut launcher_state: ResMut<State<LauncherState>>,
    mut profiles: ResMut<Profiles>,
    mut identities: ResMut<PlayerIdentities>,
) {
    egui::Area::new("launcher_background_area")
        .order(egui::Order::Background)
        .show(ctx.ctx_mut(), |ui| {
            ui.image(&launcher_resources.background, egui::vec2(940., 540.));
        });

    egui::Area::new("launcher_titleshadow_area")
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0., 8.))
        .show(ctx.ctx_mut(), |ui| {
            ui.add_space(4.);

            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new("blocc")
                        .heading()
                        .size(92.)
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

    match launcher_state.current() {
        LauncherState::MainMenu => {
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

                            if let Some(news) = &launcher_resources.news {
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
                                let _ = launcher_state.set(LauncherState::ManageIdentities);
                            }

                            ui.separator();

                            let scroll_area = egui::ScrollArea::vertical()
                                .auto_shrink([false; 2])
                                .id_source("launcher_identityscrollarea");

                            scroll_area.show(ui, |ui| {
                                let mut to_be_selected = None;

                                for i in 0..identities.idents.len() {
                                    let (name, _) = &identities.idents[i];
                                    let name = name.to_owned();

                                    let selection = identities.selection.unwrap_or(usize::MAX);
                                    if ui.selectable_label(selection == i, name).clicked() {
                                        to_be_selected = Some(i);
                                    }
                                }

                                if to_be_selected.is_some() {
                                    identities.selection = to_be_selected;
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
                                let mut to_be_selected = None;

                                for (i, name) in profiles.dirs.iter().enumerate() {
                                    let selection = profiles.selection.unwrap_or(usize::MAX);
                                    if ui
                                        .selectable_label(selection == i, name.to_string_lossy())
                                        .clicked()
                                    {
                                        to_be_selected = Some(i);
                                    }
                                }

                                if to_be_selected.is_some() {
                                    profiles.selection = to_be_selected;
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
                            let _ = global_state.set("some");
                        }
                    });
                });
        }

        LauncherState::ManageIdentities => {
            egui::Area::new("launcher_manageidentities_area")
                .anchor(egui::Align2::CENTER_TOP, egui::vec2(0., 192.))
                .show(ctx.ctx_mut(), |ui| {
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

                                    for i in 0..identities.idents.len() {
                                        let (name, _) = &identities.idents[i];
                                        let name = name.to_owned();

                                        ui.horizontal(|ui| {
                                            if ui.button("-").clicked() {
                                                to_be_removed = Some(i);
                                            }
                                            ui.label(name);
                                        });

                                        if identities.idents.len() != i + 1 {
                                            ui.separator();
                                        }
                                    }

                                    if let Some(i) = to_be_removed {
                                        identities.idents.remove(i);
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
                                        identities.add_identity(
                                            launcher_resources.identity_textedit.clone(),
                                        );
                                        launcher_resources.identity_textedit.clear();
                                    }

                                    let response = ui.add(
                                        egui::TextEdit::singleline(
                                            &mut launcher_resources.identity_textedit,
                                        )
                                        .hint_text("Profile Name"),
                                    );

                                    if response.lost_focus()
                                        && ui.input().key_pressed(egui::Key::Enter)
                                    {
                                        identities.add_identity(
                                            launcher_resources.identity_textedit.clone(),
                                        );
                                        launcher_resources.identity_textedit.clear();
                                    }
                                });

                                ui.separator();

                                ui.horizontal(|ui| {
                                    if ui.button("Save").clicked() {
                                        identities.write();
                                    }
                                    if ui.button("Return").clicked() {
                                        launcher_resources.identity_textedit.clear();
                                        let _ = launcher_state.set(LauncherState::MainMenu);
                                    }
                                });
                            },
                        );
                    });
                });
        }
    }
}
