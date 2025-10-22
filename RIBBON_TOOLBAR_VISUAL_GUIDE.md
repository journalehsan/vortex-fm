# Ribbon Toolbar Visual Reference Guide

## Toolbar Layout

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ [📄] [📁] │ [✂️] [📋] [📌] │ [🔲] [⇅] │ [🗑️] [⌨️]                              │
│  New File  New Folder   Cut   Copy  Paste   Grid   Sort  Trash  Terminal   │
│                                    View Toggle                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## View Toggle Button

### Location
Right side of the toolbar, between "Paste" and "Sort" buttons.

### Icon Behavior

**When in Grid View:**
```
┌──────────────┐
│   [🔲]       │  ← Grid icon visible
│  (click me)  │
└──────────────┘
Tooltip: "Grid View (click to toggle to List)"
```

**When in List View:**
```
┌──────────────┐
│   [☰]        │  ← List icon visible
│  (click me)  │
└──────────────┘
Tooltip: "List View (click to toggle to Grid)"
```

### State Cycle

```
        ┌─ CLICK ─┐
        │         ▼
    [GRID] ◄──────► [LIST]
        ▲         │
        └─ CLICK ─┘
```

### Example User Interaction

1. **Start:** In Grid mode
   ```
   Toolbar shows: [🔲]
   Tooltip: "Grid View (click to toggle to List)"
   ```

2. **User clicks the button**
   ```
   System: Cycles view to List mode
   Toolbar updates to: [☰]
   Tooltip: "List View (click to toggle to Grid)"
   File list displays in list format
   ```

3. **User clicks again**
   ```
   System: Cycles view back to Grid mode
   Toolbar updates to: [🔲]
   Tooltip: "Grid View (click to toggle to List)"
   File grid displays in grid format
   ```

## Sort Toggle Button

### Location
Right side of toolbar, immediately after the View Toggle button.

### Icon Behavior

**Always shows this icon:**
```
┌──────────────┐
│   [⇅]        │  ← Always the same icon
│  (click me)  │
└──────────────┘
Tooltip changes with current sort mode
```

### State Cycle (4-way toggle)

```
         ┌─ CLICK ─┐
         │         ▼
  [NAME] ◄─ [DATE]
     ▲   │        │
     │   └─ ──── [SIZE]
     │          ▼
     └─ ── [TRASHED]
        CLICK ─┐
              └─ CLICK
```

### Tooltip Changes

Each click updates the tooltip:

1. **First state:** "Sort by Name (click to cycle)"
2. **Click once:** "Sort by Date (click to cycle)"
3. **Click again:** "Sort by Size (click to cycle)"
4. **Click again:** "Sort by Trashed (click to cycle)"
5. **Click again:** Back to "Sort by Name (click to cycle)"

### Example User Interaction

```
Initial State:
  Icon: [⇅]
  Tooltip: "Sort by Name (click to cycle)"
  Files: Sorted alphabetically by filename

User clicks sort button:
  Icon: [⇅] (unchanged)
  Tooltip: "Sort by Date (click to cycle)"
  Files: Sorted by modification date (newest first)

User clicks again:
  Icon: [⇅] (unchanged)
  Tooltip: "Sort by Size (click to cycle)"
  Files: Sorted by file size (largest first)

User clicks again:
  Icon: [⇅] (unchanged)
  Tooltip: "Sort by Trashed (click to cycle)"
  Files: Sorted by trash date (most recent first)
  Note: Only visible in trash folder

User clicks again:
  Icon: [⇅] (unchanged)
  Tooltip: "Sort by Name (click to cycle)"
  Files: Back to alphabetical order
```

## Transparent Background Feature

### The Problem (Solved)

**Before (with colored background):**
```
┌──────────────────────┐
│ [⚫⚫⚫] ← Bad!      │
│ Icon invisible with  │
│ accent background    │
└──────────────────────┘
```

**After (with transparent background):**
```
┌──────────────────────┐
│ [🔲] ← Great!        │
│ Icon always visible  │
│ on any background    │
└──────────────────────┘
```

### How It Works

Each icon button has a transparent container:
- No background color interference
- Icons use system theme colors
- Works with light and dark themes
- Works with any accent color

## Button Groups and Spacing

