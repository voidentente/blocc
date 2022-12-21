#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .add_plugin(bevy::log::LogPlugin::default())
        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::time::TimePlugin)
        .add_plugin(bevy::transform::TransformPlugin)
        .add_plugin(bevy::hierarchy::HierarchyPlugin)
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
        .add_plugin(bevy::input::InputPlugin)
        .add_plugin(bevy::window::WindowPlugin {
            window: bevy::window::WindowDescriptor {
                title: "Blocc Launcher".to_string(),
                width: 512.,
                height: 512.,
                resizable: false,
                position: bevy::window::WindowPosition::Centered,
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugin(bevy::asset::AssetPlugin {
            asset_folder: "".to_string(),
            watch_for_changes: true,
        })
        .add_plugin(bevy::scene::ScenePlugin)
        .add_plugin(bevy::winit::WinitPlugin)
        .add_plugin(bevy::render::RenderPlugin)
        .add_plugin(bevy::render::texture::ImagePlugin::default_nearest())
        .add_plugin(bevy::core_pipeline::CorePipelinePlugin)
        .add_plugin(bevy::sprite::SpritePlugin)
        .add_plugin(bevy::text::TextPlugin)
        .add_plugin(bevy::pbr::PbrPlugin)
        .add_plugin(bevy::gilrs::GilrsPlugin)
        .add_plugin(bevy::animation::AnimationPlugin::default())
        // Third party
        .add_plugin(bevy_rapid_qoi::QOIPlugin)
        .add_plugin(bevy_egui::EguiPlugin)
        .add_plugin(style::StylePlugin)
        .insert_resource(bevy_framepace::FramepaceSettings {
            limiter: bevy_framepace::Limiter::from_framerate(60.0),
        })
        .add_plugin(bevy_framepace::FramepacePlugin)
        // Internal
        .add_plugin(icon::IconPlugin)
        .add_plugin(client_state::GameStatePlugin)
        .add_plugin(player_identity::PlayerIdentityPlugin)
        .add_plugin(profile::ProfilePlugin)
        .add_plugin(launcher::LauncherPlugin)
        //
        .run();
}
