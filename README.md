<p align="center">
<a href="https://docs.rs/window"><img src="https://docs.rs/window/badge.svg"></a>
<!--<a href="https://travis-ci.com/Aldarobot/window"><img src="https://api.travis-ci.com/Aldarobot/window.svg?branch=master" alt="Cala Build Status"></a>-->
<a href="https://crates.io/crates/window"><img src="https://img.shields.io/crates/v/window.svg"></a>
<a href="https://discord.gg/nXwF59K"><img src="https://img.shields.io/badge/discord-Cala%20Project-green.svg" alt="Discord"></a>
<br>
  <strong><a href="https://aldarobot.plopgrizzly.com/window">Website</a> | <a href="https://github.com/Aldarobot/window">GitHub</a> | <a href="https://aldarobot.plopgrizzly.com/window/CHANGELOG">Changelog</a> | <a href="https://aldarobot.plopgrizzly.com/window/CONTRIBUTORS">Contributors</a> | <a href="https://aldarobot.plopgrizzly.com/cala/tutorials">Tutorials</a></strong>
</p>

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

<h1>Contributing</h1>
<p>
Contributors are always welcome!  Whether it is a bug report, bug fix, feature request, feature implementation or whatever.  Don't be shy about getting involved.  I always make time to fix bugs, so usually a patched version of the library will be out soon after a report.  Features take me longer, though.  I'll also always listen to any design critiques you have.  If you have any questions you can email me at <a href="mailto:jeronlau@plopgrizzly.com">jeronlau@plopgrizzly.com</a>.  Otherwise, <a href="https://github.com/Aldarobot/window/issues">here's a link to the issues on GitHub</a>.
</p>
<p>
And, as always, make sure to always follow the <a href="https://github.com/Aldarobot/window/blob/master/CODEOFCONDUCT.md">code of conduct</a>.  Happy coding!
</p>

<h1>License</h1>
<p>
This repository is licensed under either of the following:
</p>
<ul>
<li>MIT License (MIT) - See accompanying file <a href="https://github.com/Aldarobot/window/blob/master/LICENSE_MIT.txt">LICENSE_MIT.txt</a> or copy at <a href="https://opensource.org/licenses/MIT">https://opensource.org/licenses/MIT</a></li>
<li>Boost Software License (BSL-1.0) - See accompanying file <a href="https://github.com/Aldarobot/window/blob/master/LICENSE_BSL.txt">LICENSE_BSL.txt</a> or copy at <a href="https://www.boost.org/LICENSE_1_0.txt">https://www.boost.org/LICENSE_1_0.txt</a></li>
</ul>
<p>
at your option.
</p>

<h2>Contribution Licensing</h2>
<p>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above without any additional terms or conditions.
</p>
