# Why Alacritty and Wezterm Don't Work (Currently)

## The Problem

The current implementations try to:

1. **Wezterm**: Spawn via `wezterm cli spawn` - Creates external process, not embedded
2. **Alacritty**: Spawn via `alacritty --working-directory` - Creates external window, not embedded

These strategies don't actually embed a terminal window into the Vortex UI - they spawn separate processes.

## Solutions

### Option 1: Embed Wezterm (RECOMMENDED)
- Use Wezterm's X11/Wayland embedding capabilities
- OR use Wezterm's multiplexer (mux) server
- Pro: Full terminal features
- Con: Complex implementation

### Option 2: Embed Alacritty
- Alacritty doesn't support embedding well
- Would need to hook into its rendering
- Pro: Native rendering
- Con: Very complex, not recommended

### Option 3: Keep Fallback (CURRENT)
- Use text-based fallback (working now!)
- Pro: Works everywhere, simple
- Con: No colors, no fancy features

### Option 4: Hybrid Approach (BEST)
- Use text fallback by default
- Detect and offer Wezterm/Alacritty spawn as OPTIONS
- User chooses to open external terminal or use fallback

## Recommendation

I suggest **Option 4** - Hybrid Approach:

1. Keep fallback terminal working (you have this now) ✅
2. Add buttons in toolbar to launch external Wezterm/Alacritty
3. User can choose which to use

This gives best of both worlds:
- ✅ Integrated terminal always available (fallback)
- ✅ Can launch fancy external terminals if available
- ✅ No complex embedding code needed

## Implementation Path

### Simple (Recommended): Add Launch Buttons
```rust
// In toolbar, add buttons:
"Launch Wezterm" - Spawns external wezterm window
"Launch Alacritty" - Spawns external alacritty window
```

### Advanced: Wezterm Embedding
```rust
// Complex: Hook into Wezterm's mux server
// Requires: X11/Wayland protocol handling
// Time: ~40-60 hours of development
```

### Medium: IPC Communication
```rust
// Moderate: Use sockets to communicate with external terminals
// Requires: Custom protocol implementation
// Time: ~20-30 hours of development
```

## What Would You Prefer?

1. **Keep current fallback** - Terminal works now, add external launch buttons?
2. **Simple Wezterm embedding** - Basic embedding without full features?
3. **Full Wezterm integration** - Complete embedding with all features (time-intensive)?
4. **Something else** - What's your priority?

## Current Status

✅ **Fallback terminal is fully working**
- Commands execute
- Output displays
- Toolbar controls work
- Everything integrated

⚠️ **Wezterm/Alacritty would require:**
- Complex IPC/embedding code
- X11/Wayland protocol knowledge
- Significant development time
- May not work on all systems

## My Recommendation

**Go with Hybrid Approach:**
1. Keep fallback (you have working terminal now!)
2. Add "Launch External Terminal" button in toolbar
3. User can click to spawn Wezterm or Alacritty if available
4. Best of both: Always have embedded fallback, optionally use external

Would you like me to implement this?
