# Linux Music Player

A lightweight Linux music player built with Rust.

The project aims to bring a simple, mobile-style music experience to the Linux desktop.

It focuses on local music playback, a clean interface, fast library loading, and minimal configuration.

## Status

Prototype / MVP

The project is under active development.

The first target platform is Debian Linux.

## Features

### Current MVP

* Local music playback
* Music folder selection
* Music library scanning
* Audio metadata
* Album artwork
* Songs view
* Albums view
* Artists view
* Search
* Play and pause
* Previous and next
* Seeking
* Volume control
* Queue
* Shuffle
* Repeat
* Recently played
* Playback state persistence
* Clean, mobile-style interface

### Planned

* Keyboard controls
* Media-key support
* MPRIS integration
* Drag and drop
* Background library scanning
* Improved artwork caching
* Debian packaging
* AppImage
* Flatpak

### Future

The following features are outside the current MVP:

* Music file organizer
* Tag editor
* Audio converter
* Duplicate music finder
* Audio analyzer
* Lyrics
* Equalizer
* Audio visualizer
* Streaming services
* Cloud synchronization
* User accounts
* Windows support
* macOS support

## Design

The application uses a mobile-inspired interface adapted for Linux desktop.

Primary navigation:

```text id="9xy3nq"
Home
Library
Search
```

The player provides:

```text id="d7l7hl"
Album artwork
Song information
Playback progress
Previous
Play / Pause
Next
Shuffle
Repeat
Queue
Volume
```

The interface should remain usable in a compact window while adapting to larger desktop windows.

## Screenshots

Screenshots will be added after the first usable UI implementation.

```text id="g3m9nt"
Coming soon
```

## Technology

| Component          | Technology |
| ------------------ | ---------- |
| Language           | Rust       |
| UI                 | GTK4       |
| UI components      | libadwaita |
| Audio playback     | GStreamer  |
| Build system       | Cargo      |
| Repository         | GitHub     |
| Target             | Linux      |
| First distribution | Debian     |

## Architecture

The application uses a simple local-first architecture.

```text id="q8p1zk"
                    Linux Desktop
                         │
                         ▼
                  GTK4 / libadwaita
                         │
                         ▼
                  Application State
                    │           │
                    ▼           ▼
                 Library      Player
                    │           │
                Metadata     GStreamer
                    │           │
                Filesystem   Audio Output
```

The application does not require:

* A backend server
* A cloud database
* User accounts
* Network access for normal playback
* Streaming APIs

## Audio playback

The application uses GStreamer for audio playback.

Supported formats depend on the GStreamer plugins installed on the system.

Initial testing focuses on:

* MP3
* FLAC
* WAV
* OGG
* AAC

The application does not implement audio codecs itself.

Normal playback should avoid unnecessary audio conversion or processing.

## Music library

The application reads music from folders selected by the user.

The MVP does not automatically modify the user's music collection.

The application does not:

* Move music files
* Rename music files
* Delete music files
* Rewrite metadata
* Replace album artwork

Music organization features are planned separately for the future.

## Performance

Library scanning runs outside the GTK UI thread.

The application should:

* Show the interface quickly
* Scan music in the background
* Keep the UI responsive
* Load artwork efficiently
* Avoid unnecessary audio processing
* Search the active library quickly

The initial implementation uses an in-memory library.

A database is not required for the MVP.

## Requirements

You need:

* Linux
* Rust toolchain
* GTK4 development libraries
* libadwaita development libraries
* GStreamer
* Required GStreamer plugins

The first supported distribution is Debian.

Other distributions are planned after the initial Debian workflow becomes stable.

## Development setup

Clone the repository:

```bash
git clone <repository-url>
cd <repository-directory>
```

Install the required system dependencies for Debian.

Then verify Rust:

```bash
rustc --version
cargo --version
```

Build the project:

```bash
cargo build
```

Run the application:

```bash
cargo run
```

## Development commands

Check the project:

```bash
cargo check
```

Run tests:

```bash
cargo test
```

Format the code:

```bash
cargo fmt
```

