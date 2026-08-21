# Contributing

Thank you for contributing to this project.

This project is a small Rust Linux music player. The goal is to keep development simple, focused, and maintainable.

Before contributing, read:

* `README.md`
* `PRD.md`
* `design.md`
* `architecture.md`
* `AGENTS.md`

## 1. Project scope

The current project focuses on:

* Linux desktop
* Debian as the primary target
* Local music playback
* Music library management
* Simple mobile-style UI
* Fast startup and library loading
* Reliable audio playback

The following features are outside the current MVP:

* Streaming services
* User accounts
* Cloud synchronization
* Social features
* Recommendations
* Advertising
* Telemetry
* Audio conversion
* Music organization tools
* Advanced audio editing
* Windows support
* macOS support

Do not add features outside the approved scope without discussing them first.

## 2. Development principles

Contributors should prefer:

* Simple solutions
* Small changes
* Existing project dependencies
* Native Linux functionality
* Clear Rust code
* Testable code
* Minimal abstractions
* Measured performance improvements

Avoid:

* Premature optimization
* Unnecessary dependencies
* Large refactors
* New infrastructure
* Complex abstractions
* Unrelated changes

## 3. Technology stack

The project uses:

* Rust
* GTK4
* libadwaita
* GStreamer
* Cargo
* GitHub Actions

Do not introduce another framework without discussing the architectural impact.

## 4. Development environment

The primary development target is Debian Linux.

You need:

* Rust toolchain
* Cargo
* GTK4 development libraries
* libadwaita development libraries
* GStreamer development libraries
* Required GStreamer plugins

The exact Debian package requirements should stay documented in the README as the project dependencies stabilize.

Verify your Rust installation:

```bash id="y0m4v8"
rustc --version
cargo --version
```

## 5. Getting the source code

Clone the repository:

```bash id="5xw4ec"
git clone <repository-url>
cd <repository-directory>
```

Build the project:

```bash id="v8z0wq"
cargo build
```

Run the application:

```bash id="s9f7aa"
cargo run
```

## 6. Before making changes

Before starting work:

1. Read the relevant project documentation.
2. Check existing issues and pull requests.
3. Check whether someone is already working on the same feature.
4. Understand the existing implementation.
5. Identify the smallest appropriate change.

Do not start with a large rewrite when a small change solves the problem.

## 7. Finding something to work on

Good contribution areas include:

* Bug fixes
* Playback reliability
* Library scanning
* Metadata handling
* UI improvements
* Accessibility
* Performance improvements
* Test coverage
* Documentation
* Debian compatibility
* Linux integration

For larger changes, open an issue before implementation.

## 8. Issues

When reporting a bug, include:

* Operating system and version
* Application version or Git commit
* Steps to reproduce
* Expected behavior
* Actual behavior
* Relevant logs
* Audio format involved, when applicable

Example:

```text id="v3w1hx"
OS:
Debian 13

Application:
v0.1.0

Audio format:
FLAC

Steps:
1. Open the application.
2. Select a music folder.
3. Open an album.
4. Play track 3.

Expected:
Track starts playing.

Actual:
Playback fails.

Logs:
<relevant output>
```

Do not attach private music files unless required and safe to share.

## 9. Feature requests

Before proposing a feature, consider:

* Does the feature fit the project's purpose?
* Does it improve the core music-player experience?
* Does it add significant complexity?
* Does it require a new dependency?
* Does it require network access?
* Does it change the architecture?

The MVP prioritizes core playback and library functionality.

Large features should be discussed before implementation.

## 10. Branches

Use focused branches.

Examples:

```text id="t2x9zq"
feature/library-search
feature/mpris-support
fix/playback-error
fix/library-scan
docs/update-installation
test/queue-behavior
```

Avoid working directly on the main branch for non-trivial changes.

## 11. Commits

Keep commits small and focused.

Preferred format:

```text id="p5d4zq"
type: short description
```

Examples:

```text id="v2j9s7"
feat: add local folder scanning
feat: add queue controls
fix: handle missing audio files
fix: prevent UI freeze during scanning
test: add shuffle tests
docs: update Debian setup
refactor: separate player state
```

Use these common types:

* `feat`
* `fix`
* `test`
* `refactor`
* `docs`
* `build`
* `ci`

Avoid mixing unrelated changes in one commit.

## 12. Pull requests

A pull request should:

* Solve one focused problem.
* Explain the change.
* Explain why the change is needed.
* Include relevant tests.
* Mention known limitations.
* Avoid unrelated modifications.

Suggested structure:

```text id="b6e3qk"
## What changed

Brief description.

## Why

Reason for the change.

## Testing

Commands executed and results.

## Screenshots

Include screenshots for meaningful UI changes.

## Notes

Known limitations or follow-up work.
```

## 13. Code style

Follow standard Rust conventions.

Run:

```bash id="j4r7sz"
cargo fmt
```

Before submitting, verify:

```bash id="x3s1ak"
cargo fmt --check
cargo check
cargo test
cargo clippy
```

Fix warnings introduced by your changes.

Do not hide warnings simply to make CI pass.

## 14. Rust guidelines

Prefer:

* `Result` for recoverable errors
* `Option` for optional values
* Clear structs
* Clear enums
* Small functions
* Explicit ownership
* Idiomatic Rust

Avoid unnecessary:

* `clone()`
* Global mutable state
* `unsafe`
* Complex lifetime structures
* Deep abstraction layers
* Large generic frameworks

Do not use `unsafe` unless there is a clear technical requirement.

If `unsafe` is required, document why.

