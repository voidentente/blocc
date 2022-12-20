use bevy::prelude::*;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(client_state::GameState::Launcher).with_system(draw),
        );
    }
}

fn draw(windows: Res<Windows>, mut ctx: ResMut<bevy_egui::EguiContext>) {
    let win = windows.primary();
    let height = win.height();
    let width = win.width();

    egui::Area::new("launcher_startbutton_area")
        .fixed_pos(egui::pos2(width * 0.5 - 64., height - 128.))
        .show(ctx.ctx_mut(), |ui| {
            let button_text = egui::RichText::new("Start");

            let button = egui::Button::new(button_text).min_size(egui::vec2(128., 64.));

            if ui.add(button).clicked() {
                info!("Starting...");
            }
        });
}
