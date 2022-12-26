use std::path::Path;

use bevy::prelude::*;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

pub const PATH: &str = "identities.ron";

/// This plugin can be added to read and store a `profiles.ron` file that stores player identities.
pub struct PlayerIdentityPlugin;

impl Plugin for PlayerIdentityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter("launcher").with_system(
            |mut commands: Commands| commands.insert_resource(PlayerIdentities::read()),
        ));
    }
}

#[derive(Serialize, Deserialize, Resource)]
pub struct PlayerIdentities {
    pub idents: Vec<(String, Keypair)>,
    pub selection: Option<usize>,
}

impl PlayerIdentities {
    pub fn write(&self) {
        let contents = ron::ser::to_string_pretty(&self.idents, Default::default())
            .expect("Failed to write PlayerIdenties to disk");

        std::fs::write(PATH, contents).unwrap();
    }

    pub fn read() -> Self {
        let path = Path::new(PATH);

        if !path.exists() {
            PlayerIdentities {
                idents: Vec::new(),
                selection: None,
            }
            .write();
        }

        Self {
            idents: ron::de::from_bytes(&std::fs::read(path).unwrap())
                .expect("Failed to read PlayerIdentities from disk"),
            selection: None,
        }
    }

    pub fn add_identity<T: Into<String>>(&mut self, name: T) {
        self.idents
            .push((name.into(), Keypair::generate(&mut OsRng {})));
    }
}
