# Architecture Document

## 1. Overview

This document defines the technical architecture for the Linux music player.

The application is a local-first Linux desktop application built with Rust.

The architecture prioritizes:

* Fast development
* Simple code
* Reliable playback
* Responsive UI
* Low resource usage
* Easy maintenance
* No backend infrastructure
* No unnecessary services

The project is a short-term MVP. The architecture should support future improvements without introducing production-level complexity too early.

## 2. Architecture goals

The architecture should:

* Keep the UI responsive.
* Separate UI from playback logic.
* Separate library scanning from UI rendering.
* Keep filesystem operations outside the UI thread.
* Use GStreamer for audio playback.
* Keep music files untouched.
* Minimize dependencies.
* Avoid a database in the initial MVP.
* Make future database integration possible.
* Make future Linux distribution support practical.

## 3. Technology stack

| Layer           | Technology            | Purpose                             |
| --------------- | --------------------- | ----------------------------------- |
| Language        | Rust                  | Application development             |
| UI              | GTK4                  | Native Linux UI                     |
| UI components   | libadwaita            | Modern Linux application components |
| Audio           | GStreamer             | Audio decoding and playback         |
| Metadata        | Rust metadata library | Read audio metadata                 |
| Storage         | Local filesystem      | Music files                         |
| Runtime state   | Rust data structures  | Current library and playback state  |
| Configuration   | Local config file     | User preferences                    |
| Build           | Cargo                 | Build and dependency management     |
| Version control | Git                   | Source control                      |
| Hosting         | GitHub                | Repository and releases             |

## 4. High-level architecture

```text id="7xw0rx"
                         Linux
                           │
                           ▼
                  ┌─────────────────┐
                  │   GTK4 /        │
                  │   libadwaita    │
                  │      UI         │
                  └────────┬────────┘
                           │
                           ▼
                  ┌─────────────────┐
                  │ Application     │
                  │ State           │
                  └──────┬─────┬────┘
                         │     │
              ┌──────────┘     └──────────┐
              ▼                           ▼
      ┌───────────────┐           ┌───────────────┐
      │ Music Library │           │ Audio Player  │
      └───────┬───────┘           └───────┬───────┘
              │                           │
       ┌──────┴──────┐                    ▼
       ▼             ▼              ┌─────────────┐
 Filesystem      Metadata           │  GStreamer  │
 Scanner         Reader             └──────┬──────┘
                                           │
                                           ▼
                                      Audio Output
```

## 5. Architectural layers

The application uses four main layers.

```text id="8q5j9v"
UI
 │
 ▼
Application State
 │
 ├──────────────► Library
 │
 └──────────────► Player
```

### UI layer

Responsible for:

* Displaying information
* Handling user interaction
* Showing loading and error states
* Sending commands to application state

The UI should not directly perform filesystem scanning or audio decoding.

### Application state

Responsible for:

* Current screen
* Selected song
* Current queue
* Playback state
* Library state
* User preferences

This layer connects UI actions to application functionality.

### Library layer

Responsible for:

* Music folder management
* File discovery
* Metadata extraction
* Artwork discovery
* Search
* Library filtering

### Player layer

Responsible for:

* GStreamer pipeline
* Playback
* Pause
* Seek
* Volume
* Track changes
* Playback errors
* Playback state

## 6. Runtime data flow

### Application startup

```text id="1j8s8d"
Application starts
       │
       ▼
Load configuration
       │
       ▼
Create GTK application
       │
       ▼
Create application state
       │
       ▼
Create main window
       │
       ▼
Start library loading
       │
       ▼
Display UI
```

The UI should appear before expensive library operations finish.

## 7. Music scanning flow

```text id="i2p7oj"
Configured music folder
          │
          ▼
    Directory scanner
          │
          ▼
    Find audio files
          │
          ▼
    Metadata reader
          │
          ▼
     Song objects
          │
          ▼
     Library state
          │
          ▼
         UI
```

Scanning should run outside the GTK main thread.

The scanner should send incremental updates where practical.

The UI should not wait for the entire library before displaying usable content.

## 8. Playback flow

```text id="x9uq3w"
User selects song
        │
        ▼
Application state
        │
        ▼
Player controller
        │
        ▼
GStreamer pipeline
        │
        ▼
Audio decoder
        │
        ▼
Audio output
```

The player controller owns the GStreamer playback logic.

The UI communicates with the player through application-level commands.

## 9. UI communication

Avoid allowing individual UI widgets to directly control internal services.

Preferred flow:

```text id="0q2jcg"
User action
    │
    ▼
UI callback
    │
    ▼
Application command
    │
    ▼
Application state
    │
    ├──────► Library
    │
    └──────► Player
              │
              ▼
           GStreamer
```

This keeps UI code easier to replace or reorganize later.

