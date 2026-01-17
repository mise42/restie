# App Icons Specification

## Overview
This specification defines requirements for application icons in the Restie break time reminder app. Icons must be visually consistent, platform-appropriate, and communicate the app's purpose.

## ADDED Requirements

### Requirement: Icon Design Concept
The app icon SHALL clearly represent the purpose of Restie as a break time reminder application.

#### Scenario: User sees icon in dock/taskbar
Given the user has Restie installed on their desktop
When the user looks at the application icon in dock/taskbar
Then the icon MUST immediately communicate "break time" or "rest period" through visual design

#### Scenario: User distinguishes Restie from other apps
Given the user has multiple apps installed
When the user browses their installed applications
Then the Restie icon MUST be visually distinct from other productivity, timer, or system apps

### Requirement: Icon Style
The app icon SHALL follow a minimal and modern design aesthetic using a monochrome color scheme.

#### Scenario: User views icon at standard sizes
Given the user sees the icon at 32x32, 128x128, or 512x512 pixels
When the user inspects the icon design
Then the icon MUST use geometric shapes, clean lines, and a single color palette

#### Scenario: User views icon on high-DPI displays
Given the user has a high-DPI or retina display
When the icon is rendered at 2x or 3x scaling
Then the icon MUST maintain visual quality without pixelation or blurring

### Requirement: Icon Recognition
The app icon SHALL be clearly recognizable at all supported sizes from 16px to 1024px.

#### Scenario: Icon rendered at smallest size
Given the icon is displayed at 16x16 pixels (e.g., in file list or system tray)
When the user views the icon
Then the icon design MUST remain identifiable without losing clarity

#### Scenario: Icon rendered at largest size
Given the icon is displayed at 1024x1024 pixels (e.g., in App Store or installer)
When the user views the icon
Then the icon MUST show all design details without artifacts or banding

### Requirement: Platform Icon Formats
The app SHALL provide icon files in all formats required by target platforms (macOS, Windows, Linux).

#### Scenario: App builds for macOS
Given the developer builds the Tauri app for macOS
When the build process completes
Then the `src-tauri/icons/` directory MUST contain a valid `icon.icns` file with all required resolutions

#### Scenario: App builds for Windows
Given the developer builds the Tauri app for Windows
When the build process completes
Then the `src-tauri/icons/` directory MUST contain a valid `icon.ico` file with embedded resolutions (16x16, 32x32, 48x48, 256x256)

#### Scenario: App builds for Linux
Given the developer builds the Tauri app for Linux
When the build process completes
Then the `src-tauri/icons/` directory MUST contain PNG files at 32x32, 128x128, and 128x128@2x (256x256) sizes

### Requirement: Icon Configuration
The Tauri configuration SHALL correctly reference all icon files for bundling.

#### Scenario: Developer updates icon files
Given the developer has replaced icon files in `src-tauri/icons/`
When the app is built with `bun run tauri:build`
Then all platform-specific builds MUST include the new icon files without configuration errors

#### Scenario: User runs development build
Given the developer runs `bun run tauri:dev`
When the development app window opens
Then the window title bar and application menu MUST display the custom icon

### Requirement: Icon Accessibility
The app icon SHALL be accessible to users with visual impairments through sufficient contrast and shape recognition.

#### Scenario: Icon on light background
Given the icon is displayed on a light-colored background
When the user views the icon
Then the icon MUST maintain a minimum contrast ratio of 4.5:1 (WCAG AA)

#### Scenario: Icon on dark background
Given the icon is displayed on a dark-colored background
When the user views the icon
Then the icon MUST maintain a minimum contrast ratio of 4.5:1 (WCAG AA) or use a light mode variant

#### Scenario: Silhouette test
Given the icon is converted to a black-and-white silhouette
When a user identifies the icon
Then the silhouette MUST still clearly represent the intended concept (e.g., coffee cup)

## Notes

### Icon File Locations
All icon files MUST be stored in `src-tauri/icons/` directory as specified in `tauri.conf.json`:

```json
{
  "bundle": {
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

### Size Guidelines for Master Design
- **Canvas**: 1024x1024 pixels (master design size)
- **Safe zone**: Keep design within inner 819x819px (80% of canvas)
- **Corner radius**: 22% (approximately 225px radius) for modern app aesthetic
- **Padding**: 4-8% (40-80px) around design for visual balance

### Recommended Design Tools
- Vector design: Figma, Sketch, Inkscape
- Raster editing: Photopea, GIMP, Photoshop
- Batch export: ImageMagick CLI tools

### Testing Checklist
After icon implementation, verify:
- [ ] Icon appears in app title bar (development mode)
- [ ] Icon appears in dock/taskbar (all platforms)
- [ ] Icon appears in system tray (if applicable)
- [ ] Icon appears in About/About Restie dialog
- [ ] Icon appears in application folder (Finder/Explorer)
- [ ] No pixelation or blurring at any size
- [ ] Recognizable at 16x16 pixels
- [ ] CI builds complete without icon-related errors

## Related Capabilities
None - this is a standalone capability.
