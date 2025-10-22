# 🔍 Debug Instructions - Ribbon Toolbar Toggle Buttons

## Quick Start

```bash
# Make the debug script executable (first time only)
chmod +x RUN_DEBUG.sh

# Run with debug logging
./RUN_DEBUG.sh
```

## What to Do

1. **Run the debug version** with logs enabled
2. **Click the buttons** in the toolbar:
   - View toggle button [🔲] or [☰]
   - Sort button [⇅]
3. **Watch the logs** for the expected sequence
4. **Identify where it breaks** using the flow diagram

## Expected Log Sequence - View Toggle

When you click the view button, you should see:

```
📥 App::update - Received RibbonMessage: ToggleView
🔧 Calling ribbon_toolbar.update() with message: ToggleView
🔄 RibbonToolbar::ToggleView - OLD: Grid -> NEW: List
📖 RibbonToolbar::get_view() = List
✅ ToggleView handler: Creating Message::TabView(None, List)
📤 Emitting message and recursing: TabView(None, List)
📌 App::Message::TabView - Changing view to: List
  ✅ Found tab entity: updating tab config view
  ✅ Updating app config view
  ✅ Syncing ribbon toolbar view
  📤 Recursing with Message::TabConfig
```

**If you see all of this, but view doesn't change:**
- Problem is in the tab rendering or config application
- Search for `Message::TabConfig` handler in logs

## Expected Log Sequence - Sort Toggle

When you click the sort button, you should see:

```
📥 App::update - Received RibbonMessage: ToggleSort
🔧 Calling ribbon_toolbar.update() with message: ToggleSort
⇅ RibbonToolbar::ToggleSort - OLD: Name -> NEW: Modified
📖 RibbonToolbar::get_sort() = Modified
✅ ToggleSort handler: Creating Message::SetSort(Modified, false)
📤 Emitting message and recursing: TabMessage(None, SetSort(...))
📬 App::Message::TabMessage - entity_opt: None
  📍 Using entity: [some number]
  ✅ Tab found, calling tab.update()
```

**If you see all of this, but sort doesn't change:**
- Problem is in the tab's SetSort message handler

## Filtering Logs

### Only Show Ribbon Toggle Logs
```bash
RUST_LOG=debug cargo run 2>&1 | grep -E "ToggleView|ToggleSort"
```

### Only Show Key Markers
```bash
RUST_LOG=debug cargo run 2>&1 | grep -E "📥|🔄|✅|📤|📌"
```

### Only Show Warnings/Errors
```bash
RUST_LOG=debug cargo run 2>&1 | grep -E "⚠️|error"
```

### Save to File for Analysis
```bash
./RUN_DEBUG.sh save
cat debug.log | grep ToggleView
```

## Troubleshooting Flowchart

```
Do you see "📥 Received RibbonMessage"?
  │
  ├─ NO → Button click not detected
  │       • Check ribbon_toolbar.rs button implementation
  │       • Verify on_press() is connected
  │
  └─ YES → Do you see "🔄 OLD: X -> NEW: Y"?
            │
            ├─ NO → State not updating
            │       • Check RibbonToolbar::update() logic
            │       • Verify match statement syntax
            │
            └─ YES → Do you see "✅ handler: Creating Message"?
                      │
                      ├─ NO → Handler not reached
                      │       • Check match statement
                      │       • Verify message type
                      │
                      └─ YES → Do you see "📤 Emitting message"?
                                │
                                ├─ NO → Message creation failed
                                │       • Check message construction
                                │
                                └─ YES → Do you see "📌 TabView/TabMessage"?
                                          │
                                          ├─ NO → Message routing broken
                                          │       • Check app message handling
                                          │
                                          └─ YES → Do you see "✅ Found tab entity"?
                                                    │
                                                    ├─ NO → Tab entity not found
                                                    │       • Check tab model
                                                    │       • Verify active tab exists
                                                    │
                                                    └─ YES → UI should change!
                                                             If not:
                                                             • Check rendering code
                                                             • Search for TabConfig handler
```

