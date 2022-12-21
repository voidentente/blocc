use bevy::prelude::*;
use client_state::GameState;

pub struct ProfilePlugin;

impl Plugin for ProfilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Launcher).with_system(Profiles::update));
        app.init_resource::<Profiles>();
        app.init_resource::<ProfileSelection>();
    }
}

#[derive(Resource)]
pub struct Profiles(pub Vec<String>);

impl Default for Profiles {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Profiles {
    fn update(mut commands: Commands) {
        match std::fs::read_dir("profiles") {
            Ok(dir) => {
                let mut vec = Vec::new();

                for entry in dir.flatten() {
                    if entry.metadata().unwrap().is_dir() {
                        vec.push(entry.file_name().into_string().unwrap());
                    }
                }

                commands.insert_resource(Self(vec));
            }

            Err(e) => {
                warn!("{}", e);
            }
        }
    }
}

#[derive(Debug, Resource)]
pub struct ProfileSelection(pub Option<usize>);

impl Default for ProfileSelection {
    fn default() -> Self {
        Self(None)
    }
}
