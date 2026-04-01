# bevy_drm_vk

An attempt to support [Linux DRM/KMS](https://en.wikipedia.org/wiki/Direct_Rendering_Manager) in Bevy.

The idea was to support KMS/DRM via raw-window-handle [RawDisplayHandle::Drm](https://docs.rs/raw-window-handle/latest/raw_window_handle/enum.RawDisplayHandle.html#variant.Drm)
and [RawWindowHandle::Drm](https://docs.rs/raw-window-handle/latest/raw_window_handle/enum.RawWindowHandle.html#variant.Drm)

Support depends on this [wgpu PR](https://github.com/gfx-rs/wgpu/pull/9182).
This PR is backported to wgpu 28 and a Bevy commit that supports 28 is used.
 
You must set `WGPU_ADAPTER_NAME` env var when running the demo.

It fails in UTM/qemu.
```sh-session
$ sudo WGPU_ADAPTER_NAME="Virtio-GPU Venus (Apple M2 Pro)" target/release/examples/demo
...
thread 'main' (13089) panicked at /usr/local/cargo/git/checkouts/bevy-50d7e162b728c6c6/543b305/crates/bevy_render/src/view/window/mod.rs:363:26:
Failed to create wgpu surface: CreateSurfaceError { inner: Hal(FailedToCreateSurfaceForAnyBackend({Vulkan: InstanceError { message: "No CRTC for drm plane", source: None }})) }
```

It fails on a Rockchip/Mali GPI R36S device with panfrost drivers due to missing support in panfrost Vulkan.
```sh-session
$ WGPU_ADAPTER_NAME="Mali-G31" PAN_I_WANT_A_BROKEN_VULKAN_DRIVER=1 ./demo
WARNING: panvk is not a conformant Vulkan implementation, testing use only.

thread 'main' (61424) panicked at /usr/local/cargo/git/checkouts/bevy-50d7e162b728c6c6/543b305/crates/bevy_render/src/renderer/mod.rs:285:36:
Unable to find a GPU! Make sure you have installed required drivers! For extra information, see: https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/option.rs:2184:5
   4: core::option::Option<T>::expect
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/option.rs:971:21
   5: bevy_render::renderer::initialize_renderer::{{closure}}
             at /usr/local/cargo/git/checkouts/bevy-50d7e162b728c6c6/543b305/crates/bevy_render/src/renderer/mod.rs:285:36
   6: bevy_drm_vk::render_creation::{{closure}}
             at /mnt/bevy_drm_vk/src/lib.rs:65:77
   7: futures_lite::future::block_on::{{closure}}
             at /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-lite-2.6.1/src/future.rs:96:35
   8: std::thread::local::LocalKey<T>::try_with
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/std/src/thread/local.rs:513:12
   9: std::thread::local::LocalKey<T>::with
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/std/src/thread/local.rs:477:20
  10: futures_lite::future::block_on
             at /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-lite-2.6.1/src/future.rs:75:11
  11: bevy_drm_vk::render_creation
             at /mnt/bevy_drm_vk/src/lib.rs:66:26
  12: demo::main
             at /mnt/bevy_drm_vk/examples/demo.rs:15:34
  13: core::ops::function::FnOnce::call_once
             at /rustc/01f6ddf7588f42ae2d7eb0a2f21d44e8e96674cf/library/core/src/ops/function.rs:250:5
```