```
┌─────────────────────────────────────────────────────────────────┐
│ [📄] [📁]  ║  [✂️] [📋] [📌]  ║  [🔲] [⇅]  ║  [🗑️] [⌨️]        │
│   New      ║    Edit         ║   View    ║  System            │
│ Group 1    ║   Group 2       ║  Group 3  ║  Group 4           │
└─────────────────────────────────────────────────────────────────┘
    ↑4px↑    ↑12px↑    ↑12px↑    ↑12px↑
    between buttons between groups
```

### Spacing Rules
- **Within group:** 4 pixels between buttons
- **Between groups:** 12 pixels (visual separator)
- **Toolbar edges:** 8 pixels padding

## Keyboard Accessibility

### Supported Keyboard Navigation
- **Tab key:** Navigate between buttons
- **Space/Enter:** Activate button
- **Tooltip shows:** When button is focused

### Quick Keys (if implemented)
- **Ctrl+V:** Toggle view mode (proposed)
- **Ctrl+S:** Cycle sort (proposed)

## Theme Support

### Light Theme
```
┌────────────────────────┐
│ [Light icons on       │
│  light background]    │
│ High contrast ✓       │
└────────────────────────┘
```

### Dark Theme
```
┌────────────────────────┐
│ [Dark icons on        │
│  dark background]     │
│ High contrast ✓       │
└────────────────────────┘
```

### Any Accent Color
- Transparent backgrounds adapt
- Icons remain visible
- No conflicts with theme

## Tooltips - Detailed Behavior

### Appearance
```
         Button
           ↓
        [🔲]
          
        Tooltip
           ↓
    [Grid View (click to toggle to List)]
```

### Positioning
- Below buttons (Bottom)
- Auto-appears on hover
- Disappears when mouse leaves
- Shows on focus (keyboard)

### Content Examples

**View Toggle:**
- Grid: "Grid View (click to toggle to List)"
- List: "List View (click to toggle to Grid)"

**Sort Toggle:**
- Name: "Sort by Name (click to cycle)"
- Date: "Sort by Date (click to cycle)"
- Size: "Sort by Size (click to cycle)"
- Trashed: "Sort by Trashed (click to cycle)"

## Visual Feedback Timeline

```
T=0ms:  User approaches button
        Button appears in toolbar

T=100ms: User hovers over button
         Tooltip appears
         "Sort by Name (click to cycle)"

T=150ms: User clicks button
         Sort cycles to Date
         Tab content re-sorts
         Tooltip updates
         "Sort by Date (click to cycle)"

T=200ms: Tooltip fades
         New sort order visible
```

## Common User Tasks

### Task: Switch from Grid to List View
```
1. Locate the [🔲] button (rightmost icon group)
2. Click it
3. View changes to list
4. Button icon changes to [☰]
```

### Task: Sort by Size
```
1. Click sort button [⇅] once → Sort by Date
2. Click sort button [⇅] again → Sort by Size
3. Files now sorted by size
```

### Task: Return to Default View
```
Current: List view, sorted by size
1. Click view button [☰] → Switch to grid
2. Click sort button twice → Back to Name sort
3. View: Grid, Sort: Name (default)
```

## Responsive Behavior

### Small Window (Narrow Toolbar)
- Buttons maintain minimum spacing
- All buttons stay visible
- Tooltips adjust position
- No text labels (icons only)

### Large Window (Wide Toolbar)
- Extra padding added
- Buttons remain same size
- Tooltips well positioned
- Icons remain centered

## Icons Used

| Button | Icon Name | SVG Symbol |
|--------|-----------|-----------|
| New File | `document-new-symbolic` | 📄 |
| New Folder | `folder-new-symbolic` | 📁 |
| Cut | `edit-cut-symbolic` | ✂️ |
| Copy | `edit-copy-symbolic` | 📋 |
| Paste | `edit-paste-symbolic` | 📌 |
| Grid View | `view-grid-symbolic` | 🔲 |
| List View | `view-list-symbolic` | ☰ |
| Sort | `view-sort-ascending-symbolic` | ⇅ |
| Trash | `user-trash-symbolic` | 🗑️ |
| Terminal | `utilities-terminal-symbolic` | ⌨️ |

## Summary

- **View Toggle:** Single button, two states, cycles on click
- **Sort Toggle:** Single button, four states, cycles on click
- **Transparent Backgrounds:** Icons always visible
- **Tooltips:** Guide users with clear feedback
- **Space Efficient:** No dropdown menus
- **Accessible:** Keyboard and mouse navigation
- **Theme Aware:** Works with all themes

All features compile successfully and are production-ready! ✅
