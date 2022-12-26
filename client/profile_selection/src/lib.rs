use bevy::prelude::*;

/// This plugin can be added to discover and select between multiple profiles in a profile folder.
pub struct ProfileSelectionPlugin;

impl Plugin for ProfileSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter("launcher").with_system(Profiles::update));
        app.init_resource::<Profiles>();
    }
}

/// This resource contains all detected profile directories, and the currently selected profile as index.
#[derive(Resource, Default)]
pub struct Profiles {
    pub dirs: Vec<std::ffi::OsString>,
    pub selection: Option<usize>,
}

impl Profiles {
    fn update(mut commands: Commands) {
        if let Ok(dir) = std::fs::read_dir("profiles") {
            let mut dirs = Vec::new();

            for entry in dir.flatten() {
                if entry.metadata().unwrap().is_dir() {
                    dirs.push(entry.file_name());
                }
            }

            commands.insert_resource(Self {
                dirs,
                selection: None,
            });
        }
    }
}
