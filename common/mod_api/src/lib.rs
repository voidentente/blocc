//! A neutral sided mod loader that loads all `.bloc` files in the folder
//! specified by the resource `ProfileSelection` in three `stages`,
//! in a dynamically specified `order` by passing `&mut app`.

use bevy::prelude::*;

pub struct ModAPIPlugin;

impl Plugin for ModAPIPlugin {
    fn build(&self, _app: &mut App) {}
}

pub type Name = &'static str;

pub type Version = (u8, u8, u8);

pub type Authors = &'static [&'static str];

pub type Dependencies = &'static [(Name, Version, Version)];

/// Implement this trait on a struct to create a Blocc mod interface.
/// Example:
/// ```
/// pub(crate) struct MyMod;
///
/// impl BloccMod for MyMod {
///     ...
/// }
/// ```
pub trait BloccMod {
    /// The name of this mod.
    /// Example:
    /// ```
    /// const NAME: Name = "Some Blocc Mod";
    /// ```
    const NAME: Name;

    /// The version of this mod, formatted as SemVer.
    /// Example:
    /// ```
    /// const VERSION: Version = (0, 1, 0);
    /// ```
    const VERSION: Version;

    /// The author(s) of this mod.
    /// Example:
    /// ```
    /// const AUTHORS: Authors = &["Author 1", "Author 2"];
    /// ```
    const AUTHORS: Authors;

    /// The names of the mods and their minimum and maximum version requirement this mod depends on.
    /// Example:
    /// ```
    /// const DEPENDENCIES: Dependencies = [
    ///     ("World", (0, 1, 0), (0, 3, 4)),
    ///     ("Some Blocc Mod", (0, 1, 0), (0, 1, 0)),
    /// ]
    /// ```
    const DEPENDENCIES: Dependencies;

    /// Called in the first stage of mod loading by the modloader.
    fn pre_init(app: &mut App);

    /// Called in the second stage of mod loading by the modloader.
    fn init(app: &mut App);

    /// Called in the third and last stage of mod loading by the modloader.
    fn post_init(app: &mut App);
}
