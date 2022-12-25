use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use winit::window::Icon;

pub struct MainWindowIconPlugin;

impl Plugin for MainWindowIconPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_main_window_icon);
    }
}

fn set_main_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    let icon = {
        let image =
            image::load_from_memory_with_format(embedded_assets::ICON, image::ImageFormat::Png)
                .unwrap()
                .into_rgba8();

        let (width, height) = image.dimensions();

        Icon::from_rgba(image.into_raw(), width, height).unwrap()
    };

    primary.set_window_icon(Some(icon));
}
