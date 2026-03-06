use bevy::{
    prelude::*,
    window::{
        ClosingWindow, PrimaryWindow, RawHandleWrapper, RawHandleWrapperHolder, WindowClosed,
        WindowClosing, WindowEvent, WindowResized, WindowWrapper,
    },
};

use crate::drm::{Drm, DrmWrapper};

pub fn observe_window_added(
    window: On<Add, PrimaryWindow>,
    mut commands: Commands,
    mut query: Query<(&mut Window, &RawHandleWrapperHolder), With<PrimaryWindow>>,
    drm: Res<DrmWrapper>,
    mut window_event_messages: MessageWriter<WindowEvent>,
    mut window_resized_messages: MessageWriter<WindowResized>,
) -> Result<()> {
    let Ok((mut primary_window, handle_holder)) = query.get_mut(window.entity) else {
        return Ok(());
    };
    let Some(ref drm) = drm.0 else {
        return Ok(());
    };

    let (width, height) = drm.mode().size();
    primary_window
        .resolution
        .set_physical_resolution(width as u32, height as u32);
    let resized = WindowResized {
        window: window.entity,
        width: width as f32,
        height: height as f32,
    };
    window_resized_messages.write(resized.clone());
    window_event_messages.write(WindowEvent::WindowResized(resized));
    if let Ok(raw_handle_wrapper) = RawHandleWrapper::new(drm) {
        commands
            .entity(window.entity)
            .insert(raw_handle_wrapper.clone());
        *handle_holder.0.lock().unwrap() = Some(raw_handle_wrapper);
    }

    Ok(())
}

// See https://github.com/bevyengine/bevy/blob/5f8270f2e049f90139a503d1e930070d926f9427/crates/bevy_winit/src/system.rs#L240
pub fn despawn_windows(
    closing: Query<Entity, With<ClosingWindow>>,
    mut closed: RemovedComponents<Window>,
    window_entities: Query<Entity, With<Window>>,
    mut drm: ResMut<DrmWrapper>,
    mut closing_event_writer: MessageWriter<WindowClosing>,
    mut closed_event_writer: MessageWriter<WindowClosed>,
    mut windows_to_drop: Local<Vec<WindowWrapper<Drm>>>,
) {
    // Drop all the windows that are waiting to be closed
    windows_to_drop.clear();
    for window in closing.iter() {
        closing_event_writer.write(WindowClosing { window });
    }
    for window in closed.read() {
        info!("Closing window {}", window);
        // Guard to verify that the window is in fact actually gone,
        // rather than having the component added
        // and removed in the same frame.
        if !window_entities.contains(window) {
            if let Some(window_wrapper) = drm.0.take() {
                // Keeping WindowWrapper that are dropped for one frame
                // Otherwise the last `Arc` of the window could be in the rendering thread, and dropped there
                // This would hang on macOS
                // Keeping the wrapper and dropping it next frame in this system ensure its dropped in the main thread
                windows_to_drop.push(window_wrapper);
            }
            closed_event_writer.write(WindowClosed { window });
        }
    }
}
