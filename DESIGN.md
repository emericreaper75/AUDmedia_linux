# Design Document

## 1. Overview

This document defines the visual and interaction design for the Linux music player.

The design goal is a simple, mobile-style music experience adapted for Linux desktop.

The application should feel focused, fast, and easy to understand.

The design should avoid the dense layouts common in traditional desktop music players.

## 2. Design goals

Primary goals:

* Simple navigation
* Large, clear playback controls
* Strong focus on album artwork
* Minimal visual clutter
* Fast interaction
* Compact desktop window
* Good keyboard and mouse support
* Responsive layout
* Native Linux appearance
* Consistent spacing and typography

The application should feel comfortable in a small window and remain useful when maximized.

## 3. Design principles

### Simplicity

Every screen should have a clear primary purpose.

Avoid controls that users rarely need.

### Content first

Music information should receive more visual attention than application controls.

### Consistency

Use the same spacing, typography, icons, buttons, and interaction patterns throughout the application.

### Native behavior

Use GTK4 and libadwaita components where suitable instead of recreating standard desktop controls.

### Responsive interaction

Scanning, artwork loading, and other expensive operations must not freeze the interface.

## 4. Application structure

The application uses three primary destinations:

```text
┌───────────────────────────────┐
│                               │
│          Main content         │
│                               │
│                               │
│                               │
├───────────────────────────────┤
│ Home │ Library │ Search       │
└───────────────────────────────┘
```

A persistent mini-player appears when music is playing.

The full player opens from the mini-player.

## 5. Window

### Default window

The application should open in a compact desktop window.

Suggested initial size:

```text
Width:  420px
Height: 720px
```

The window should remain resizable.

These values are starting points, not fixed requirements.

### Large window

When the window becomes wider, content should expand without introducing unnecessary panels.

The player should preserve the mobile-style visual hierarchy.

## 6. Navigation

Primary navigation:

* Home
* Library
* Search

Use a simple bottom navigation style when the window is compact.

For larger desktop layouts, the navigation can transition to a sidebar if GTK/libadwaita conventions support the change cleanly.

Do not introduce multiple navigation layers for the MVP.

## 7. Home screen

The Home screen provides quick access to music.

Suggested structure:

```text
┌───────────────────────────────┐
│ Good morning                  │
│                               │
│ Recently played               │
│                               │
│ [Art] [Art] [Art] [Art]       │
│ Album Album Album Album       │
│                               │
│ Recently added                │
│                               │
│ Song                           │
│ Artist                         │
│ Song                           │
│ Artist                         │
│                               │
│                               │
│ ───────────────────────────── │
│ Mini player                    │
│                               │
│ Home   Library   Search       │
└───────────────────────────────┘
```

The Home screen should avoid excessive sections.

The MVP should prioritize:

* Recently played
* Recently added
* Current playback

## 8. Library screen

The Library screen should provide simple access to the music collection.

Suggested navigation:

```text
Library

[ Songs ] [ Albums ] [ Artists ]
```

### Songs

Display:

* Artwork thumbnail
* Song title
* Artist
* Album
* Optional duration

### Albums

Display:

* Album artwork
* Album title
* Artist

Use a responsive grid.

### Artists

Display:

* Artist name
* Optional number of albums

Avoid adding extensive statistics in the MVP.

## 9. Album view

Album view should focus on the album.

```text
┌───────────────────────────────┐
│ ←                             │
│                               │
│          Album Art            │
│                               │
│        Album Name             │
│        Artist                 │
│        2026 · 12 songs        │
│                               │
│        ▶ Play                 │
│                               │
│  01  Song name               │
│  02  Song name               │
│  03  Song name               │
│  04  Song name               │
│                               │
└───────────────────────────────┘
```

The primary action should be Play.

Users should be able to tap a song to start playback immediately.

## 10. Search

Search should remain simple.

```text
┌───────────────────────────────┐
│ Search                        │
│                               │
│ 🔍 Search music               │
│                               │
│ Songs                         │
│ ┌───────────────────────────┐ │
│ │ Song       Artist          │ │
│ │ Song       Artist          │ │
│ └───────────────────────────┘ │
│                               │
│ Albums                        │
│                               │
│ Artists                       │
└───────────────────────────────┘
```

Search fields:

* Song title
* Artist
* Album

Search should provide results quickly.

The MVP does not need fuzzy search unless simple matching proves insufficient.

## 11. Mini-player

The mini-player appears above primary navigation.

```text
┌───────────────────────────────┐
│ [Art] Song Name       ▶       │
│       Artist                  │
└───────────────────────────────┘
```

The mini-player should:

* Show artwork
* Show song title
* Show artist
* Provide play/pause
* Open the full player when clicked

Avoid placing too many controls in the mini-player.

## 12. Full player

The full player is the main playback screen.

```text
┌───────────────────────────────┐
│ ←                         ⋮   │
│                               │
│                               │
│          Album Art            │
│                               │
│                               │
│ Song Name                     │
│ Artist                        │
│                               │
│ ━━━━━━━━━━━○────────────      │
│ 1:42                    4:12  │
│                               │
│       ↶    ▶    ↷            │
│                               │
│   🔀                  🔁      │
│                               │
│            Queue              │
└───────────────────────────────┘
```

Primary controls:

* Previous
* Play/pause
* Next

Secondary controls:

* Shuffle
* Repeat
* Queue
* Volume

The play/pause button should receive the strongest visual emphasis.

## 13. Queue

The queue should use a simple list.

```text
Queue

Now playing
Song Name
Artist

Up next

01  Song Name
    Artist

02  Song Name
    Artist

03  Song Name
    Artist
```

Users should be able to remove songs from the queue.

Drag-to-reorder can remain optional for the MVP.

