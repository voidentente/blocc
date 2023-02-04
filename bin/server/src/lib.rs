use bevy::prelude::*;

pub fn new() -> App {
    unimplemented!()
}

pub fn new_sub_app() -> App {
    App::new()
}

pub fn runner(_world: &mut World, app: &mut App) {
    app.update();
}