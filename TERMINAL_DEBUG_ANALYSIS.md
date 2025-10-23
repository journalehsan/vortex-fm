# Terminal Implementation Debug Analysis

## Issues Found

### 1. **Readonly Text Area Issue**
**File:** `src/widgets/terminal_panel.rs` lines 202-216

**Problem:**
- `output_display()` returns a `scrollable(text(...))` which is read-only by design
- This is a display widget, not an input widget
- The actual command input exists but is defined in a method that's never called

**Root Cause:**
- The main `view()` method (lines 115-143) only renders placeholder text
- It doesn't call `fallback_terminal_view()` (lines 161-173) which has the actual UI
- This means neither the output area nor the input area are ever rendered

### 2. **Terminal Toolbar Not Showing**
**File:** `src/widgets/terminal_panel.rs` & `src/widgets/terminal_toolbar.rs`

**Problem:**
- `TerminalToolbar` is defined and functional but NEVER used in `TerminalPanel`
- The toolbar has buttons and path display but it's not part of the view hierarchy

**Root Cause:**
- `terminal_panel.rs` imports nothing from `terminal_toolbar.rs`
- The toolbar is never instantiated or rendered
- Toolbar messages are never dispatched

### 3. **Terminal Not Actually Embedded**
**File:** `src/core/terminal/strategy.rs` lines 37-106

**Problem:**
- `WeztermStrategy::spawn()` and `AlacrittyStrategy::spawn()` don't actually embed anything
- They try to spawn external processes, not embed them within the UI
- Alacritty spawning happens but the terminal runs separately
- Wezterm uses CLI commands that don't create embedded terminals

**Expected Behavior:**
- Should either:
  1. Actually embed the terminal widget (if possible with Wezterm/Alacritty)
  2. Fall back to the text-based UI for command execution
- Currently attempts external spawning that doesn't integrate with the UI

### 4. **Fallback Terminal Not Properly Integrated**
**File:** `src/widgets/terminal_panel.rs` lines 161-216

**Problem:**
- `fallback_terminal_view()` method exists but is NEVER called
- Output buffer exists (`output_buffer: Vec<TerminalOutputLine>`) but never updated from command execution
- Command input area defined but disconnected from actual message handling
- `update()` method handles `CommandInput` message but `ExecuteCommand` gets empty string

**Root Cause:**
```rust
// Line 147: ExecuteCommand receives empty string
TerminalMessage::ExecuteCommand(_) => {
    // Line 149: Gets command from self.command_input instead of the message
    if !self.command_input.trim().is_empty() {
```

### 5. **Placeholder Rendering Instead of Actual UI**
**File:** `src/widgets/terminal_panel.rs` lines 115-143

**Problem:**
```rust
pub fn view(&self) -> Element<'_, TerminalMessage> {
    // This just renders debug text, not the actual terminal UI!
    widget::container(
        widget::text(format!(
            "🖥️ Terminal Panel\n...",
        ))
        // ... styling
    )
}
```

## Solutions Needed

### Solution 1: Fix Terminal Panel View
- Call `fallback_terminal_view()` instead of rendering placeholder text
- Add toolbar to the view hierarchy
- Integrate both components properly

### Solution 2: Implement Fallback Terminal UI
- Make output area interactive (though read-only for display)
- Properly connect command input to message handling
- Render both output and input areas together

### Solution 3: Add Terminal Toolbar
- Instantiate `TerminalToolbar` in `TerminalPanel`
- Add it to the view above the terminal area
- Wire up toolbar messages (toggle position, sync directory)

### Solution 4: Fix Message Handling
- Change `ExecuteCommand` to carry the command string
- Or extract from `self.command_input` AFTER clearing it to prevent re-execution
- Properly connect `CommandInput` message to text input

### Solution 5: Simplify Terminal Strategy
- For now, only use the Fallback strategy
- Remove external spawning that doesn't embed
- Later: implement proper embedding using Wezterm's IPC or other methods

## Testing Plan

1. Open terminal (Ctrl+`)
2. Verify toolbar appears with position toggle and sync buttons
3. Enter command in input field
4. Press Enter
5. Verify output appears in scrollable text area
6. Verify no readonly error message shows
7. Verify toolbar buttons work
8. Test with different commands (ls, pwd, echo, etc.)

## Files to Modify

1. `/src/widgets/terminal_panel.rs` - Fix view() and integrate toolbar
2. `/src/widgets/terminal_toolbar.rs` - Ensure proper event handling
3. `/src/core/terminal/strategy.rs` - Simplify to focus on fallback
