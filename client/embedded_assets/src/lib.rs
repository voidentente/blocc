pub const ICON: &'static [u8] = include_bytes!("../assets/icon.qoi");

pub const FONT_DREAMS: &'static [u8] = include_bytes!("../assets/fonts/DadasDreams.ttf");

pub const FONT_SQUARE: &'static [u8] = include_bytes!("../assets/fonts/UASQUARE.ttf");

pub const FONT_NOVEMBER: &'static [u8] = include_bytes!("../assets/fonts/novem___.ttf");

pub const FONT_KONGTEXT: &'static [u8] = include_bytes!("../assets/fonts/kongtext.ttf");

pub const LAUNCHER_BACKGROUND: &'static [u8] = include_bytes!("../assets/launcher/background.qoi");

// Optionally, add the static slice to a static asset server.
// This is a workaround for when a handle is required.

use bevy::asset::{AssetIo, AssetIoError, AssetServer, Metadata};
use bevy::prelude::*;
use bevy::utils::{BoxedFuture, HashMap};
use std::path::{Path, PathBuf};

pub struct EmbeddedAssetServerPlugin;

impl Plugin for EmbeddedAssetServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EmbeddedAssetServer(
            AssetServer::new(EmbeddedAssetIo::new()),
        ));
    }
}

#[derive(Resource)]
pub struct EmbeddedAssetServer(pub AssetServer);

pub struct EmbeddedAssetIo(HashMap<String, &'static [u8]>);

impl EmbeddedAssetIo {
    fn new() -> Self {
        // Assets can be added to the server like this:

        let inner = [
            ("icon".into(), ICON),
            ("font_dreams".into(), FONT_DREAMS),
            ("font_square".into(), FONT_SQUARE),
            ("font_november".into(), FONT_NOVEMBER),
            ("font_kongtext".into(), FONT_KONGTEXT),
            ("launcher_background".into(), LAUNCHER_BACKGROUND),
        ]
        .into();

        // They can then be accessed like this:
        // ```
        // fn fun(server: Res<EmbeddedAssetServer>) {
        //     let font_dreams = server.get_handle::<Font>("font_dreams");
        // }
        // ```

        Self(inner)
    }
}

impl AssetIo for EmbeddedAssetIo {
    fn load_path<'a>(&'a self, _: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
        Box::pin(async move {
            Err(AssetIoError::Io(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "unsupported in embedded server",
            )))
        })
    }

    fn read_directory(&self, _: &Path) -> Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
        Err(AssetIoError::Io(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "unsupported in embedded server",
        )))
    }

    fn watch_path_for_changes(&self, _: &Path) -> Result<(), AssetIoError> {
        Err(AssetIoError::Io(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "unsupported in embedded server",
        )))
    }

    fn watch_for_changes(&self) -> Result<(), AssetIoError> {
        Err(AssetIoError::Io(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "unsupported in embedded server",
        )))
    }

    fn get_metadata(&self, _: &Path) -> Result<Metadata, AssetIoError> {
        Err(AssetIoError::Io(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "unsupported in embedded server",
        )))
    }
}
