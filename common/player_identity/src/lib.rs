use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const PATH: &str = "player.id.ron";

pub struct PlayerIdentityPlugin;

/// Servers do not need to add this plugin.
impl Plugin for PlayerIdentityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<PlayerIdentities>::new(&["id.ron"]));
        app.init_resource::<PlayerIdentitySelection>();
        app.add_startup_system(read);
    }
}

pub fn read(mut commands: Commands, asset_server: Res<AssetServer>) {
    let path = Path::new(PATH);

    if !path.exists() {
        PlayerIdentities(Vec::new()).write();
    }

    commands.insert_resource(PlayerIdentitiesHandle(asset_server.load(path)));
}

#[derive(Serialize, Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "995f655c-ec63-4c41-bca1-2be2df3d660d"]
pub struct PlayerIdentities(pub Vec<(String, Keypair)>);

impl PlayerIdentities {
    pub fn write(&self) {
        let config = PrettyConfig::default();

        let contents = to_string_pretty(self, config).unwrap();

        std::fs::write(PATH, contents).unwrap();
    }

    pub fn add_identity<T: Into<String>>(&mut self, name: T) {
        self.0.push((name.into(), Keypair::generate(&mut OsRng {})));
    }
}

#[derive(Resource)]
pub struct PlayerIdentitiesHandle(pub Handle<PlayerIdentities>);

#[derive(Resource, Default)]
pub struct PlayerIdentitySelection(pub Option<usize>);
