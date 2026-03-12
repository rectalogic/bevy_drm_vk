use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use bevy::{
    app::ScheduleRunnerPlugin,
    prelude::*,
    window::{
        PrimaryWindow, RawHandleWrapper, RawHandleWrapperHolder, WindowResolution, WindowWrapper,
    },
};

use crate::{drm::DrmWrapper, system::despawn_windows};

mod drm;
mod system;

pub struct DrmPlugin;

impl Plugin for DrmPlugin {
    fn build(&self, app: &mut App) {
        let drm = drm::Drm::new().expect("DRM should be initialized");
        let (width, height) = drm.mode().size();
        let frame_duration = Duration::from_secs_f64(1.0 / drm.mode().vrefresh() as f64);
        let drm_wrapper = DrmWrapper(Some(WindowWrapper::new(drm)));
        let raw_handle_wrapper = RawHandleWrapper::new(drm_wrapper.0.as_ref().unwrap()).unwrap();
        app.add_plugins(ScheduleRunnerPlugin::run_loop(frame_duration))
            .insert_resource(drm_wrapper)
            .add_systems(Last, despawn_windows);

        app.world_mut().spawn((
            PrimaryWindow,
            Window {
                resolution: WindowResolution::new(width as u32, height as u32)
                    .with_scale_factor_override(1.0),
                ..default()
            },
            raw_handle_wrapper.clone(),
            RawHandleWrapperHolder(Arc::new(Mutex::new(Some(raw_handle_wrapper)))),
        ));
    }
}
