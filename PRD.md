# Product Requirements Document

## 1. Product overview

### Working title

Linux Mobile-Style Music Player

### Product type

Lightweight Linux desktop music player.

### Project stage

Prototype/MVP.

### Goal

Build a simple Linux music player that provides the clean, focused experience commonly found in mobile music players.

The application focuses on local music playback. It should start quickly, load music efficiently, provide a clean interface, and avoid unnecessary desktop-player complexity.

### Target platform

Primary target:

* Debian Linux

The application should use Linux-native technologies so support for other Linux distributions remains practical later.

## 2. Problem

Finding a Linux music player with a simple, mobile-like experience is difficult.

Many existing players focus on large desktop interfaces, advanced library management, plugins, or extensive configuration.

This project aims to provide a smaller alternative focused on:

* Fast access to music
* Simple navigation
* Clean visual design
* Reliable local playback
* Minimal configuration

## 3. Target users

Primary user:

* Linux desktop users
* Users with locally stored music
* Users who prefer simple interfaces
* Users who want a mobile-style music experience on desktop

Initial development is primarily driven by personal use and learning.

The project does not target large-scale commercial deployment in v1.

## 4. Product principles

The application should follow these principles:

* Simple over feature-heavy
* Fast over complex
* Local-first
* Native Linux experience
* Clean visual hierarchy
* Minimal configuration
* Reliable playback
* Small codebase
* Easy future maintenance

## 5. Scope

### Must have

* Local music playback
* Music folder selection
* Music library scanning
* Audio metadata extraction
* Album artwork
* Song list
* Album view
* Artist information
* Search
* Play/pause
* Previous/next
* Seeking
* Volume control
* Queue
* Shuffle
* Repeat
* Recently played
* Remember configured music folders
* Basic playback state persistence
* Clean application icon
* Fast application startup

### Should have

* Keyboard controls
* Media-key support
* MPRIS integration
* Drag and drop
* Background library scanning
* Missing-file handling
* Empty-library state
* Loading states
* Error messages for unsupported or corrupted files

### Optional

* Lyrics
* Equalizer
* ReplayGain
* Gapless playback
* Audio visualizer
* Advanced sorting
* Custom themes

### Future

These features remain outside the MVP:

* Audio converter
* Music tag editor
* File renamer
* Music-folder organizer
* Duplicate finder
* Audio analyzer
* Network playback
* Streaming services
* Cloud synchronization
* User accounts
* Windows support
* macOS support

Future features require explicit scope approval before implementation.

## 6. User experience

### Design direction

The interface should feel closer to a mobile music application than a traditional desktop media player.

The UI should:

* Use large, clear controls
* Keep navigation simple
* Minimize unnecessary panels
* Prioritize album artwork and song information
* Work well in a compact window
* Adapt to larger desktop windows without becoming cluttered

### Main navigation

The initial navigation should contain:

* Home
* Library
* Search

A persistent mini-player should provide access to the currently playing song.

### Home

The Home screen should provide:

* Recently played music
* Recently added music
* Quick access to albums
* Current playback

### Library

The Library should provide:

* Songs
* Albums
* Artists

The exact layout should remain simple.

### Search

Search should support:

* Song title
* Artist
* Album

Search results should update quickly.

### Player

The full player should contain:

* Album artwork
* Song title
* Artist
* Album
* Playback progress
* Current time
* Remaining time
* Play/pause
* Previous
* Next
* Shuffle
* Repeat
* Volume
* Queue access

## 7. Audio requirements

The application should rely on GStreamer for audio playback.

The application should support audio formats provided by the installed GStreamer plugins.

Initial testing should cover:

* MP3
* FLAC
* WAV
* OGG
* AAC

The application should not implement its own codecs.

Normal playback should preserve the source audio stream without unnecessary conversion or processing.

Advanced audio processing remains outside the MVP.

## 8. Music library

### Folder selection

Users should be able to select one or more music folders.

The application should remember selected folders between launches.

### Scanning

The scanner should:

* Find supported audio files
* Read metadata
* Detect artwork
* Build the in-memory library
* Run without freezing the UI
* Report invalid files without stopping the entire scan

### Performance

Library scanning should run asynchronously.

The UI should become usable before a large scan finishes.

The application should avoid rescanning every file unnecessarily during normal startup.

### File handling

The application should not move, rename, or modify user music files in the MVP.