## Common Issues & Solutions

### Issue: No logs when clicking button

**Possible causes:**
1. Button click not detected
2. Message not dispatched
3. Logs not enabled

**Solution:**
```bash
# Make sure logs are enabled
RUST_LOG=debug cargo run

# Test if button is being clicked at all
RUST_LOG=debug cargo run 2>&1 | grep -i "ribbon"
```

---

### Issue: "🔄 OLD: Grid -> NEW: Grid" (No change)

**Meaning:** State cycling failed

**Possible causes:**
1. Match statement logic error
2. State mutation not happening
3. Wrong branch taken

**Solution:**
- Check the match statement in `RibbonToolbar::update()`
- Verify Grid => List and List => Grid logic
- Add more logging to match branches

---

### Issue: All logs appear, but UI doesn't change

**Meaning:** Message is being processed, but not reflected in UI

**Possible causes:**
1. Tab rendering not responding to config change
2. View state not propagating
3. UI cache not invalidating

**Solution:**
- Search logs for "TabConfig" to see if it's being applied
- Check tab rendering code for view/sort changes
- Look for any caching or memoization

---

### Issue: "⚠️ Tab not found"

**Meaning:** Tab entity is invalid or tab model is empty

**Possible causes:**
1. No active tab
2. Tab closed
3. Entity incorrect

**Solution:**
- Make sure you have at least one open tab
- Create a new tab before clicking buttons
- Check logs for tab creation

---

## Debug Points in Code

### src/views/ribbon_toolbar.rs
```rust
pub fn update(&mut self, message: RibbonMessage) {
    // Look for:
    // "🔄 RibbonToolbar::ToggleView"
    // "⇅ RibbonToolbar::ToggleSort"
    // "📖 RibbonToolbar::get_view()"
    // "📖 RibbonToolbar::get_sort()"
}
```

### src/app.rs (Lines ~5298-5323)
```rust
Message::RibbonMessage(ribbon_msg) => {
    // Look for:
    // "📥 App::update - Received RibbonMessage"
    // "🔧 Calling ribbon_toolbar.update()"
    // "✅ ToggleView/ToggleSort handler"
    // "📤 Emitting message"
}
```

### src/app.rs (Lines ~4665-4680)
```rust
Message::TabView(entity_opt, view) => {
    // Look for:
    // "📌 App::Message::TabView"
    // "✅ Found tab entity"
    // "✅ Syncing ribbon toolbar"
}
```

### src/app.rs (Lines ~4394-4415)
```rust
Message::TabMessage(entity_opt, tab_message) => {
    // Look for:
    // "📬 App::Message::TabMessage"
    // "✅ Tab found, calling tab.update()"
}
```

## Advanced Debugging

### Log Only Specific Message
```bash
RUST_LOG=vortex_fm::views::ribbon_toolbar=debug cargo run
```

### Increase Log Verbosity
```bash
RUST_LOG=trace cargo run
```

### Save and Analyze Logs
```bash
./RUN_DEBUG.sh save
# Open debug.log in your editor
# Search for patterns like "OLD: Grid -> NEW" to see state changes
```

### Compare Multiple Runs
```bash
# First test
./RUN_DEBUG.sh save
mv debug.log debug_run1.log

# Second test
./RUN_DEBUG.sh save
mv debug.log debug_run2.log

# Compare
diff debug_run1.log debug_run2.log
```

## Getting Help

When asking for help, provide:
1. The complete log output (use `./RUN_DEBUG.sh save`)
2. What you clicked
3. What changed (or what didn't)
4. Which log markers you see
5. Which markers are missing

Use this format:
```
I clicked: [View/Sort button]
I see logs: 📥 📌 ✅ (list what you see)
Missing: 📤 (list what's not there)
UI changed: Yes/No
```

---

**Good luck debugging! The logs will show you exactly where things break.** 🎯
