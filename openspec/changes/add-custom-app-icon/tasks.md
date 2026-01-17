# Implementation Tasks

## Tasks

- [x] 1. Create monochrome coffee cup icon design
  - Sketch minimal coffee cup concept
  - Use geometric shapes for clean, modern look
  - Ensure readability at small sizes (16px, 32px)
  - Export as high-resolution PNG (1024x1024)

- [x] 2. Generate required icon sizes and formats
  - 32x32.png
  - 128x128.png
  - 128x128@2x.png (256x256)
  - icon.png (512x512 or higher resolution)
  - Square variants (30x30, 44x44, 71x71, 89x89, 107x107, 142x142, 150x150, 284x284, 310x310)
  - icon.ico (Windows)
  - icon.icns (macOS)

- [x] 3. Backup and replace existing icons
  - Copy existing icons to temporary backup location
  - Delete or move default Tauri icons from `src-tauri/icons/`
  - Copy new icon files to `src-tauri/icons/`

- [x] 4. Verify icon configuration
  - Check `src-tauri/tauri.conf.json` icon paths
  - Update paths if filenames changed
  - Ensure all required formats are listed

- [x] 5. Test icon display locally
  - Run `bun run tauri:dev` to test in development
  - Verify icon appears in app window title bar
  - Check system tray icon (if applicable)
  - Test on macOS (or target platform if developing locally)

- [ ] 6. Build and verify in production build
  - Run `bun run tauri:build`
  - Inspect built application bundle
  - Verify icon appears in:
    - Application executable
    - Dock/taskbar
    - About/About Restie window
    - Application folder (Finder/Explorer)

- [ ] 7. Add icons to git and commit
  - Stage new icon files
  - Ensure git LFS is not needed (icons should be <100MB)
  - Commit with descriptive message
  - Push to remote repository

- [ ] 8. Verify CI builds with new icons
  - Wait for CI pipeline to complete
  - Check that build artifacts include new icons
  - Confirm no errors related to icon files

## Dependencies
- Task 1 must complete before Task 2
- Task 2 must complete before Task 3
- Task 3 must complete before Task 4
- Task 4 must complete before Tasks 5, 6
- Task 5 can run in parallel with Task 6 (but recommend doing sequentially for local testing)
- Task 6 must complete before Task 7
- Task 7 must complete before Task 8
