use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
};

pub struct TextAssetPlugin;

impl Plugin for TextAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TextAsset>();
        app.add_asset_loader(TextAssetLoader);
    }
}

#[derive(bevy::reflect::TypeUuid)]
#[uuid = "e74d5a29-3e60-4632-a988-53e09a1157a1"]
pub struct TextAsset(pub String);

pub struct TextAssetLoader;

impl AssetLoader for TextAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let slice = std::str::from_utf8(bytes)?;

            let string = slice.to_string();

            load_context.set_default_asset(LoadedAsset::new(TextAsset(string)));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}
