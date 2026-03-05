use std::error::Error;

use bevy_drm::card::Card;
use drm::{
    ClientCapability, Device as _,
    control::{Device as _, ModeTypeFlags, connector::State},
};

fn main() -> Result<(), Box<dyn Error>> {
    let card = Card::open_default()?;
    card.set_client_capability(ClientCapability::UniversalPlanes, true)?;

    let resources = card.resource_handles()?;

    for &connector_handle in resources.connectors() {
        let Ok(connector_info) = card.get_connector(connector_handle, false) else {
            continue;
        };
        if connector_info.state() != State::Connected {
            continue;
        }
        let modes = connector_info.modes();
        let Some(mode) = modes
            .iter()
            .find(|&mode| mode.mode_type().contains(ModeTypeFlags::PREFERRED))
            .or_else(|| modes.first())
        else {
            continue;
        };
        println!("{mode:?}");
        let encoder_info = if let Some(h) = connector_info.current_encoder() {
            card.get_encoder(h).ok()
        } else {
            None
        }
        .or_else(|| {
            connector_info
                .encoders()
                .iter()
                .find_map(|&h| card.get_encoder(h).ok())
        });
        let Some(encoder_info) = encoder_info else {
            continue;
        };
        let crtc_handle = resources
            .filter_crtcs(encoder_info.possible_crtcs())
            .into_iter()
            .next();
        let Some(crtc_handle) = crtc_handle else {
            continue;
        };

        for plane_handle in card.plane_handles()? {
            let plane_info = card.get_plane(plane_handle)?;
            if resources
                .filter_crtcs(plane_info.possible_crtcs())
                .contains(&crtc_handle)
            {
                // usable plane
                println!("{plane_info:?}");
            }
        }
    }
    Ok(())
}
