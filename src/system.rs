use bevy::{
    prelude::*,
    window::{ClosingWindow, WindowClosed, WindowClosing, WindowWrapper},
};

use crate::drm::{Drm, DrmWrapper};

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
