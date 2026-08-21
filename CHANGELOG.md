# Changelog

All notable changes to this project will be documented in this file.

The project follows Semantic Versioning where practical.

Format:

```text
MAJOR.MINOR.PATCH
```

* MAJOR: incompatible changes
* MINOR: new backward-compatible features
* PATCH: backward-compatible fixes

## [Unreleased]

### Added

Nothing yet.

### Changed

Nothing yet.

### Fixed

Nothing yet.

### Removed

Nothing yet.

## [0.1.0] - 2026-08-21

Initial MVP release.

### Added

* Rust-based Linux music player
* GTK4 application interface
* libadwaita integration
* GStreamer audio playback
* Local music folder selection
* Music library scanning
* Audio metadata extraction
* Album artwork
* Songs view
* Albums view
* Artists view
* Music search
* Play and pause
* Previous and next track controls
* Playback seeking
* Volume control
* Queue
* Shuffle
* Repeat
* Recently played music
* Basic playback state persistence
* MPRIS D-Bus integration
* Keyboard and media-key controls
* Mobile-style user interface
* Debian Linux support

### Supported audio formats

Initial testing targets:

* MP3
* FLAC
* WAV
* OGG
* AAC

Actual format support depends on the GStreamer plugins installed on the user's system.

### Known limitations

* Local music only
* No streaming services
* No user accounts
* No cloud synchronization
* No music organization tools
* No audio conversion
* No lyrics integration
* No equalizer
* No audio visualizer
* Debian is the primary tested distribution
* Other Linux distributions are not yet officially tested

## Release process

Before creating a release:

1. Update the version in `Cargo.toml`.
2. Update this file.
3. Run formatting checks.
4. Run tests.
5. Run Clippy.
6. Build the release binary.
7. Test the application on the target Debian environment.
8. Review the Git diff.
9. Create a Git tag.
10. Create the GitHub release.
11. Add release notes.

Recommended commands:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy
cargo build --release
```

Create a version tag:

```bash
git tag v0.1.0
```

Push the tag:

```bash
git push origin v0.1.0
```

## Changelog guidelines

When adding changes, use the following categories where relevant:

### Added

New functionality.

### Changed

Changes to existing functionality.

### Fixed

Bug fixes.

### Removed

Removed functionality.

### Security

Security-related changes.

### Performance

Meaningful performance improvements.

Keep entries short and user-focused.

Avoid documenting every internal refactor unless the change affects users or developers.

## Versioning policy

During the prototype stage, breaking changes between minor versions are acceptable when they simplify development.

Once the project reaches a stable public release, follow Semantic Versioning more strictly.

The first stable release should be:

```text
1.0.0
```

Only assign `1.0.0` when the core playback and library workflows are stable and suitable for regular use.
