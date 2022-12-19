use bevy::prelude::*;

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(client_state::GameState::Launcher).with_system(draw),
        );
    }
}

fn draw(mut ctx: ResMut<bevy_egui::EguiContext>) {
    egui::Area::new("launcher_title_area").show(ctx.ctx_mut(), |ui| {
        ui.label("Blocc");
    });
}