The application should reference files in their existing locations.

## 9. Metadata

The application should read common metadata such as:

* Title
* Artist
* Album
* Album artist
* Track number
* Disc number
* Genre
* Year
* Duration

Embedded album artwork should be supported where available.

Missing metadata should not prevent playback.

## 10. Playback

The playback engine should support:

* Play
* Pause
* Stop
* Previous
* Next
* Seek
* Volume
* Queue
* Shuffle
* Repeat

The player should handle:

* Missing files
* Unsupported files
* Corrupted files
* Playback errors

A failed file should not crash the application.

## 11. Queue

The queue should allow users to:

* View upcoming songs
* Play a selected song
* Remove songs
* Clear the queue
* Continue playback after a song finishes

Shuffle should operate on the queue rather than permanently changing the library order.

## 12. Persistence

The application should remember:

* Music folders
* Last played song
* Playback position where practical
* Volume
* Repeat mode
* Shuffle state
* Basic UI preferences

The first version should avoid a database unless testing shows a clear need.

Use a lightweight local configuration/cache approach initially.

## 13. Architecture

### Technology stack

Frontend/UI:

* Rust
* GTK4
* libadwaita

Audio:

* GStreamer

Storage:

* Local filesystem
* In-memory library
* Lightweight configuration/cache

Build system:

* Cargo

Repository:

* GitHub

### Architecture flow

```text
                    Linux Desktop
                         |
                         v
                  GTK4 / libadwaita
                         |
              +----------+----------+
              |                     |
              v                     v
        Music Library            Player UI
              |                     |
              v                     v
       File + Metadata         Playback Engine
              |                     |
              +----------+----------+
                         |
                         v
                     GStreamer
                         |
                         v
                    Audio Output
```

## 14. Project structure

```text
music-player/
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── player.rs
│   ├── library.rs
│   ├── metadata.rs
│   ├── queue.rs
│   └── ui/
│       ├── mod.rs
│       ├── home.rs
│       ├── player.rs
│       ├── library.rs
│       └── components.rs
├── assets/
│   ├── icons/
│   └── artwork/
├── tests/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
└── .gitignore
```

The structure should remain flexible during the prototype phase.

Avoid creating modules without a real requirement.

## 15. Performance requirements

The application should prioritize:

* Fast startup
* Responsive UI
* Non-blocking library scanning
* Low idle resource usage
* Fast search
* Efficient artwork handling
* Efficient playback

The UI thread should not perform expensive filesystem or metadata operations.

A large library should not make the interface unusable during scanning.

## 16. Error handling

The application should gracefully handle:

* Missing music files
* Permission errors
* Unsupported formats
* Corrupted metadata
* Corrupted audio
* Missing artwork
* Invalid folders
* Playback failures

Errors should provide useful user-facing messages where appropriate.

Internal errors should provide enough information for debugging.

The application should not crash because one music file fails.

## 17. Security and privacy

The application is local-first.

MVP requirements:

* No user accounts
* No remote backend
* No telemetry
* No cloud storage
* No external music API
* No network dependency for normal playback

User music files should remain on the user's system.

## 18. Application icon

The application should have a custom icon.

Requirements:

* Recognizable at small sizes
* Simple design
* Works on light and dark desktops
* Suitable for application launchers
* Suitable for window decorations
* SVG source
* Required raster sizes generated for packaging

The icon should match the visual identity of the application.

## 19. Testing

### Unit testing

Test:

* Metadata parsing
* Queue operations
* Shuffle
* Repeat
* Library filtering
* Search
* Configuration handling

### Integration testing

Test:

* Folder scanning
* Metadata extraction
* Library creation
* Playback state
* File error handling

### Manual testing

Test at minimum:

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
* Queue behavior
* Search
* Shuffle
* Repeat

### Performance testing

Use a test library with approximately:

* 10,000 songs
* 1,000 albums
* 500 artists

The UI should remain responsive while scanning.

## 20. Development milestones

### Milestone 1: Playback prototype

Goal:

```text
Launch
  ↓
Select folder
  ↓
Find audio files
  ↓
Show songs
  ↓
Select song
  ↓
Play
```

### Milestone 2: Library

Add:

* Metadata
* Albums
* Artists
* Search
* Artwork
* Recently played

### Milestone 3: Mobile-style UI

Add:

