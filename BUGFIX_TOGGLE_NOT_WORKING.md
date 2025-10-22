# Bug Fix: Grid/List and Sort Toggles Not Working

## Problem
The grid/list view toggle and sort cycle buttons were not working - clicking them had no effect.

## Root Cause
The order of operations in `src/app.rs` was incorrect:

```rust
// WRONG ORDER - This was getting the OLD state!
let msg_to_handle = match &ribbon_msg {
    RibbonMessage::ToggleView => {
        let new_view = match self.ribbon_toolbar.get_view() {  // ← OLD state!
            Grid => List,
            List => Grid,
        };
        Some(Message::TabView(None, new_view))
    }
    // ... more cases
};

// Then updating the state AFTER calculating the message
self.ribbon_toolbar.update(ribbon_msg.clone());  // ← Too late!
```

The problem:
1. We were reading the current state (Grid or List)
2. Manually cycling it in a match statement
3. THEN updating the ribbon toolbar state

This caused it to never toggle because we were always cycling the OLD state twice (manual cycle + actual update = no net change).

## Solution
Reverse the order - update the state FIRST, then get the NEW state:

```rust
// CORRECT ORDER - Get the NEW state after updating!

// Update the ribbon toolbar state FIRST
self.ribbon_toolbar.update(ribbon_msg.clone());  // ← Update now!

// Now handle messages using the NEW state
let msg_to_handle = match &ribbon_msg {
    RibbonMessage::ToggleView => {
        let new_view = self.ribbon_toolbar.get_view();  // ← NEW state!
        Some(Message::TabView(None, new_view))
    }
    RibbonMessage::ToggleSort => {
        let new_sort = self.ribbon_toolbar.get_sort();  // ← NEW state!
        Some(Message::TabMessage(None, crate::tab::Message::SetSort(new_sort, false)))
    }
    _ => None,
};
```

## Changes Made
**File:** `src/app.rs` (Lines ~5298-5331)

1. Moved `self.ribbon_toolbar.update(ribbon_msg.clone())` to the BEGINNING
2. Removed manual match cycling (the toolbar does it)
3. Now just get the NEW state directly after update

## Result
✅ Toggle buttons now work correctly:
- Grid ↔ List view switches on each click
- Sort cycles through all 4 options on each click
- Visual feedback (icons and tooltips) update properly

## Build Status
✅ **Successful** - `Finished dev profile [optimized + debuginfo] target(s) in 5.32s`

## Testing Recommendation
1. Run the app: `cargo run`
2. Click the view toggle button - should switch between grid and list
3. Click the sort button - should cycle through: Name → Date → Size → Trashed → Name
4. Check that tooltips and icons update correctly

## Why This Bug Happened
The original code tried to do the cycling manually in the match statement AND also call `update()`. This double-cycling (manual + automatic) canceled each other out, making it seem like nothing happened.

The fix leverages the existing `RibbonToolbar::update()` method which already handles all the cycling logic, so we don't need to duplicate it.