## 10. Application state

The application state should contain the minimum information needed by the UI.

Conceptually:

```rust
struct AppState {
    library: Library,
    queue: Queue,
    playback: PlaybackState,
    preferences: Preferences,
}
```

The exact implementation should follow GTK's ownership and event model.

Avoid creating a large global state object with unrelated responsibilities.

## 11. Library model

A song should have a structure similar to:

```rust
struct Track {
    path: PathBuf,
    title: String,
    artist: Option<String>,
    album: Option<String>,
    album_artist: Option<String>,
    genre: Option<String>,
    year: Option<u32>,
    track_number: Option<u32>,
    disc_number: Option<u32>,
    duration: Option<Duration>,
    artwork: Option<ArtworkRef>,
}
```

The final structure should reflect the metadata library selected during implementation.

## 12. Album model

Albums should be derived from track metadata rather than stored separately in the initial MVP.

Conceptually:

```rust
struct Album {
    title: String,
    artist: Option<String>,
    tracks: Vec<TrackId>,
    artwork: Option<ArtworkRef>,
}
```

This avoids duplicate information.

## 13. Artist model

Artists should also be derived from library data.

Conceptually:

```rust
struct Artist {
    name: String,
    tracks: Vec<TrackId>,
    albums: Vec<AlbumId>,
}
```

The first version does not need a separate persistent artist database.

## 14. Library storage strategy

The MVP uses memory for the active library.

```text id="5xk5m5"
Filesystem
    │
    ▼
Scanner
    │
    ▼
Track objects
    │
    ▼
In-memory Library
```

Advantages:

* Simple
* Fast access
* No database dependency
* Easy to implement
* Easy to reset

Disadvantages:

* Initial scan still takes time
* Library state must be rebuilt after startup
* Large libraries consume memory

If startup performance becomes a real problem, add persistent metadata caching later.

Do not introduce SQLite before the problem exists.

## 15. Configuration

Configuration should contain lightweight user preferences.

Possible values:

```text id="zv6dqe"
music_folders
volume
repeat_mode
shuffle_enabled
last_track
last_position
window_size
window_state
theme_preference
```

Configuration should not contain the complete music library in the MVP.

## 16. Filesystem locations

Use platform-standard Linux directories.

Conceptually:

```text id="1nqjz2"
Configuration:
~/.config/<app-name>/

Cache:
~/.cache/<app-name>/

Data:
~/.local/share/<app-name>/
```

The final application should use the appropriate Rust/XDG path library instead of manually assuming paths.

## 17. GStreamer architecture

The player should use a GStreamer pipeline similar to:

```text id="4n2l1v"
File
 │
 ▼
Source
 │
 ▼
Decode
 │
 ▼
Audio conversion when required
 │
 ▼
Audio sink
 │
 ▼
System audio
```

GStreamer should handle format detection and decoding.

The application should avoid unnecessary audio transformations.

### Player responsibilities

The player module should expose operations such as:

```text
play(track)
pause()
resume()
stop()
seek(position)
set_volume(value)
next()
previous()
```

The exact API should follow the final implementation.

## 18. Playback events

The player needs to report events back to application state.

Examples:

```text
Started
Paused
Resumed
Stopped
PositionChanged
DurationChanged
Finished
Error
```

The UI should react to state changes rather than polling the GStreamer pipeline excessively.

## 19. Queue architecture

The queue should be independent from the library.

```text id="m4t0d8"
Library
   │
   ├── Track A
   ├── Track B
   ├── Track C
   └── Track D

Queue
   │
   ├── Track C
   ├── Track A
   └── Track D
```

This allows the user to create a playback order without changing library order.

Shuffle should operate on queue playback order.

## 20. Search architecture

Search should operate on the in-memory library.

Search fields:

* Title
* Artist
* Album

Initial implementation can use straightforward case-insensitive matching.

Conceptually:

```text
Search query
     │
     ▼
Library tracks
     │
     ▼
Filter
     │
     ▼
Search results
```

Do not add a search engine dependency for the MVP.

## 21. Artwork architecture

Artwork sources should follow a simple priority:

```text
Embedded artwork
       ↓
Local album artwork
       ↓
Placeholder
```

Artwork should be loaded asynchronously.

Use cached thumbnails for list and grid views.

Avoid keeping large original artwork images in memory when smaller versions are sufficient.

## 22. Threading model

GTK requires UI work on the main thread.

The application should use background tasks for expensive work.

```text
                 ┌──────────────┐
                 │ GTK Main     │
                 │ Thread       │
                 └──────┬───────┘
                        │
              Events / State Updates
                        │
        ┌───────────────┴───────────────┐
        ▼                               ▼
 Library Worker                    Other Tasks
        │
        ├── filesystem scanning
        ├── metadata extraction
        └── artwork processing
```

