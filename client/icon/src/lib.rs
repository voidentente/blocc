use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;

use rapid_qoi::Qoi;
use winit::window::Icon;

pub struct IconPlugin;

impl Plugin for IconPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_window_icon);
    }
}

fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    let icon = {
        let (header, rgba) = Qoi::decode_alloc(assets::ICON).unwrap();

        Icon::from_rgba(rgba, header.width, header.height).unwrap()
    };

    primary.set_window_icon(Some(icon));
}
