pub mod addon;
mod api;
mod globals;
mod util;

#[cfg(feature = "log")]
mod logger;

pub use self::{
    addon::{AddonFlags, AddonLoad, AddonUnload, UpdateProvider},
    api::*,
    globals::{addon_api, on_unload, ui},
};
pub use imgui;
pub use nexus_codegen::export;

/// Fields supported by the [`export`] macro.
pub struct SupportedFields {
    /// Raidcore addon id or random unique negative integer, if not on Raidcore.
    pub signature: i32,

    /// Load function of the addon.
    pub load: Option<AddonLoad>,

    /// Unload function of the addon.
    pub unload: Option<AddonUnload>,

    /// Information about the addon.
    pub flags: Option<AddonFlags>,

    /// What platform the addon is hosted on.
    pub provider: Option<UpdateProvider>,

    /// Link to the update resource.
    pub update_link: Option<&'static str>,
}

#[doc(hidden)]
pub mod __macro {
    pub use crate::globals::{deinit, init};
}
