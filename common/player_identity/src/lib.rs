//! An identity is required to coordinate players on servers, like inventories.
//! A player can have multiple identities; names are not unique, but the UUID is.
//! Note that the UUID is considered to be a soft secret and should not be leaked.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;
use toml::{from_str, to_string_pretty, Value};
pub struct PlayerIdentityPlugin;

impl Plugin for PlayerIdentityPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerIdentities::read());
    }
}

pub const PATH: &str = "./identities.toml";

#[derive(Debug, Resource)]
pub struct PlayerIdentities(Value);

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct PlayerIdentity {
    uuid: String,
    name: String,
}

impl PlayerIdentity {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            uuid: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
        }
    }
}

impl PlayerIdentities {
    pub fn read() -> Self {
        let path = Path::new(PATH);

        if path.exists() {
            Self(from_str(&std::fs::read_to_string(path).unwrap()).unwrap())
        } else {
            std::fs::File::create(path).unwrap();
            Self(Value::Table(toml::map::Map::new()))
        }
    }

    pub fn write(&self) {
        std::fs::write(PATH, to_string_pretty(&self.0).unwrap()).unwrap();
    }
}
