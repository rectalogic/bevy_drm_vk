use bevy::prelude::*;
use drm::{
    ClientCapability, Device,
    control::{self, Device as ControlDevice, ModeTypeFlags, PlaneType, connector::State},
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

#[derive(Debug)]
pub struct Dri(File);

pub struct RawDri {
    fd: i32,
    plane: Option<u32>,
}

impl AsFd for Dri {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl Device for Dri {}
impl ControlDevice for Dri {}

impl HasDisplayHandle for RawDri {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unsafe {
            Ok(DisplayHandle::borrow_raw(RawDisplayHandle::Drm(
                DrmDisplayHandle::new(self.fd),
            )))
        }
    }
}

impl HasWindowHandle for RawDri {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let Some(plane) = self.plane else {
            return Err(HandleError::Unavailable);
        };
        unsafe {
            Ok(WindowHandle::borrow_raw(RawWindowHandle::Drm(
                DrmWindowHandle::new(plane),
            )))
        }
    }
}

impl Dri {
    pub fn open(path: &str) -> Result<Self, io::Error> {
        let this = Self(OpenOptions::new().read(true).write(true).open(path)?);
        this.set_client_capability(ClientCapability::UniversalPlanes, true)?;
        Ok(this)
    }

    pub fn open_default() -> Result<Self, io::Error> {
        Self::open("/dev/dri/card0")
    }

    pub fn raw(&self) -> Result<Option<RawDri>, io::Error> {
        Ok(Some(RawDri {
            fd: self.as_fd().as_raw_fd(),
            plane: self.plane()?,
        }))
    }

    fn plane(&self) -> Result<Option<u32>, io::Error> {
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
                error!("Using current mode {current_mode:?} not preferred mode {preferred_mode:?}");
                // XXX we should modeset https://github.com/Smithay/drm-rs/blob/develop/examples/atomic_modeset.rs
            }

            for plane_handle in self.plane_handles()? {
                if !matches!(self.plane_type(plane_handle)?, Some(PlaneType::Primary)) {
                    continue;
                }
                let plane_info = self.get_plane(plane_handle)?;
                if resources
                    .filter_crtcs(plane_info.possible_crtcs())
                    .contains(&crtc_handle)
                {
                    return Ok(Some(plane_handle.into()));
                }
            }
        }
        Ok(None)
    }

    pub fn plane_type(&self, plane: control::plane::Handle) -> io::Result<Option<PlaneType>> {
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
