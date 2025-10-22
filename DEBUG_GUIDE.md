# Debug Guide - Ribbon Toolbar Toggle Buttons

## 🐛 How to Debug Using Logs

### Enable Debug Logging

To see the debug logs, run with debug output enabled:

```bash
RUST_LOG=debug cargo run
```

Or for even more verbose output:

```bash
RUST_LOG=vortex_fm=debug cargo run
```

### What Logs to Look For

#### 1. **Button Click Detection**
```
📥 App::update - Received RibbonMessage: ToggleView
```
✅ This means the button click was registered and the message was sent

#### 2. **Ribbon Toolbar State Update**
```
🔧 Calling ribbon_toolbar.update() with message: ToggleView
🔄 RibbonToolbar::ToggleView - OLD: Grid -> NEW: List
📖 RibbonToolbar::get_view() = List
```
✅ This shows the state actually changed from Grid to List

#### 3. **Message Creation**
```
✅ ToggleView handler: Creating Message::TabView(None, List)
```
✅ The new message with the new view was created

#### 4. **Message Emission**
```
📤 Emitting message and recursing: TabView(None, List)
```
✅ The message is being sent to the app update

#### 5. **TabView Handler**
```
📌 App::Message::TabView - Changing view to: List
  ✅ Found tab entity: updating tab config view
  ✅ Updating app config view
  ✅ Syncing ribbon toolbar view
  📤 Recursing with Message::TabConfig
```
✅ The view is being applied to the tab

### Full Debug Flow for View Toggle

```
┌─────────────────────────────────────────────────────────┐
│ User clicks view button [🔲]                            │
└─────────────────────────────────────────────────────────┘
                        ↓
📥 App::update - Received RibbonMessage: ToggleView
                        ↓
🔧 Calling ribbon_toolbar.update()
                        ↓
🔄 RibbonToolbar::ToggleView - OLD: Grid -> NEW: List
                        ↓
📖 RibbonToolbar::get_view() = List
                        ↓
✅ ToggleView handler: Creating Message::TabView(None, List)
                        ↓
📤 Emitting message and recursing
                        ↓
📌 App::Message::TabView - Changing view to: List
                        ↓
  ✅ Found tab entity: updating tab config view
  ✅ Updating app config view
  ✅ Syncing ribbon toolbar view
                        ↓
📤 Recursing with Message::TabConfig
                        ↓
View changes to List! ✅
```

### Full Debug Flow for Sort Toggle

```
┌─────────────────────────────────────────────────────────┐
│ User clicks sort button [⇅]                            │
└─────────────────────────────────────────────────────────┘
                        ↓
📥 App::update - Received RibbonMessage: ToggleSort
                        ↓
🔧 Calling ribbon_toolbar.update()
                        ↓
⇅ RibbonToolbar::ToggleSort - OLD: Name -> NEW: Modified
                        ↓
📖 RibbonToolbar::get_sort() = Modified
                        ↓
✅ ToggleSort handler: Creating Message::SetSort(Modified, false)
                        ↓
📤 Emitting message and recursing
                        ↓
📬 App::Message::TabMessage - entity_opt: None, message: SetSort(...)
                        ↓
  📍 Using entity: [active tab]
  ✅ Tab found, calling tab.update()
                        ↓
Sort changes to Date! ✅
```

## 🔍 Troubleshooting Using Logs

### Scenario 1: No logs appear when clicking button

**Problem:** Button click not detected

**Check:**
```
📥 App::update - Received RibbonMessage: ...
```

**If missing:** 
- Button not connected to app message handler
- Click event not being captured
- Check button `on_press()` implementation

### Scenario 2: Logs show RibbonMessage but no state change

**Problem:** Message received but state not updating

**Check:**
```
🔄 RibbonToolbar::ToggleView - OLD: Grid -> NEW: ???
```

**If shows same value:**
- `update()` method not working
- State mutation not happening
- Check `ribbon_toolbar.rs` update logic

### Scenario 3: State changes but view doesn't update

**Problem:** Message handled but UI not reflecting

**Check:**
```
📌 App::Message::TabView - Changing view to: List
  ✅ Found tab entity: updating tab config view
```

**If all show but UI doesn't change:**
- Tab view rendering might be cached
- View component not responding to config change
- Check tab view rendering code

### Scenario 4: "Tab not found" warning

**Problem:** Cannot find tab entity

**Logs:**
```
⚠️  Tab not found for entity: ...
```

**Fix:**
- Entity might be invalid
- Check that active tab exists
- Verify tab model is initialized

## 📊 Log Analysis Tips

### Use grep to filter logs

```bash
# Only see toggle logs
RUST_LOG=debug cargo run 2>&1 | grep -E "Toggle|RibbonMessage"

# Only see warnings
RUST_LOG=debug cargo run 2>&1 | grep "⚠️"

# See the complete flow
RUST_LOG=debug cargo run 2>&1 | grep -E "📥|🔧|🔄|✅|📤|📌"
```

### Compare before and after behavior

```bash
# Run with detailed logs and save to file
RUST_LOG=debug cargo run 2>&1 | tee debug.log

# Then analyze with grep/awk
grep "ToggleView\|ToggleSort" debug.log
```

## 🎯 What Each Log Level Tells You

### `log::debug!()` - Detailed information
- Function entry/exit points
- Variable values
- State transitions
- Used for development/debugging

### `log::info!()` - General information
- Important lifecycle events
- High-level operations
- Used for production

### `log::warn!()` - Warnings
- Unexpected but recoverable situations
- "Tab not found" type errors
- Suggests something unusual happened

### `log::error!()` - Errors
- Critical failures
- Operations that failed completely
- Prevents normal operation

## 🚀 Running Tests with Debug Output

```bash
# Run with debug logs
RUST_LOG=debug cargo test --lib

# Run specific test with logs
RUST_LOG=debug cargo test --lib test_name -- --nocapture
```

## 📝 Adding Your Own Logs

To add more debugging information:

```rust
// Log a value
log::debug!("Current view: {:?}", current_view);

// Log with emoji for easy spotting
log::debug!("🎯 Processing: {}", value);

// Log entry and exit
log::debug!("→ Entering function");
// ... function code ...
log::debug!("← Exiting function");

// Conditional logging
if some_condition {
    log::warn!("⚠️  Unexpected state!");
}
```

## 🔗 Related Files

- `src/views/ribbon_toolbar.rs` - Ribbon state and cycling logic
- `src/app.rs` - Message handling and routing
- `src/tab.rs` - Tab view and sort handling

## ✅ Verification Checklist

When toggle buttons aren't working, verify each step:

- [ ] See "📥 Received RibbonMessage"?
- [ ] See "🔄 OLD: X -> NEW: Y" (different values)?
- [ ] See "✅ ToggleView/ToggleSort handler"?
- [ ] See "📤 Emitting message"?
- [ ] See "📌 TabView/TabMessage handler"?
- [ ] See "✅ Found tab entity"?
- [ ] View/sort actually changed on screen?

If any step is missing, focus debugging on that area!

---

**Pro Tip:** Copy the complete flow above and cross-reference with your actual logs to see where things break!
