use std::ops::Deref as _;

use bevy::ecs::error::BevyError;
use bevy_drm::drm::Drm;

fn main() -> Result<(), BevyError> {
    let drm = Drm::new()?;
    dbg!(drm.window_wrapper().deref());
    Ok(())
}
