use bevy::prelude::*;
use image::GenericImageView;

pub fn setup_icon(windows: NonSend<bevy::winit::WinitWindows>) {
    let primary = windows
        .get_window(bevy::window::WindowId::primary())
        .unwrap();

    let image = image::load_from_memory_with_format(
        include_bytes!("../assets/icon.png"),
        image::ImageFormat::Png,
    )
        .unwrap();

    let (width, height) = image.dimensions();

    let icon =
        winit::window::Icon::from_rgba(image.into_rgba8().into_raw(), width, height).unwrap();

    primary.set_window_icon(Some(icon));
}