Do not perform large directory scans or metadata extraction directly inside GTK callbacks.

## 23. Concurrency approach

Start with the simplest Rust concurrency model that works correctly.

Preferred approach:

* GTK main thread for UI
* Background worker for library scanning
* Message passing for results
* GStreamer event handling through its normal integration

Avoid introducing Tokio unless a concrete asynchronous requirement needs it.

The application has no network workload in the MVP.

## 24. Error architecture

Errors should remain structured internally.

Possible categories:

```text
FileError
MetadataError
ArtworkError
PlaybackError
ConfigurationError
```

User-facing errors should be simple.

Example:

```text
Unable to play this song.

The audio file could not be opened.
```

Developer-facing logs should contain more useful details.

## 25. Logging

Use Rust logging rather than scattered `println!` calls.

Logging levels should include:

* Error
* Warn
* Info
* Debug

Normal operation should avoid excessive logging.

Debug logs should help diagnose:

* Scanner failures
* Metadata failures
* GStreamer errors
* Configuration problems

## 26. Security model

The application does not need authentication.

Security requirements:

* Do not execute files found during music scanning.
* Treat metadata as untrusted input.
* Validate filesystem paths.
* Avoid unsafe shell commands.
* Do not load remote resources in the MVP.
* Do not collect user data.
* Do not include secrets in the repository.

Music files should be treated as data only.

## 27. Dependency strategy

Keep dependencies minimal.

Core dependencies should cover:

* GTK4
* libadwaita
* GStreamer
* Metadata parsing
* Configuration
* Logging
* XDG directories where required

Before adding a dependency, verify whether GTK, GStreamer, Rust standard library, or an existing project dependency already provides the required functionality.

Avoid adding large frameworks for small features.

## 28. Repository structure

Recommended repository:

```text id="q4obm0"
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
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
└── .gitignore
```

Keep build artifacts out of Git.

## 29. Module responsibilities

### `main.rs`

Responsible for:

* Application entry point
* Application initialization
* Starting GTK

### `app.rs`

Responsible for:

* Application state
* Application-level commands
* Connecting major components

### `player.rs`

Responsible for:

* GStreamer
* Playback controls
* Playback events
* Audio errors

### `library.rs`

Responsible for:

* Music folders
* Scanning
* Track collection
* Search
* Library filtering

### `metadata.rs`

Responsible for:

* Audio metadata extraction
* Artwork discovery

### `queue.rs`

Responsible for:

* Queue ordering
* Shuffle
* Repeat
* Next/previous behavior

### `ui/`

Responsible for:

* Screens
* Components
* Navigation
* User interaction
* Rendering application state

## 30. Dependency direction

Modules should follow a predictable dependency direction.

```text id="0sjw6m"
UI
 │
 ▼
Application State
 │
 ├── Library
 │    └── Metadata
 │
 └── Player
      └── GStreamer
```

Lower-level modules should not depend on UI modules.

For example:

```text
player.rs
```

should not know whether the current screen is Home or Player.

This separation allows the UI to change without rewriting playback logic.

## 31. Database decision

No database in MVP.

Reason:

* Local library only
* Short-term project
* Small architecture
* Faster development
* Less code
* Fewer migration concerns

Reconsider persistent database storage if testing shows:

* Slow startup
* Very large libraries
* Expensive repeated metadata scans
* Need for advanced library queries
* Need for persistent playback history

If required later, SQLite is the preferred direction.

## 32. Caching strategy

MVP caching should remain limited.

Cache:

* Processed artwork thumbnails
* Optional metadata scan information if needed

Do not cache:

* Full audio files
* Transcoded audio
* Duplicate music

The application should never create unnecessary copies of the user's music.

## 33. Startup strategy

Startup should follow this order:

```text
1. Initialize application
2. Load configuration
3. Create window
4. Display UI
5. Load cached/basic state
6. Start library scan
7. Update UI incrementally
```

The application should avoid:

```text
Launch
  ↓
Scan entire music collection
  ↓
Read all metadata
  ↓
Load all artwork
  ↓
Finally show UI
```

The second approach produces poor perceived performance.

## 34. Performance targets

Initial practical targets:

* UI appears quickly after launch.
* Basic controls remain responsive during scanning.
* Search responds quickly for normal libraries.
* Playback starts without unnecessary preprocessing.
* Artwork loading does not block navigation.
* Large libraries do not freeze the application.

Exact numerical targets should be established after an initial working prototype provides real measurements.

## 35. Testing architecture

Testing should follow module boundaries.

```text
Unit tests
   │
   ├── Library
   ├── Queue
   ├── Search
   ├── Metadata
   └── Configuration

Integration tests
   │
   ├── File scanning
   ├── Playback
   └── Persistence

Manual tests
   │
   ├── UI
   ├── Audio formats
   └── Linux integration
```

