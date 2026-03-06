use bevy::{prelude::*, window::WindowWrapper};
use drm::{
    ClientCapability, Device,
    control::{self, Device as ControlDevice, ModeTypeFlags, PlaneType, connector::State, plane},
};
use raw_window_handle::{
    DisplayHandle, DrmDisplayHandle, DrmWindowHandle, HandleError, HasDisplayHandle,
    HasWindowHandle, RawDisplayHandle, RawWindowHandle, WindowHandle,
};
use std::{
    fs::{File, OpenOptions},
    io,
    os::{
        fd::AsRawFd,
        unix::io::{AsFd, BorrowedFd},
    },
};

#[derive(Resource, Debug)]
pub struct DrmWrapper(pub Option<WindowWrapper<Drm>>);

#[derive(Debug)]
pub struct Drm {
    card: Card,
    window: DrmWindow,
}

impl Drm {
    pub fn new() -> Result<Self, BevyError> {
        Self::with_card(Card::open_default()?)
    }

    pub fn with_card_path(card_path: &str) -> Result<Self, BevyError> {
        Self::with_card(Card::open(card_path)?)
    }

    fn with_card(card: Card) -> Result<Self, BevyError> {
        let Some(drm_window) = card.drm_window()? else {
            return Err("Could not initialize DRM".into());
        };
        Ok(Self {
            card,
            window: drm_window,
        })
    }

    pub fn mode(&self) -> &control::Mode {
        &self.window.mode
    }
}

impl HasDisplayHandle for Drm {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unsafe {
            Ok(DisplayHandle::borrow_raw(RawDisplayHandle::Drm(
                DrmDisplayHandle::new(self.card.as_fd().as_raw_fd()),
            )))
        }
    }
}

impl HasWindowHandle for Drm {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        unsafe {
            Ok(WindowHandle::borrow_raw(RawWindowHandle::Drm(
                DrmWindowHandle::new(self.window.plane_handle.into()),
            )))
        }
    }
}

#[derive(Debug)]
struct Card(File);

#[derive(Debug)]
struct DrmWindow {
    mode: control::Mode,
    plane_handle: plane::Handle,
}

impl AsFd for Card {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl Device for Card {}
impl ControlDevice for Card {}

impl Card {
    fn open(path: &str) -> Result<Self, io::Error> {
        let this = Self(OpenOptions::new().read(true).write(true).open(path)?);
        this.set_client_capability(ClientCapability::UniversalPlanes, true)?;
        Ok(this)
    }

    fn open_default() -> Result<Self, io::Error> {
        Self::open("/dev/dri/card0")
    }

    fn drm_window(&self) -> Result<Option<DrmWindow>, io::Error> {
        let Some((plane_handle, mode)) = self.initialize()? else {
            return Ok(None);
        };
        Ok(Some(DrmWindow { mode, plane_handle }))
    }

    fn initialize(&self) -> Result<Option<(plane::Handle, control::Mode)>, io::Error> {
        let resources = self.resource_handles()?;

        for &connector_handle in resources.connectors() {
            let Ok(connector_info) = self.get_connector(connector_handle, true) else {
                continue;
            };
            if connector_info.state() != State::Connected {
                continue;
            }
            let modes = connector_info.modes();
            let preferred_mode = modes
                .iter()
                .find(|&mode| mode.mode_type().contains(ModeTypeFlags::PREFERRED))
                .or_else(|| modes.first());

            let Some(crtc_handle) = connector_info
                .encoders()
                .iter()
                .find_map(|&encoder_handle| {
                    resources
                        .filter_crtcs(self.get_encoder(encoder_handle).ok()?.possible_crtcs())
                        .into_iter()
                        .next()
                })
            else {
                continue;
            };

            let current_mode = self.get_crtc(crtc_handle)?.mode();
            if current_mode.as_ref() != preferred_mode {
                warn!("Using current mode {current_mode:?} not preferred mode {preferred_mode:?}");
                // XXX we should modeset https://github.com/Smithay/drm-rs/blob/develop/examples/atomic_modeset.rs
            }
            let Some(mode) = current_mode else {
                continue;
            };

            for plane_handle in self.plane_handles()? {
                if !matches!(self.plane_type(plane_handle)?, Some(PlaneType::Primary)) {
                    continue;
                }
                let plane_info = self.get_plane(plane_handle)?;
                if resources
                    .filter_crtcs(plane_info.possible_crtcs())
                    .contains(&crtc_handle)
                {
                    return Ok(Some((plane_handle, mode)));
                }
            }
        }
        Ok(None)
    }

    fn plane_type(&self, plane: control::plane::Handle) -> io::Result<Option<PlaneType>> {
        let props = self.get_properties(plane)?;

        for (&prop_handle, &raw_value) in props.iter() {
            let info = self.get_property(prop_handle)?;
            if info.name().to_bytes() == b"type" {
                let ty = match raw_value as u32 {
                    x if x == PlaneType::Primary as u32 => Some(PlaneType::Primary),
                    x if x == PlaneType::Overlay as u32 => Some(PlaneType::Overlay),
                    x if x == PlaneType::Cursor as u32 => Some(PlaneType::Cursor),
                    _ => None,
                };
                return Ok(ty);
            }
        }

        Ok(None)
    }
}
