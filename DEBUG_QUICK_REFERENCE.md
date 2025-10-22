# 🔍 Debug Quick Reference

## TL;DR - Get Started in 30 Seconds

```bash
cd /home/ehsator/Documents/GitHub/vortex-fm
./RUN_DEBUG.sh
# Click buttons, watch logs for: 📥 → 🔄 → ✅ → 📤 → 📌
```

## Quick Commands

| What | Command |
|------|---------|
| Run with logs | `./RUN_DEBUG.sh` |
| Run full verbose | `RUST_LOG=debug cargo run` |
| Run filtered | `./RUN_DEBUG.sh filter` |
| Save logs | `./RUN_DEBUG.sh save` |
| View saved logs | `cat debug.log` |
| Grep logs | `grep "📥" debug.log` |

## Log Marker Meanings

| Marker | Meaning | What to Check |
|--------|---------|---------------|
| 📥 | Button clicked | Button connection |
| 🔧 | Function called | Code execution |
| 🔄 | State changed | State mutation |
| 📖 | Value read | State getter |
| ✅ | Success/OK | Processing |
| 📤 | Message sent | Message creation |
| 📌 | Location/handler | Handler execution |
| 📬 | Message received | Message routing |
| 📍 | Entity resolved | Entity lookup |
| ⚠️ | Warning | Unexpected state |

## Expected Log Sequence

### View Toggle (Click grid/list button)
```
📥 → 🔧 → 🔄 → 📖 → ✅ → 📤 → 📌 → View Changes ✅
```

### Sort Toggle (Click sort button)
```
📥 → 🔧 → ⇅ → 📖 → ✅ → 📤 → 📬 → Sort Changes ✅
```

## Troubleshooting Checklist

- [ ] Run `./RUN_DEBUG.sh`
- [ ] Click view button
  - [ ] See `📥` (input received)?
  - [ ] See `🔄 OLD: → NEW:` (different values)?
  - [ ] See `📤` (message sent)?
  - [ ] See `📌` (TabView handler)?
- [ ] Click sort button
  - [ ] See `📥` (input received)?
  - [ ] See `⇅ OLD: → NEW:` (different values)?
  - [ ] See `📤` (message sent)?
  - [ ] See `📬` (TabMessage handler)?

## Common Issues

| Problem | Solution |
|---------|----------|
| No logs when clicking | Check button `on_press()` connection |
| `OLD: Grid → NEW: Grid` | State cycling not working, check `update()` |
| Logs show but UI doesn't change | Rendering issue, search for `TabConfig` |
| `⚠️ Tab not found` | Open a tab before clicking buttons |
| All markers but one missing | Check the code at that step |

## Key Code Locations

| Component | File | Line |
|-----------|------|------|
| Ribbon state | `src/views/ribbon_toolbar.rs` | `101-127` |
| Get view | `src/views/ribbon_toolbar.rs` | `129-131` |
| Get sort | `src/views/ribbon_toolbar.rs` | `133-135` |
| RibbonMessage handler | `src/app.rs` | `~5298-5331` |
| TabView handler | `src/app.rs` | `~4665-4680` |
| TabMessage handler | `src/app.rs` | `~4394-4415` |

## Filter Commands

```bash
# Only toggle logs
grep -E "ToggleView|ToggleSort" debug.log

# Only key markers
grep -E "📥|🔄|✅|📤|📌" debug.log

# Only warnings
grep "⚠️" debug.log

# Only specific function
grep "TabView" debug.log
```

## Full Documentation

- **DEBUG_GUIDE.md** - Comprehensive guide with examples
- **DEBUG_INSTRUCTIONS.md** - Step-by-step troubleshooting
- **RUN_DEBUG.sh** - Executable debug script

## Still Stuck?

Check DEBUG_INSTRUCTIONS.md for the full troubleshooting flowchart!
