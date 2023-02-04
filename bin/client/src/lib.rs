use bevy::prelude::*;

pub fn new() -> App {
    let mut app = App::new();

    app.add_plugin(logging::BloccLoggPlugin::default());
    app.add_plugin(CorePlugin::default());
    app.add_plugin(bevy::diagnostic::DiagnosticsPlugin);
    app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default());
    app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
    app.add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin);

    app.add_plugin(bevy::time::TimePlugin);
    app.add_plugin(bevy::input::InputPlugin);
    app.add_plugin(TransformPlugin);
    app.add_plugin(AssetPlugin {
        watch_for_changes: true,
        ..Default::default()
    });

    app.add_plugin(WindowPlugin {
        window: WindowDescriptor {
            title: "Blocc".to_string(),
            width: 1024.,
            height: 512.,
            resize_constraints: bevy::window::WindowResizeConstraints {
                min_width: 1024.,
                min_height: 512.,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    app.add_plugin(bevy::winit::WinitPlugin);
    app.add_plugin(bevy::render::RenderPlugin);
    app.add_plugin(ImagePlugin::default_nearest());
    app.insert_resource(bevy_framepace::FramepaceSettings {
        limiter: bevy_framepace::Limiter::from_framerate(60.),
    });
    app.add_plugin(bevy_framepace::FramepacePlugin);

    app
}