# Window
Minimal Rust code for creating a window, automatically choosing a backend window manager and graphics API.

Other Rust window creation libraries require you to build for a specific backend, so I made this crate to fix the issue.  You can now make a program that runs Wayland on a machine that has Wayland installed, and will fall back to XCB if it's not installed.  And, will run OpenGLES (eventually try Vulkan first, too) if it's installed, and fall back to OpenGL if it's not installed.

Since this crate is minimal, it doesn't even handle window decoration.  If you want window decoration and GUI widgets, check out [barg](https://crates.io/crates/barg) which depends on this crate.

## Backends
### Linux Window Managers
- Wayland
- XCB

### Graphics APIs
- OpenGLES
- OpenGL

## TODO
### Other Window Managers
- Windows WinAPI
- Android
- MacOS Cocoa
- Redox
- Nintendo Switch
- XBOX
- PlayStation
- Wasm Canvas

### Graphics APIs
- WebGL
- Vulkan
