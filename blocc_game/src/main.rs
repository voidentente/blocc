#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(bevy::log::LogPlugin::default())
        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::time::TimePlugin)
        .add_plugin(bevy::transform::TransformPlugin)
        .add_plugin(bevy::hierarchy::HierarchyPlugin)
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
        .add_plugin(bevy::input::InputPlugin)
        .add_plugin(bevy::window::WindowPlugin::default())
        .add_plugin(bevy::asset::AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
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
        .add_plugin(bevy_framepace::FramepacePlugin)
        // Internal
        .add_plugin(client_state::GameStatePlugin)
        .add_plugin(player_identity::PlayerIdentityPlugin)
        .add_plugin(launcher::LauncherPlugin)
        //
        .run();
}
