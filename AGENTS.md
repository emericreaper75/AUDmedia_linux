# AGENTS.md

## 1. Purpose

This file defines how AI coding agents should work on this project.

The project is a small Rust Linux music player designed to provide a simple, mobile-style music experience on Linux desktops.

Agents must optimize for:

* Correctness
* Simplicity
* Small changes
* Fast feedback
* Maintainability
* Native Linux behavior
* Minimal dependencies

Do not over-engineer the project.

## 2. Project context

Project type:

* Linux desktop application
* Prototype/MVP
* Local music player
* Short-term project

Primary platform:

* Debian Linux

Technology:

* Rust
* GTK4
* libadwaita
* GStreamer
* Cargo
* GitHub

The application has no:

* Backend
* Cloud service
* User account
* Streaming service
* Network dependency
* Remote database

## 3. Core product goal

Build a fast, clean music player that feels similar to a simple mobile music application.

The primary user workflow is:

```text
Launch application
       ↓
Select music folder
       ↓
Scan music
       ↓
Browse library
       ↓
Select song
       ↓
Play
       ↓
Control playback
```

The user should reach playback with minimal interaction.

## 4. Architecture rules

Follow the architecture defined in `architecture.md`.

Core architecture:

```text
GTK4 + libadwaita
        │
        ▼
Application State
        │
   ┌────┴────┐
   ▼         ▼
Library    Player
   │         │
Metadata   GStreamer
   │         │
Filesystem Audio Output
```

Agents must preserve this separation.

### UI

UI code should:

* Display application state.
* Handle user interaction.
* Send commands.
* React to state changes.

UI code should not:

* Perform large filesystem scans.
* Read thousands of metadata files synchronously.
* Implement audio playback.
* Modify music files.

### Library

Library code owns:

* Music folder management
* File discovery
* Metadata extraction
* Artwork discovery
* Search
* Filtering

### Player

Player code owns:

* GStreamer
* Playback
* Pause
* Seek
* Volume
* Track changes
* Playback events
* Playback errors

### Queue

Queue code owns:

* Playback order
* Next/previous behavior
* Shuffle
* Repeat

## 5. Source of truth

Before changing architecture or adding dependencies, inspect:

* `PRD.md`
* `design.md`
* `architecture.md`
* `AGENTS.md`

If these files use different filenames in the repository, use the actual existing filenames.

The latest explicit project requirement from the user takes priority over older documentation.

Do not silently change an architectural decision.

If a requirement forces an architectural change, explain:

* What changed
* Why it changed
* What files are affected
* What tests need updating

## 6. Agent operating rules

Before modifying code:

1. Inspect the repository structure.
2. Read relevant source files.
3. Read relevant project documentation.
4. Check existing dependencies.
5. Check existing tests.
6. Identify the smallest useful change.
7. Implement the change.
8. Run relevant checks.
9. Fix failures caused by the change.
10. Report what changed.

Do not modify files without first understanding their role.

## 7. Keep changes small

Prefer:

```text
One feature
    ↓
Small implementation
    ↓
Tests
    ↓
Validation
```

Avoid large rewrites.

Do not refactor unrelated code while implementing a feature.

Do not rename files, modules, functions, or variables without a reason.

Do not reorganize the architecture for stylistic reasons.

## 8. No speculative features

Agents must not add features because they seem useful.

Do not add:

* Streaming
* Accounts
* Cloud synchronization
* Recommendations
* Lyrics services
* Telemetry
* Analytics
* Equalizers
* Visualizers
* Audio conversion
* Music organization
* Plugin systems
* Cross-platform support

unless the user explicitly approves the feature.

## 9. Dependency rules

Before adding a crate:

1. Check whether the standard library already provides the functionality.
2. Check whether an existing dependency provides it.
3. Check whether the dependency fits the project architecture.
4. Check maintenance and compatibility.
5. Add the smallest suitable dependency.

Avoid dependencies that introduce large frameworks for small tasks.

Do not add:

* A web framework
* A backend framework
* An ORM
* A database
* Tokio

