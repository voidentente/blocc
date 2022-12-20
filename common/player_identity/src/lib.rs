use bevy::prelude::*;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use std::path::Path;
use toml::map::Map;
use toml::Value;

pub const PATH: &str = "./identity.toml";

pub struct PlayerIdentityPlugin;

impl Plugin for PlayerIdentityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerIdentities>();
    }
}

#[derive(Debug, Resource)]
pub struct PlayerIdentities(Vec<(String, Keypair)>);

impl Default for PlayerIdentities {
    fn default() -> Self {
        Self::read()
    }
}

impl PlayerIdentities {
    pub fn new<T: Into<String>>(&mut self, name: T) {
        self.0.push((name.into(), Keypair::generate(&mut OsRng {})));
    }

    pub fn read() -> Self {
        let path = Path::new(PATH);

        if path.exists() {
            let read = std::fs::read_to_string(path).unwrap();

            let mut vec = Vec::new();

            if let Value::Table(table) = toml::from_str(&read).unwrap() {
                for (name, key) in table {
                    if let Value::String(key) = key {
                        vec.push((
                            name,
                            Keypair::from_bytes(&base64::decode(key).unwrap()).unwrap(),
                        ));
                    } else {
                        panic!("PlayerIdentities: Invalid Format - Not String")
                    }
                }

                Self(vec)
            } else {
                panic!("PlayerIdentities: Invalid Format - Not Map")
            }
        } else {
            std::fs::File::create(path).unwrap();

            Self(Vec::new())
        }
    }

    pub fn write(&self) {
        let mut map = Map::new();

        for (name, key) in &self.0 {
            map.insert(
                name.to_owned(),
                Value::String(base64::encode(key.to_bytes())),
            );
        }

        let val = Value::Table(map);

        std::fs::write(PATH, toml::to_string_pretty(&val).unwrap()).unwrap();
    }
}
