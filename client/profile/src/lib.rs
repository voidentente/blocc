use bevy::prelude::*;
use client_state::GameState;

pub struct ProfilePlugin;

impl Plugin for ProfilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Launcher).with_system(Profiles::update));
        app.init_resource::<Profiles>();
        app.init_resource::<ProfileSelection>();
    }
}

#[derive(Resource, Default)]
pub struct Profiles(pub Vec<String>);

impl Profiles {
    fn update(mut commands: Commands) {
        if let Ok(dir) = std::fs::read_dir("profiles") {
            let mut vec = Vec::new();

            for entry in dir.flatten() {
                if entry.metadata().unwrap().is_dir() {
                    vec.push(entry.file_name().into_string().unwrap());
                }
            }

            commands.insert_resource(Self(vec));
        }
    }
}

#[derive(Resource, Default)]
pub struct ProfileSelection(pub Option<usize>);