Check formatting:

```bash
cargo fmt --check
```

Run Clippy:

```bash
cargo clippy
```

Build a release binary:

```bash
cargo build --release
```

## Project structure

```text id="n1jv9p"
linux-music-player/
├── .github/
│   └── workflows/
│       └── ci.yml
├── assets/
│   ├── icons/
│   └── screenshots/
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── player.rs
│   ├── library.rs
│   ├── metadata.rs
│   ├── queue.rs
│   └── ui/
├── tests/
├── AGENTS.md
├── PRD.md
├── design.md
├── architecture.md
├── README.md
├── CONTRIBUTING.md
├── CHANGELOG.md
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── .gitignore
```


## Testing

The project prioritizes tests around core behavior.

Important areas include:

* Music scanning
* Metadata extraction
* Search
* Queue behavior
* Shuffle
* Repeat
* Configuration
* Playback state
* Error handling

Manual testing should cover:

* MP3
* FLAC
* WAV
* OGG
* AAC
* Missing artwork
* Missing metadata
* Invalid files
* Large music libraries
* Application restart

## Performance testing

The application should remain responsive with large music collections.

A useful test library contains approximately:

```text id="0x2j5g"
10,000 songs
1,000 albums
500 artists
```

The exact performance targets will be established after the first working implementation.

## Security and privacy

The application is local-first.

The MVP does not collect:

* Music data
* User accounts
* Analytics
* Telemetry
* Remote playback data

The application does not require a network connection for normal music playback.

Never commit:

* API keys
* Passwords
* Access tokens
* Private keys
* `.env` files
* Personal credentials

## Contributing

Contributions are welcome once the project reaches a stable development stage.

Before making a significant change:

1. Read `PRD.md`.
2. Read `design.md`.
3. Read `architecture.md`.
4. Read `AGENTS.md`.
5. Check existing issues.
6. Keep the change focused.
7. Add or update tests.
8. Run the project checks.

See `CONTRIBUTING.md` for the complete contribution workflow.

## Roadmap

### Phase 1: Playback

```text id="v3nqg0"
[ ] Create Rust project
[ ] Create GTK application
[ ] Integrate GStreamer
[ ] Play local audio
[ ] Add playback controls
```

### Phase 2: Library

```text id="p2a3f8"
[ ] Select music folders
[ ] Scan music
[ ] Extract metadata
[ ] Build library
[ ] Add search
[ ] Add artwork
```

### Phase 3: UI

```text id="c8q4bn"
[ ] Home
[ ] Library
[ ] Search
[ ] Mini-player
[ ] Full player
[ ] Queue
```

### Phase 4: Linux integration

```text id="q4w7vp"
[ ] Keyboard controls
[ ] Media keys
[ ] MPRIS
[ ] Background scanning
```

### Phase 5: Release

```text id="f6p2jd"
[ ] Performance testing
[ ] Error handling
[ ] Final application icon
[ ] Debian testing
[ ] CI
[ ] Debian package
[ ] GitHub release
```

## Known limitations

The MVP intentionally has a limited scope.

Current limitations will include:

* Linux-first development
* Debian as the first tested distribution
* Local files only
* No streaming
* No cloud synchronization
* No automatic music organization
* Audio format support depends on installed GStreamer plugins

This section should be updated as development progresses.

## License

The project will use an open-source license.

The final license should be added before the first public release.

Recommended choice:

MIT License

If the project uses dependencies with additional license requirements, review their licenses before release.

## Project documentation

```text id="5wq7z4"
PRD.md
    Product requirements

design.md
    UI and interaction design

architecture.md
    Technical architecture

AGENTS.md
    AI coding-agent instructions

CONTRIBUTING.md
    Contribution workflow

CHANGELOG.md
    Release history
```

## Project goal

Build a small Linux music player that feels simple enough to use every day.

The first milestone is straightforward:

```text id="v8k2rm"
Select music folder
       ↓
Scan music
       ↓
Browse library
       ↓
Select song
       ↓
Play
```

The project should stay focused until this workflow works reliably.
