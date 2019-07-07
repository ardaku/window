# Window
Minimal Rust code for creating a window, automatically choosing a backend window manager and graphics API.

Other Rust window creation libraries require you to build for a specific backend, so I made this crate to fix the issue.  You can now make a program that runs Wayland on a machine that has Wayland installed, and will fall back to XCB if it's not installed.  And, will run OpenGLES (eventually try Vulkan first, too) if it's installed, and fall back to OpenGL if it's not installed.

Since this crate is minimal, it doesn't even handle window decoration.  If you want window decoration and GUI widgets, check out [barg](https://crates.io/crates/barg) which depends on this crate.

## Backends
### Linux Window Managers
- Wayland

### Graphics APIs
- OpenGLES

## TODO
### Other Window Managers
- XCB
- Windows WinAPI
- Android
- MacOS Cocoa
- Redox
- Nintendo Switch
- XBOX
- PlayStation
- Wasm Canvas

### Graphics APIs
- OpenGL
- WebGL
- Vulkan

## Cala
This crate is part of the [cala](https://crates.io/crates/cala) project.

## Links
- [Website](https://aldarobot.github.io/window/)
- [Cargo](https://crates.io/crates/window)
- [Documentation](https://docs.rs/window)
- [Change Log](https://aldarobot.github.io/window/CHANGELOG)
- [Contributors](https://aldarobot.github.io/window/CONTRIBUTORS)
- [Code of Conduct](https://aldarobot.github.io/window/CODEOFCONDUCT)
- [Join Zulip Chat](https://plopgrizzly.zulipchat.com/join/pp13s6clnexk03tvlnrtjvi1/)
