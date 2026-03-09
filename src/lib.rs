use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*, window::WindowWrapper};

use crate::{
    drm::DrmWrapper,
    system::{despawn_windows, observe_window_added},
};

mod drm;
mod system;

pub struct DrmPlugin;

impl Plugin for DrmPlugin {
    fn build(&self, app: &mut App) {
        let drm = drm::Drm::new().expect("DRM should be initialized");
        let frame_duration = Duration::from_secs_f64(1.0 / drm.mode().vrefresh() as f64);
        app.add_plugins(ScheduleRunnerPlugin::run_loop(frame_duration))
            .insert_resource(DrmWrapper(Some(WindowWrapper::new(drm))))
            .add_observer(observe_window_added)
            .add_systems(Last, despawn_windows);
    }
}