unless a specific approved requirement needs one.

## 10. Audio rules

GStreamer is the playback engine.

Agents must not implement audio codecs.

Do not unnecessarily:

* Transcode audio
* Convert audio formats
* Modify source files
* Create duplicate audio files
* Change sample rates
* Change bit depth
* Apply processing during normal playback

Normal playback should preserve the source audio quality.

Supported formats depend on the installed GStreamer plugins.

Do not claim universal format support without testing.

## 11. Filesystem rules

Music files belong to the user.

The application must treat them as read-only during normal library operations.

Scanning must not:

* Rename music
* Move music
* Delete music
* Rewrite metadata
* Modify album artwork

Future music organization tools require explicit approval.

## 12. Performance rules

Never perform expensive operations on the GTK UI thread.

Potentially expensive operations include:

* Directory traversal
* Metadata extraction
* Artwork extraction
* Artwork resizing
* Large searches
* Library rebuilding

Use background workers and communicate results back to the UI.

The application should remain responsive while scanning.

## 13. Startup rules

Prefer this startup sequence:

```text
Initialize application
        ↓
Load configuration
        ↓
Create UI
        ↓
Show window
        ↓
Start library scan
        ↓
Update library incrementally
```

Do not delay displaying the UI until the entire music library finishes scanning.

## 14. State management

Keep state ownership clear.

The application should have a small central state model.

Conceptually:

```rust
struct AppState {
    library: Library,
    queue: Queue,
    playback: PlaybackState,
    preferences: Preferences,
}
```

Do not create multiple competing sources of truth.

Do not store the same state independently in multiple UI components.

## 15. UI rules

Follow `design.md`.

The UI should remain:

* Simple
* Clean
* Compact
* Mobile-like
* Native to Linux
* Responsive

Prefer GTK4 and libadwaita components.

Do not build custom components when an appropriate native component already exists.

Avoid unnecessary panels, dialogs, menus, and configuration screens.

## 16. Responsive design

The application must work in a compact window.

Compact layout:

* Bottom navigation
* Single-column content
* Large artwork
* Mini-player

Larger layouts may use:

* Wider grids
* More content per row
* Sidebar navigation where appropriate

Do not sacrifice the mobile-style experience to make the application look like a traditional desktop media player.

## 17. Error handling

Do not use `unwrap()` or `expect()` for operations where failure is reasonably possible.

Examples:

* Opening files
* Reading metadata
* Loading artwork
* Loading configuration
* GStreamer operations
* Filesystem access

Handle expected failures explicitly.

An invalid music file should not crash the application.

One broken file should not stop the entire library scan.

User-facing errors should be simple.

Developer-facing logs should contain useful diagnostic information.

## 18. Logging

Use the project's logging system.

Do not scatter debug `println!` statements throughout the application.

Use appropriate levels:

```text
error
warn
info
debug
```

Avoid excessive logging during normal playback or library scanning.

## 19. Concurrency

Keep concurrency simple.

Preferred model:

```text
GTK main thread
       │
       ├── UI
       └── application state

Background worker
       │
       ├── filesystem scanning
       ├── metadata extraction
       └── artwork processing
```

Use message passing for communication.

Do not introduce an asynchronous runtime without a concrete requirement.

## 20. Database policy

The MVP does not use a database.

Do not introduce SQLite simply because a library application often uses a database.

Consider persistent database storage only if real testing demonstrates a need such as:

* Large library startup performance
* Expensive repeated metadata scanning
* Advanced library queries
* Large persistent history requirements

If a database becomes necessary, document the architectural change before implementing it.

## 21. Configuration and cache

Use standard Linux user directories.

Conceptually:

```text
~/.config/<app-name>/
~/.cache/<app-name>/
~/.local/share/<app-name>/
```

Use an appropriate Rust/XDG library rather than hardcoding paths.

Never store secrets in configuration files unless an approved future requirement specifically requires secure credential storage.

## 22. Artwork rules

Artwork priority:

```text
Embedded artwork
       ↓
Local album artwork
       ↓
Placeholder
```

