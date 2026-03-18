use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use bevy::{
    app::ScheduleRunnerPlugin,
    prelude::*,
    render::{
        renderer::initialize_renderer,
        settings::{Backends, InstanceFlags, RenderCreation, WgpuSettings},
    },
    tasks,
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
        let raw_handle_wrapper_holder =
            RawHandleWrapperHolder(Arc::new(Mutex::new(Some(raw_handle_wrapper.clone()))));

        app.world_mut().spawn((
            PrimaryWindow,
            Window {
                resolution: WindowResolution::new(width as u32, height as u32)
                    .with_scale_factor_override(1.0),
                ..default()
            },
            raw_handle_wrapper,
            raw_handle_wrapper_holder.clone(),
        ));

        app.add_plugins(ScheduleRunnerPlugin::run_loop(frame_duration))
            .insert_resource(drm_wrapper)
            .add_systems(Last, despawn_windows);
    }
}

pub fn render_creation() -> RenderCreation {
    let settings = WgpuSettings {
        backends: Some(Backends::VULKAN),
        adapter_name: Some(std::env::var("WGPU_ADAPTER_NAME").unwrap().to_lowercase()),
        instance_flags: InstanceFlags::from_env_or_default(),
        ..default()
    };
    let async_renderer =
        async move { initialize_renderer(Backends::VULKAN, None, &settings).await };
    RenderCreation::from(tasks::block_on(async_renderer))
}
