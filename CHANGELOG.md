# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://github.com/AldaronLau/semver#a-guide-to-semver).

## [0.5.0] - 2021-01-05
### Changed
 - The entire API of the `input` module.  See the
   [human crate v0.2.0 changelog](https://github.com/libcala/human/blob/main/CHANGELOG.md#020---2021-01-03)
   for more details.

## [0.4.1] - 2020-11-20
### Fixed
 - Incorrect code broken by crater run for Box Custom Allocator PR.

## [0.4.0] - 2020-07-13
### Added
- `input()` for getting user input from the window.

### Changed
- Rename `Graphic` to `RasterId`
- Rename `Group::push()` to `Group::write()`
- Rename `Group::push_tex()` to `Group::write_tex()`
- `Group::write()` and `Group::write_tex()` now require location parameter, as
  well as returning the next location

### Removed
- `Key` enum
- `Window::toolbar()`
- `Window::key()`

## [0.3.1] - 2020-02-14
### Fixed
- Not compiling on most recent version of Rust (const/static issues)
- Stop using mem::zeroed for &dyn

## [0.3.0] - 2019-10-24
### Added
- Tint & Blending
- `aspect()` for getting aspect ratio

### Fixed
- Warnings for not using `dyn` keyword
- Jittery rendering
- Transformation bug where rotations would only be able to be applied to
  the identity matrix.

### Changed
- Optimized OpenGL hot loop.
- Started using index buffer objects.
- Instances are now replaced with groups

## [0.2.0] - 2019-08-02
### Added
- Toolbar support.
- Texture updating.
- `texture_coords()`.
- Culling & Depth test.
- Camera, more `Transform` functions.
- Keyboard input.
- Texture atlas support with custom mipmaps.

### Fixed
- Shape builder's `face()` not working correctly.

## [0.1.0] - 2019-07-07
### Added
- Support for Wayland + OpenGLES
