# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://code.plopgrizzly.com/semver/).

## [Unreleased]

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
