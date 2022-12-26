#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use iyes_loopless::prelude::*;

fn main() {
    App::new()
        .add_state("launcher")
        .insert_resource(Msaa { samples: 1 })
        .add_fixed_timestep(std::time::Duration::from_millis(1000), "tick")
        .add_plugin(bevy::log::LogPlugin::default())
        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::time::TimePlugin)
        .add_plugin(bevy::transform::TransformPlugin)
        .add_plugin(bevy::hierarchy::HierarchyPlugin)
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
        .add_plugin(bevy::input::InputPlugin)
        .add_plugin(bevy::window::WindowPlugin {
            window: bevy::window::WindowDescriptor {
                title: "Blocc".to_string(),
                width: 940.,
                height: 540.,
                resizable: false,
                position: bevy::window::WindowPosition::Centered,
                present_mode: bevy::window::PresentMode::Mailbox,
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugin(bevy_web_asset::WebAssetPlugin {
            asset_plugin: bevy::asset::AssetPlugin {
                asset_folder: "profiles".to_string(),
                watch_for_changes: true,
            },
        })
        .add_plugin(bevy::scene::ScenePlugin)
        .add_plugin(bevy::winit::WinitPlugin)
        .add_plugin(bevy::render::RenderPlugin)
        .add_plugin(bevy::render::texture::ImagePlugin::default_nearest())
        .add_plugin(bevy::core_pipeline::CorePipelinePlugin)
        .add_plugin(bevy::sprite::SpritePlugin)
        .add_plugin(bevy::text::TextPlugin)
        .add_plugin(bevy::pbr::PbrPlugin)
        .add_plugin(bevy::animation::AnimationPlugin::default())
        .add_plugin(bevy_egui::EguiPlugin)
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(egui_style::StylePlugin)
        .add_plugin(text_asset::TextAssetPlugin)
        .add_plugin(app_icon::MainWindowIconPlugin)
        .add_plugin(player_identity::PlayerIdentityPlugin)
        .add_plugin(profile_selection::ProfileSelectionPlugin)
        .add_plugin(launcher::LauncherPlugin)
        .run();
}