Prioritize tests around playback and library scanning.

## 36. CI architecture

GitHub Actions should initially perform:

```text
Push
  │
  ▼
Checkout
  │
  ▼
Install Rust
  │
  ▼
Install required Linux dependencies
  │
  ▼
cargo fmt --check
  │
  ▼
cargo check
  │
  ▼
cargo test
  │
  ▼
cargo clippy
```

Packaging should be added after the basic CI workflow works.

## 37. Build configuration

Development:

```text
cargo run
```

Check:

```text
cargo check
```

Test:

```text
cargo test
```

Format:

```text
cargo fmt
```

Lint:

```text
cargo clippy
```

Release:

```text
cargo build --release
```

The exact Debian dependency installation commands should be documented separately in the README.

## 38. Deployment architecture

### Development

```text
Source
  ↓
Cargo
  ↓
Rust binary
  ↓
Debian
```

### Initial release

```text
GitHub
  │
  ├── Source
  ├── Release
  └── Debian-compatible package
```

### Future

```text
GitHub Release
   ├── .deb
   ├── AppImage
   └── Flatpak
```

Do not build all packaging targets before the core application stabilizes.

## 39. Data migration

The MVP should minimize persistent application data.

If configuration format changes:

* Detect the old format.
* Migrate simple values where practical.
* Provide safe defaults for missing values.
* Never delete music files during migration.

A formal database migration system is unnecessary until persistent database storage exists.

## 40. Backup considerations

The application does not own the user's music collection.

Therefore:

* Do not create backup copies of music.
* Do not modify music files automatically.
* Document where application configuration and cache data live.
* Keep important application state small enough for users to recreate.

## 41. Future extensibility

The architecture should allow future features without forcing them into the MVP.

Potential future modules:

```text
src/
├── organizer.rs
├── tag_editor.rs
├── converter.rs
├── analyzer.rs
└── duplicate_finder.rs
```

These should only be added when their features are approved.

The core player should remain independent from these tools.

## 42. Architectural decisions

### ADR-001: Rust

Decision:

Use Rust.

Reason:

* Strong performance
* Memory safety
* Good desktop application ecosystem
* Suitable for a small native application
* Good long-term learning value

### ADR-002: GTK4

Decision:

Use GTK4.

Reason:

* Native Linux UI
* Good Rust bindings
* Suitable for Debian
* Works well with libadwaita

### ADR-003: libadwaita

Decision:

Use libadwaita.

Reason:

* Modern GNOME/Linux design patterns
* Responsive components
* Consistent styling
* Reduced custom UI work

### ADR-004: GStreamer

Decision:

Use GStreamer.

Reason:

* Mature Linux multimedia framework
* Broad codec support through plugins
* Handles playback complexity
* Avoids implementing codecs manually

### ADR-005: No database in MVP

Decision:

Use in-memory library state.

Reason:

* Short-term project
* Local files only
* Smaller implementation
* Less infrastructure
* Easier development

### ADR-006: No backend

Decision:

No backend service.

Reason:

* No accounts
* No streaming
* No synchronization
* No cloud features
* No server-side functionality required

### ADR-007: Background library scanning

Decision:

Perform filesystem and metadata scanning outside the UI thread.

Reason:

* Keeps UI responsive
* Prevents large libraries from freezing the application
* Supports incremental library updates

## 43. Architecture constraints

The implementation should avoid:

* Microservices
* Backend servers
* Cloud databases
* Web frameworks
* Electron
* Tokio without a clear need
* Large state-management frameworks
* Premature database integration
* Custom audio codecs
* Automatic music-file modification

## 44. Architecture acceptance criteria

The architecture is ready for implementation when:

* GTK can create the main application window.
* GStreamer can play a local audio file.
* UI code does not directly manage filesystem scanning.
* Library scanning runs outside the UI thread.
* Player logic remains independent from individual screens.
* Music files remain untouched.
* Configuration remains local.
* No backend is required.
* Core functionality works without network access.
* The project builds through Cargo.
* Basic checks run through GitHub Actions.

## 45. Initial implementation boundary

The first implementation should contain only:

```text id="9d0kqs"
Rust
  │
  ├── GTK4
  ├── libadwaita
  ├── GStreamer
  │
  ├── File scanner
  ├── Metadata reader
  ├── In-memory library
  ├── Queue
  └── Basic configuration
```

The first technical milestone is:

```text
Launch
  ↓
GTK window
  ↓
Select music folder
  ↓
Scan audio files
  ↓
Display tracks
  ↓
Select track
  ↓
GStreamer playback
```

Once this path works reliably, build the remaining UI and playback features around the stable core.
