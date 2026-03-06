use bevy::prelude::*;

use crate::system::{despawn_windows, observe_window_added};

pub mod drm; //XXX not pub
mod system;

pub struct DrmPlugin;

impl Plugin for DrmPlugin {
    fn build(&self, app: &mut App) {
        let drm = drm::Drm::new().expect("DRM should be initialized");
        let refresh_rate = drm.mode().vrefresh();
        app.insert_resource(drm)
            .add_observer(observe_window_added)
            .add_systems(Last, despawn_windows)
            .set_runner(move |app| drm_runner(app, refresh_rate));
    }
}

fn drm_runner(app: App, refresh_rate: u32) -> AppExit {
    AppExit::Success
}
