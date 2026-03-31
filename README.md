# Bevy DRM

The idea was to support KMS/DRM via raw-window-handle [RawDisplayHandle::Drm](https://docs.rs/raw-window-handle/latest/raw_window_handle/enum.RawDisplayHandle.html#variant.Drm)
and [RawWindowHandle::Drm](https://docs.rs/raw-window-handle/latest/raw_window_handle/enum.RawWindowHandle.html#variant.Drm)

Support depends on this [wgpu PR](https://github.com/gfx-rs/wgpu/pull/9182)
 
It fails in UTM/qemu to what appears to be a virtgl bug.
It fails on a Rockchip/Mali GPI R36S device due to missing support in panfrost Vulkan.