Artwork loading should not block the UI.

Use appropriately sized cached artwork for list and grid views.

Do not keep unnecessary full-resolution images in memory.

## 23. Search rules

Initial search should remain simple.

Search:

* Song title
* Artist
* Album

Use case-insensitive matching.

Do not add a search engine dependency for the MVP.

Optimize only after measuring a real performance problem.

## 24. Queue rules

The queue is independent from the library.

Changing the queue must not change library ordering.

Shuffle should affect playback order.

Repeat behavior should be explicit and predictable.

Agents must add tests when changing queue behavior.

## 25. Testing requirements

Every meaningful logic change should have appropriate tests.

Prioritize:

* Queue behavior
* Shuffle
* Repeat
* Search
* Metadata handling
* Library scanning
* Configuration
* Playback state

For UI changes, perform manual verification when automated UI testing is not practical.

## 26. Required validation

Before considering a code change complete, run the relevant checks.

At minimum when practical:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy
```

For formatting changes:

```bash
cargo fmt
```

For release builds:

```bash
cargo build --release
```

Do not claim tests passed unless they actually ran successfully.

If a check cannot run because of missing system dependencies, report the exact blocker.

## 27. Build validation

The first target is Debian Linux.

Agents should verify:

* Rust compilation
* GTK4 dependencies
* libadwaita dependencies
* GStreamer dependencies
* Application startup
* Local audio playback

Do not assume another Linux distribution behaves identically to Debian.

## 28. Git rules

Agents should keep commits focused when asked to create commits.

Preferred commit structure:

```text
feat: add basic audio playback
fix: handle missing audio files
refactor: separate queue state
test: add queue shuffle tests
docs: update installation instructions
```

Do not combine unrelated changes in one commit.

Do not rewrite Git history unless explicitly requested.

Do not force-push unless explicitly requested.

## 29. GitHub rules

Never commit:

* API keys
* Passwords
* Access tokens
* Private keys
* `.env` files
* Personal credentials
* Local machine secrets

Check `.gitignore` before committing generated files.

Required repository files:

```text
README.md
LICENSE
.gitignore
Cargo.toml
Cargo.lock
```

Recommended:

```text
.github/
    workflows/
        ci.yml
```

## 30. GitHub Actions

CI should initially verify:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy
```

Linux system dependencies required by GTK, libadwaita, and GStreamer should be installed in the CI environment.

Packaging automation should come later.

Do not complicate CI before the basic build is stable.

## 31. Documentation rules

Update documentation when behavior changes.

Relevant documentation includes:

* `README.md`
* `PRD.md`
* `design.md`
* `architecture.md`

Do not update architecture documentation for every small implementation detail.

Update architecture documentation when an architectural decision changes.

## 32. Feature workflow

When implementing a new feature:

```text
Requirement
    ↓
Check PRD
    ↓
Check architecture
    ↓
Identify affected modules
    ↓
Implement smallest version
    ↓
Add tests
    ↓
Run validation
    ↓
Update documentation if required
```

If the feature is outside the approved scope, stop and request approval.

## 33. Change impact

When requirements change, inspect:

* UI
* Application state
* Library
* Player
* Queue
* Configuration
* Dependencies
* Tests
* CI
* Packaging
* Documentation

Do not silently implement a change that invalidates an existing architecture decision.

## 34. Agent decision policy

Agents should make small implementation decisions independently when:

* The decision does not change architecture.
* The decision does not add a dependency.
* The decision does not expand product scope.
* The decision follows existing documentation.
* The change is easy to reverse.

Agents should ask for approval when:

* Adding a major dependency
* Adding a new service
* Changing the architecture
* Adding a database
* Adding network functionality
* Expanding the product scope
* Changing the target platforms
* Modifying user music files
* Changing the application's core UX direction

## 35. Do not over-engineer

Avoid premature solutions.

Do not add:

* Microservices
* Event buses
* Complex dependency injection
* Large state-management systems
* Plugin architectures
* Multiple databases
* Message brokers
* Cloud infrastructure
* Complex caching layers

