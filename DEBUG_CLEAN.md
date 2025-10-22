# 🧹 Clean Debug Logs - Simple Version

## ✨ The Cleanest Way to See Logs

**Just run this ONE command:**

```bash
RUST_LOG=debug cargo run 2>&1 | grep -E "📥|🔄|✅|📤|📌|⚠️"
```

That's it! Only clean logs, no noise.

---

## What to Expect

### Click View Button [🔲]

You'll see:
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
```

### Click Sort Button [⇅]

You'll see:
```
📥 App::update - Received RibbonMessage: ToggleSort
🔧 Calling ribbon_toolbar.update() with message: ToggleSort
🔄 RibbonToolbar::ToggleSort - OLD: Name -> NEW: Modified
📖 RibbonToolbar::get_sort() = Modified
✅ ToggleSort handler: Creating Message::SetSort(Modified, false)
📤 Emitting message and recursing: TabMessage(None, SetSort(...))
📬 App::Message::TabMessage - entity_opt: None
  ✅ Tab found, calling tab.update()
```

---

## Troubleshooting

**Missing a marker?** That's where the problem is!

| Missing | Problem | Fix |
|---------|---------|-----|
| 📥 | Button click not detected | Check button `on_press()` |
| 🔄 | State not changing | Check `update()` logic |
| ✅ | Handler not executing | Check match statement |
| 📤 | Message creation failed | Check `Message::TabView` |
| 📌 | Message routing broken | Check app handler |
| ⚠️ | Warnings | Check entity/tab |

---

## Other Filter Options

**Only view toggle:**
```bash
RUST_LOG=debug cargo run 2>&1 | grep "ToggleView"
```

**Only sort toggle:**
```bash
RUST_LOG=debug cargo run 2>&1 | grep "ToggleSort"
```

**Only state changes:**
```bash
RUST_LOG=debug cargo run 2>&1 | grep "OLD.*NEW"
```

**Save to file:**
```bash
RUST_LOG=debug cargo run 2>&1 | tee debug.log
grep "📥" debug.log  # View only input markers
```

---

## The 7-Step Perfect Flow

If you see all 7, the toggle **MUST work**:

1. ✅ `📥 Received RibbonMessage`
2. ✅ `🔧 Calling ribbon_toolbar.update()`
3. ✅ `🔄 OLD: X -> NEW: Y` (different!)
4. ✅ `📖 get_view/get_sort()` 
5. ✅ `✅ handler: Creating Message`
6. ✅ `📤 Emitting message`
7. ✅ `📌 TabView/TabMessage handler`

**Missing one = Found the bug!** 🎯
