//! A neutral sided mod loader that loads all `.bloc` files in the folder
//! specified by the selection in resource `Profiles` in three `stages`,
//! in alphabetical order of file name, by passing `&mut app`.

use bevy::prelude::*;

pub struct ModAPIPlugin;

impl Plugin for ModAPIPlugin {
    fn build(&self, _app: &mut App) {}
}

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
    /// const NAME: &'static str = "Some Blocc Mod";
    /// ```
    const NAME: &'static str;

    /// The version of this mod.
    /// Example:
    /// ```
    /// const VERSION: &'static str = "V1";
    /// ```
    const VERSION: &'static str;

    /// The author(s) of this mod.
    /// Example:
    /// ```
    /// const AUTHORS: Authors = &["Author 1", "Author 2"];
    /// ```
    const AUTHORS: [&'static str];

    /// Called in the first stage of mod loading by the modloader.
    fn pre_init(app: &mut App);

    /// Called in the second stage of mod loading by the modloader.
    fn init(app: &mut App);

    /// Called in the third and last stage of mod loading by the modloader.
    fn post_init(app: &mut App);
}
