use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const PATH: &str = "player.id.ron";

pub struct PlayerIdentityPlugin;

impl Plugin for PlayerIdentityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<PlayerIdentity>::new(&["id.ron"]));
        app.add_startup_system(read);
        app.add_system(log_event);
    }
}

fn log_event(
    mut ev_asset: EventReader<AssetEvent<PlayerIdentity>>,
    assets: Res<Assets<PlayerIdentity>>,
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                info!("player.id.ron loaded");

                let player_identity = assets.get(handle).unwrap();

                for (name, _) in player_identity.0.iter() {
                    info!("...{}", name);
                }
            }
            AssetEvent::Modified { handle } => {
                info!("player.id.ron changed");

                let player_identity = assets.get(handle).unwrap();

                for (name, _) in player_identity.0.iter() {
                    info!("...{}", name);
                }
            }
            AssetEvent::Removed { .. } => {
                info!("player.id.ron removed");
            }
        }
    }
}

pub fn read(mut commands: Commands, asset_server: Res<AssetServer>) {
    let path = Path::new(PATH);

    if !path.exists() {
        PlayerIdentity(Vec::new()).write();
    }

    commands.insert_resource(PlayerIdentityHandle(asset_server.load(path)));
}

#[derive(Serialize, Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "995f655c-ec63-4c41-bca1-2be2df3d660d"]
pub struct PlayerIdentity(Vec<(String, Keypair)>);

impl PlayerIdentity {
    pub fn write(&self) {
        let config = PrettyConfig::default();

        let contents = to_string_pretty(self, config).unwrap();

        std::fs::write(PATH, contents).unwrap();
    }

    pub fn add_ident<T: Into<String>>(&mut self, name: T) {
        self.0.push((name.into(), Keypair::generate(&mut OsRng {})));
    }
}

#[derive(Resource)]
pub struct PlayerIdentityHandle(Handle<PlayerIdentity>);