Start with the smallest architecture that solves the current requirement.

## 36. Code quality

Prefer:

* Clear names
* Small functions
* Small modules
* Explicit ownership
* Simple control flow
* Strong Rust types
* Useful error types
* Testable logic

Avoid:

* Giant functions
* Giant modules
* Global mutable state
* Duplicate state
* Deep abstraction layers
* Unnecessary generics
* Clever code that reduces readability

Code should be understandable to a developer learning Rust.

## 37. Rust-specific rules

Prefer idiomatic Rust.

Use:

* `Result` for recoverable errors
* `Option` for optional values
* Borrowing where practical
* Strong types for application state
* Enums for finite states
* Small structs with clear responsibilities

Avoid unnecessary:

* `clone()`
* `Arc<Mutex<_>>`
* `unsafe`
* Global state
* Complex lifetime designs

If `unsafe` becomes necessary, explain why before introducing it.

## 38. Dependency updates

Do not update dependencies without a reason.

When updating a dependency:

1. Check compatibility.
2. Run tests.
3. Run the application.
4. Check for warnings.
5. Review the resulting lockfile changes.

Avoid unrelated dependency upgrades during feature development.

## 39. Security checks

Before release, verify:

* No secrets in Git history or current files.
* No unsafe filesystem modification.
* No unexpected network calls.
* No shell execution from untrusted metadata.
* Dependencies are reasonably maintained.
* User files remain untouched.

Dependency security scanning should be added to CI when the release workflow stabilizes.

## 40. Performance testing

Do not optimize based on assumptions.

First measure.

Important scenarios:

```text
Small library
100 songs

Medium library
1,000 songs

Large library
10,000+ songs
```

Measure:

* Startup time
* Scan time
* Search responsiveness
* Memory usage
* Artwork loading
* Playback startup

Fix measured bottlenecks before adding complex optimization.

## 41. Release readiness

Before the first public release, verify:

### Required

* Application builds on Debian.
* Music folder scanning works.
* Common formats play.
* Library works.
* Search works.
* Queue works.
* Playback controls work.
* Application does not crash on invalid files.
* README contains installation instructions.
* LICENSE exists.
* No secrets are committed.

### Recommended

* GitHub Actions CI
* Screenshots
* Release notes
* Version tag
* Debian package

### Optional

* AppImage
* Flatpak
* Automated release packaging

## 42. Current implementation order

Agents should follow this order unless the user explicitly changes the priority:

```text
1. Project setup
2. GTK4 application window
3. libadwaita integration
4. GStreamer playback
5. Basic player controls
6. Music folder selection
7. File scanning
8. Metadata extraction
9. Library model
10. Library UI
11. Search
12. Queue
13. Artwork
14. Playback persistence
15. MPRIS
16. Keyboard/media controls
17. Performance work
18. Application icon
19. Debian packaging
20. GitHub release
```

Do not start later stages while a critical earlier stage remains broken.

## 43. Current MVP boundary

The MVP consists of:

```text
Local music
     ↓
Folder scanning
     ↓
Metadata
     ↓
Library
     ↓
Search
     ↓
Queue
     ↓
GStreamer playback
     ↓
Mobile-style GTK interface
```

Everything else requires explicit approval.

## 44. Definition of done

A task is complete when:

* The requested behavior works.
* The implementation follows the architecture.
* Relevant tests exist.
* Relevant checks pass.
* No unrelated files changed unnecessarily.
* Documentation is updated when required.
* No new unapproved dependency or feature was introduced.

## 45. Agent response format

After completing a development task, report:

### Changed

List the important files and changes.

### Tests

List commands actually executed and their results.

### Notes

Mention relevant limitations, blockers, or follow-up work.

Keep the report concise.

Do not claim work that was not performed.

## 46. Final rule

Build the smallest reliable solution first.

When two approaches solve the requirement equally well, prefer the one with:

1. Less code
2. Fewer dependencies
3. Fewer moving parts
4. Better Linux integration
5. Easier testing
6. Easier maintenance

The goal is a usable Linux music player, not a large software architecture.
