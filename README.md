![Window Logo](https://raw.githubusercontent.com/libcala/window/main/res/icon.svg)

#### [Changelog][3] | [Source][4] | [Getting Started][5]

[![tests](https://github.com/libcala/window/workflows/tests/badge.svg)][2]
[![docs](https://docs.rs/window/badge.svg)][0]
[![crates.io](https://img.shields.io/crates/v/window.svg)][1]

Minimal Rust code for creating a window, automatically choosing a backend window
manager and graphics API.

Other Rust window creation libraries require you to build for a specific
backend, so I made this crate to fix the issue.  You can now make a program that
runs Wayland on a machine that has Wayland installed, and will fall back to XCB
if it's not installed.  And, will run OpenGLES (eventually try Vulkan first,
too) if it's installed, and fall back to OpenGL if it's not installed.

Since this crate is minimal, it doesn't even handle window decoration.  If you
want window decoration and GUI widgets, check out
[cala](https://crates.io/crates/cala) which depends on this crate.

Check out the [documentation][0] for examples.

### Features
 - Linux Wayland Support
 - Linux OpenGLES Support

### Planned Features
 - XCB
 - Windows WinAPI
 - Android
 - MacOS Cocoa
 - Redox
 - Nintendo Switch
 - XBOX
 - PlayStation
 - Wasm
 - OpenGL
 - WebGL
 - Vulkan

### Supported Platforms
Human targets all platforms that can run Rust, including:
 - Linux
 - Web Assembly **doesn't work yet**
 - Windows **doesn't work yet**
 - Mac OS **doesn't work yet**
 - BSD **doesn't work yet**
 - Android (may partially or fully work, but untested) **doesn't work yet**
 - iOS / various game consoles **doesn't work yet**
 - Redox **doesn't work yet**
 - Fuchsia **doesn't work yet**
 - Others? (make a PR)

## License
Licensed under any of
 - Apache License, Version 2.0, ([LICENSE_APACHE_2_0.txt][7]
   or [https://www.apache.org/licenses/LICENSE-2.0][8])
 - MIT License, ([LICENSE_MIT.txt][9] or [https://mit-license.org/][10])
 - Boost Software License, Version 1.0, ([LICENSE_BOOST_1_0.txt][11]
   or [https://www.boost.org/LICENSE_1_0.txt][12])

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as described above, without any additional terms or conditions.

## Help
If you want help using or contributing to this library, feel free to send me an
email at [aldaronlau@gmail.com][13].

[0]: https://docs.rs/window
[1]: https://crates.io/crates/window
[2]: https://github.com/libcala/window/actions?query=workflow%3Atests
[3]: https://github.com/libcala/window/blob/main/CHANGELOG.md
[4]: https://github.com/libcala/window/
[5]: https://docs.rs/window#getting-started
[6]: https://aldaronlau.com/
[7]: https://github.com/libcala/window/blob/main/LICENSE_APACHE_2_0.txt
[8]: https://www.apache.org/licenses/LICENSE-2.0
[9]: https://github.com/libcala/window/blob/main/LICENSE_MIT.txt
[10]: https://mit-license.org/
[11]: https://github.com/libcala/window/blob/main/LICENSE_BOOST_1_0.txt
[12]: https://www.boost.org/LICENSE_1_0.txt
[13]: mailto:aldaronlau@gmail.com
