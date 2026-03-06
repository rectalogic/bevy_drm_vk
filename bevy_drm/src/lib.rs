use std::time::{Duration, Instant};

use bevy::{app::PluginsState, platform::thread, prelude::*, window::WindowWrapper};

use crate::{
    drm::DrmWrapper,
    system::{despawn_windows, observe_window_added},
};

pub mod drm; //XXX not pub
mod system;

pub struct DrmPlugin;

impl Plugin for DrmPlugin {
    fn build(&self, app: &mut App) {
        let drm = drm::Drm::new().expect("DRM should be initialized");
        let frame_duration = Duration::from_secs_f64(1.0 / drm.mode().vrefresh() as f64);
        app.insert_resource(DrmWrapper(Some(WindowWrapper::new(drm))))
            .add_observer(observe_window_added)
            .add_systems(Last, despawn_windows)
            .set_runner(move |app| drm_runner(app, frame_duration));
    }
}

fn drm_runner(mut app: App, frame_duration: Duration) -> AppExit {
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();
    }
    loop {
        let start_time = Instant::now();

        app.update();

        if let Some(exit) = app.should_exit() {
            return exit;
        };
        let exe_time = Instant::now() - start_time;
        if exe_time < frame_duration {
            thread::sleep(frame_duration - exe_time);
        }
    }
}