## 14. Artwork

Album artwork is an important part of the visual design.

Priorities:

1. Embedded artwork
2. Local album artwork
3. No-artwork placeholder

Artwork should use consistent aspect ratios.

Preferred ratio:

```text
1:1
```

Use rounded corners where appropriate.

Do not stretch artwork.

Large artwork should be cached after processing to avoid repeated expensive image operations.

## 15. Typography

Use the system font provided by GTK/libadwaita.

Suggested hierarchy:

```text
Page title       Large
Album title      Large
Song title       Medium
Artist           Small
Metadata         Small
Navigation       Small
```

Typography should rely on hierarchy rather than excessive font weights.

Avoid using many different font sizes.

## 16. Spacing

Use a consistent spacing system.

Suggested base unit:

```text
4px
```

Common spacing values:

```text
4px
8px
12px
16px
24px
32px
```

Primary content should have comfortable horizontal padding.

Avoid tightly packed controls.

## 17. Icons

Use standard GTK/libadwaita iconography where suitable.

Examples:

* Play
* Pause
* Previous
* Next
* Search
* Queue
* Shuffle
* Repeat
* Volume
* Back
* More

Do not mix unrelated icon styles.

The application icon should use a separate custom visual identity.

## 18. Color

Use the libadwaita color system instead of hardcoding a large custom palette.

Support:

* Light mode
* Dark mode

The application should inherit the desktop preference by default.

Album artwork should provide most of the visual variation.

Avoid excessive accent colors.

## 19. Controls

Primary controls should be visually obvious.

Example hierarchy:

```text
Primary:
Play / Pause

Secondary:
Previous
Next

Tertiary:
Shuffle
Repeat
Queue
Volume
```

Controls should have sufficient click targets for comfortable mouse use.

## 20. States

Every major screen should define these states:

### Loading

Display a simple loading indicator while scanning or loading content.

### Empty

Example:

```text
Your library is empty

Add a music folder to get started.

[ Add Music Folder ]
```

### Error

Example:

```text
Unable to play this file

The audio file could not be opened.

[ Close ]
```

### Missing file

The library should indicate when a previously indexed file no longer exists.

The application should not crash.

## 21. First-run experience

First launch should remain simple.

```text
┌───────────────────────────────┐
│                               │
│             Icon              │
│                               │
│       Your music, simply.     │
│                               │
│  Choose a folder containing   │
│  your music to get started.  │
│                               │
│       [ Add Music Folder ]    │
│                               │
└───────────────────────────────┘
```

After selecting a folder:

```text
Scanning your music...

[ progress indicator ]
```

The user should reach the library as soon as useful content becomes available.

## 22. Settings

The MVP should keep settings minimal.

Initial settings:

* Music folders
* Theme preference
* Volume behavior
* Playback behavior

Avoid creating a large settings screen.

Advanced settings belong to later releases.

## 23. Keyboard interaction

Recommended shortcuts:

```text
Space       Play / Pause
Left        Previous or seek
Right       Next or seek
Up          Volume up
Down        Volume down
Ctrl+F      Search
Esc         Close current view
```

Exact shortcuts should follow Linux and GTK conventions where possible.

## 24. Responsive behavior

### Compact

Use:

* Bottom navigation
* Large album artwork
* Single-column content
* Compact mini-player

### Medium

Use:

* Larger library grids
* More album items per row
* Wider song lists

### Large

Use:

* More content per row
* Optional sidebar navigation
* Wider player layout

Do not force desktop-specific panels into the compact layout.

## 25. Animation

Use animation sparingly.

Suitable animations:

* Screen transitions
* Mini-player expansion
* Play/pause state changes
* Artwork transitions

Avoid:

* Decorative animations
* Long transitions
* Constant movement

Animation should never delay interaction.

## 26. Accessibility

The UI should provide:

* Keyboard navigation
* Visible focus states
* Tooltips for icon-only controls
* Sufficient contrast
* Accessible control labels
* Reasonable text scaling behavior

Do not rely on icons alone when their meaning is unclear.

## 27. Performance-oriented design

The UI design must account for application performance.

Avoid:

* Rendering thousands of widgets simultaneously
* Loading full-resolution artwork for every list item
* Blocking metadata operations
* Blocking filesystem operations
* Rebuilding the entire library for small changes

Use lazy loading and caching where practical.

## 28. Visual identity

The visual identity should communicate:

* Music
* Simplicity
* Linux
* Modern desktop application

The application icon should avoid a generic standalone music note unless the final design strongly benefits from it.

The icon should remain recognizable at small sizes.

## 29. Design constraints

The MVP should not introduce:

* Complex dashboards
* Multiple sidebars
* Social feeds
* Recommendations
* Online services
* Excessive customization
* Complex visualizers
* Desktop widgets
* Large preference panels

Every visual element should have a clear purpose.

## 30. Design acceptance criteria

The design is ready for implementation when:

* A new user understands the main navigation without instructions.
* Playing a song requires one obvious interaction.
* The current song is always easy to identify.
* Playback controls are easy to reach.
* Library browsing feels simple.
* Search is easy to find.
* Empty and error states are understandable.
* The interface remains usable in a small window.
* The UI does not become cluttered when the window grows.
* Light and dark themes both remain readable.
* The application looks consistent with modern Linux desktop applications.

## 31. Implementation priority

Implement the visual system in this order:

1. Application window
2. Navigation
3. Home screen
4. Library screen
5. Song list
6. Album grid
7. Mini-player
8. Full player
9. Queue
10. Search
11. Empty/loading/error states
12. Responsive behavior
13. Keyboard interaction
14. Settings
15. Final icon and visual polish

The first UI milestone should be functional rather than polished.

Playback and navigation should work before detailed visual refinement begins.
