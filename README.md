# Bevy DRM

The idea was to support KMS/DRM via raw-window-handle [RawDisplayHandle::Drm](https://docs.rs/raw-window-handle/latest/raw_window_handle/enum.RawDisplayHandle.html#variant.Drm)
and [RawWindowHandle::Drm](https://docs.rs/raw-window-handle/latest/raw_window_handle/enum.RawWindowHandle.html#variant.Drm)

However these turn out not to be supported by wgpu [create_surface](https://github.com/gfx-rs/wgpu/blob/0ab6edd6a35c9d24e1e28a5ebe1ed680a578f6a1/wgpu-hal/src/vulkan/instance.rs#L874).

[create_surface_from_drm](https://github.com/gfx-rs/wgpu/blob/0ab6edd6a35c9d24e1e28a5ebe1ed680a578f6a1/wgpu-core/src/instance.rs#L303) might work, but Bevy doesn't use that.

From an Ubuntu virtual console:
```sh-session
$ bevy cargo build --example demo
$ sudo WGPU_BACKEND=vulkan target/debug/examples/demo

INFO bevy_diagnostic::system_information_diagnostics_plugin::internal: SystemInfo { os: "Linux (Ubuntu 24.04)", kernel: "6.17.0-14-generic", cpu: "", core_count: "8", memory: "3.8 GiB" }
thread 'main' (14470) panicked at /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_render-0.18.1/src/renderer/mod.rs:231:22:
Failed to create wgpu surface: CreateSurfaceError { inner: Hal(FailedToCreateSurfaceForAnyBackend({Vulkan: InstanceError { message: "window handle Drm(DrmWindowHandle { plane: 33 }) is not a Vulkan-compatible handle", source: None }})) }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