* Home screen
* Library screen
* Search screen
* Mini-player
* Full player
* Queue

### Milestone 4: Playback polish

Add:

* Shuffle
* Repeat
* Keyboard controls
* Media keys
* MPRIS
* Playback persistence

### Milestone 5: Release preparation

Add:

* Application icon
* Error handling
* README
* Screenshots
* Tests
* Debian testing
* GitHub Actions
* Release build

## 21. Build order

The implementation order should follow technical dependency and risk:

1. Rust project
2. GTK4/libadwaita application
3. GStreamer integration
4. Basic playback
5. Folder scanning
6. Metadata extraction
7. Library model
8. Basic library UI
9. Player UI
10. Queue
11. Search
12. Artwork
13. Persistence
14. MPRIS and media controls
15. Performance testing
16. Application icon
17. Packaging
18. GitHub release

Playback should work before significant UI polish begins.

## 22. Risks and dependencies

### GStreamer plugins

Different Linux distributions package GStreamer plugins differently.

The application should document the required runtime packages.

### Audio format support

"All music files" depends on available GStreamer plugins.

The application should test common formats instead of claiming universal codec support.

### Library performance

Large libraries can make scanning expensive.

Use background workers and avoid blocking the GTK UI thread.

### Artwork

Large embedded images can consume memory.

Artwork should use caching and sensible image sizing.

### Distribution support

Debian is the first target.

Other distributions should follow after the Debian build and runtime behavior stabilizes.

## 23. Deployment

### First development stage

Run directly with Cargo.

### First public release

Provide a Debian-compatible package.

### Later

Consider:

* AppImage
* Flatpak
* Packages for additional distributions

Packaging work should remain secondary to the core player.

## 24. GitHub release

### Required

* GitHub repository
* Source code
* README
* LICENSE
* `.gitignore`
* `Cargo.toml`
* `Cargo.lock`
* Build instructions
* Runtime dependency instructions
* Screenshots
* Version tag

### Recommended

* GitHub Actions
* Automated tests
* Build verification
* Release notes
* Issue templates

### Optional

* AppImage
* Flatpak
* Automated package releases
* Contribution guide

No secrets should enter GitHub.

## 25. Documentation

The repository should contain:

### README

Explain:

* What the application does
* Screenshots
* Features
* Requirements
* Installation
* Running from source
* Supported platforms
* Known limitations
* Contributing
* License

### Development documentation

Document:

* Architecture
* Build process
* GStreamer dependencies
* Testing
* Release process

Keep documentation proportional to the project.

## 26. Cost

The planned MVP should require no paid services.

Expected development stack:

* Rust: free
* GTK4: free
* libadwaita: free
* GStreamer: free
* SQLite: not required initially
* GitHub: free for the planned repository workflow
* Server: not required
* Cloud database: not required
* API keys: not required

## 27. Out of scope

The MVP will not include:

* Streaming services
* Online music search
* User accounts
* Cloud synchronization
* Social features
* Recommendations
* Advertising
* Analytics
* Subscription features
* Mobile application
* Windows application
* macOS application
* Audio conversion
* Advanced audio editing
* Music organization automation

## 28. Success criteria

The MVP succeeds when you can:

1. Install the application on Debian.
2. Launch it quickly.
3. Select your music folder.
4. See your music library.
5. Search your library.
6. Open an album.
7. Play a song.
8. Control playback.
9. Queue songs.
10. Close and reopen the application without losing basic preferences.
11. Play common audio formats reliably.
12. Use the application without needing a manual or complex configuration.

The strongest success criterion is simple:

You should prefer using this player for your own music instead of searching for another Linux music player.

## 29. Future direction

If the prototype works well, the project can evolve into a broader Linux media toolkit.

Potential future applications include:

* Music organizer
* Tag editor
* Audio converter
* Duplicate detector
* File renamer
* Audio analyzer

These should remain separate scope decisions.

The core music player should stay simple.

## 30. Initial technical decision

The project will start with:

```text
Language       Rust
UI             GTK4 + libadwaita
Audio          GStreamer
Storage        Filesystem + in-memory library
Database       Not required for MVP
Platform       Linux
First target   Debian
Network        None
Backend        None
Repository     GitHub
Project type   Short-term prototype/MVP
```

The first implementation target is a working local audio player. UI polish, artwork, library organization, and additional Linux integration follow after playback works reliably.
