# Add Custom App Icon

## Summary
Replace the default Tauri placeholder icon with a custom-designed, minimal and modern coffee cup icon in monochrome style for the Restie break time reminder app.

## Motivation
The default Tauri icon doesn't reflect Restie's purpose as a break time reminder application. A custom coffee cup icon in monochrome style will:
- Make the app more recognizable and memorable
- Communicate the app's purpose (break time = coffee time)
- Align with modern, minimalist design aesthetic
- Work well across all platforms (macOS, Windows, Linux)

## Scope
- Design and create a minimal coffee cup icon in monochrome style
- Generate all required icon sizes and formats for cross-platform support
- Replace default Tauri icons in `src-tauri/icons/`
- Update `src-tauri/tauri.conf.json` if configuration changes are needed
- Ensure icons render correctly at all sizes (16px to 1024px)

## Out of Scope
- Animated icons (e.g., for notification badges)
- Alternative icon themes or seasonal variations
- SVG vector format export (only raster formats required by Tauri)

## Success Criteria
- Icon is minimalist and modern in style
- Icon clearly represents a coffee cup
- Monochrome color scheme (single shade)
- All platform-specific icon formats are generated and working
- Icons appear correctly in:
  - Application dock/taskbar
  - System tray
  - About window
  - Application folders
- CI builds include the new icons without errors

## Dependencies
None - this is a standalone visual update

## Related Changes
None

## Risks
- Icon may not render well at very small sizes (16px, 32px)
- Monochrome design may lack visual interest
- Coffee cup concept may be culturally specific

## Alternatives Considered
1. **Clock/timer icon** - More abstract, less visually distinct
2. **Pause button** - Too generic, could be confused with media apps
3. **Sun/nature** - Less directly related to "break time" activity
4. **Full color icon** - Would deviate from current minimal design direction

## Timeline
Estimated effort: 2-4 hours for design and icon generation