## 15. Dependency policy

Before adding a dependency:

1. Check whether the Rust standard library already solves the problem.
2. Check whether an existing dependency solves the problem.
3. Check whether the dependency fits the project architecture.
4. Check compatibility with Debian.
5. Check maintenance status.
6. Consider dependency size and complexity.

Do not add a dependency for a small convenience feature without a clear reason.

## 16. UI contributions

Follow `design.md`.

The UI should remain:

* Simple
* Clean
* Compact
* Mobile-style
* Native to Linux

Use GTK4 and libadwaita components where appropriate.

For UI changes, consider:

* Compact window size
* Large artwork
* Clear playback controls
* Keyboard navigation
* Light mode
* Dark mode
* Accessibility
* Empty states
* Loading states
* Error states

Avoid adding visual complexity without a clear user benefit.

## 17. Audio contributions

GStreamer handles audio playback.

Do not implement audio codecs.

Do not modify user audio files during playback.

Avoid unnecessary transcoding or audio processing.

When changing playback behavior, test at least:

* MP3
* FLAC
* WAV
* OGG
* AAC

Also test playback failure cases.

## 18. Library contributions

Library scanning should remain responsive.

Do not perform expensive filesystem or metadata operations on the GTK UI thread.

Test:

* Empty folders
* Large folders
* Invalid files
* Missing metadata
* Missing artwork
* Permission errors
* Deleted files

A single invalid file should not stop the complete scan.

## 19. Performance contributions

Do not optimize based on assumptions.

First identify the bottleneck.

Useful measurements include:

* Application startup time
* Library scan time
* Search time
* Artwork loading time
* Memory usage
* Playback startup time

A performance change should include a short explanation of what improved.

## 20. Testing

### Unit tests

Use unit tests for logic such as:

* Queue
* Shuffle
* Repeat
* Search
* Metadata conversion
* Configuration

### Integration tests

Use integration tests where practical for:

* Library scanning
* File handling
* Playback state
* Configuration persistence

### Manual testing

Manual testing remains important for:

* GTK UI
* Audio playback
* Media keys
* MPRIS
* Artwork
* Responsive layouts

## 21. UI testing

For meaningful UI changes:

* Test compact window size.
* Test a larger window.
* Test light mode.
* Test dark mode.
* Test keyboard navigation.
* Test empty states.
* Test loading states.
* Test error states.

Include screenshots in pull requests when visual changes are significant.

## 22. Security

Never commit:

* Passwords
* API keys
* Access tokens
* Private keys
* `.env` files
* Personal credentials

The application should not execute commands based on music metadata or filenames.

Treat external file contents as untrusted data.

Report security issues privately rather than publishing sensitive details in an issue.

## 23. AI-assisted contributions

AI coding agents are part of the project's development workflow.

AI agents must follow `AGENTS.md`.

Contributors using AI should still:

* Review generated code.
* Understand important changes.
* Run tests.
* Review the Git diff.
* Check dependencies.
* Verify behavior manually where needed.

Do not submit AI-generated code without reviewing it.

The contributor remains responsible for the submitted change.

## 24. Documentation changes

Update documentation when behavior changes.

Relevant files include:

```text id="7v7x6f"
README.md
PRD.md
design.md
architecture.md
AGENTS.md
CHANGELOG.md
```

Do not update every document for every code change.

Update only documents affected by the change.

## 25. Architecture changes

Do not make major architecture changes through an ordinary feature pull request without discussion.

Examples:

* Adding SQLite
* Adding a backend
* Adding network services
* Replacing GTK
* Replacing GStreamer
* Introducing an async runtime
* Adding a plugin system
* Adding cross-platform support

Explain:

* Current limitation
* Proposed solution
* Alternatives considered
* New dependencies
* Performance impact
* Maintenance impact

## 26. GitHub Actions

CI should verify at minimum:

```bash id="x9h7cd"
cargo fmt --check
cargo check
cargo test
cargo clippy
```

System dependencies required by GTK4, libadwaita, and GStreamer should be installed before these checks run.

Do not add complex CI pipelines until the basic workflow is reliable.

## 27. Release contributions

Before a release:

```bash id="q5m2nz"
cargo fmt --check
cargo check
cargo test
cargo clippy
cargo build --release
```

Also verify:

* Application launches.
* Music folders scan.
* Common formats play.
* Search works.
* Queue works.
* Playback controls work.
* Invalid files do not crash the application.
* README is current.
* CHANGELOG is updated.
* No secrets exist in the repository.

## 28. License

The project will use the license specified in `LICENSE`.

Contributors should ensure their contributions are compatible with the project's license.

## 29. Contribution workflow

The recommended workflow is:

```text id="j2v4bq"
Choose issue
    ↓
Create branch
    ↓
Understand existing code
    ↓
Implement small change
    ↓
Add tests
    ↓
Run formatting
    ↓
Run checks
    ↓
Review diff
    ↓
Commit
    ↓
Open pull request
    ↓
Review
    ↓
Merge
```

## 30. Definition of done

A contribution is ready when:

* The requested behavior works.
* The change follows the architecture.
* Relevant tests exist.
* Formatting passes.
* `cargo check` passes.
* Tests pass.
* Clippy passes where applicable.
* Documentation is updated when required.
* No unrelated changes remain.
* No unapproved dependencies were added.
* No unapproved features were added.

## 31. Final principle

Keep the project small.

Solve the current problem before preparing for problems that do not exist yet.

Prefer a clear implementation over a clever implementation.

The primary goal is a reliable, simple Linux music player.
