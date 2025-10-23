# Terminal Implementation - Visual Architecture

## System Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    VORTEX FILE MANAGER                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌────────────────────┐  ┌─────────────────────────────────┐  │
│  │  File Browser      │  │   Terminal Panel (Optional)     │  │
│  │  - Navigation      │  ├─────────────────────────────────┤  │
│  │  - File ops        │  │ ┌─────────────────────────────┐ │  │
│  │  - Sorting         │  │ │   Terminal Toolbar          │ │  │
│  │                    │  │ │ [Path] [Position] [Sync]   │ │  │
│  │                    │  │ └─────────────────────────────┘ │  │
│  │                    │  │ ┌─────────────────────────────┐ │  │
│  │                    │  │ │  Output Area (Scrollable)   │ │  │
│  │                    │  │ │  $ ls -la                   │ │  │
│  │                    │  │ │  drwxr-xr-x user group ...  │ │  │
│  │                    │  │ │  -rw-r--r-- user group ...  │ │  │
│  │                    │  │ └─────────────────────────────┘ │  │
│  │                    │  │ ┌─────────────────────────────┐ │  │
│  │                    │  │ │ $ [___________________]      │ │  │
│  │                    │  │ │   (Command Input)           │ │  │
│  │                    │  │ └─────────────────────────────┘ │  │
│  └────────────────────┘  └─────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Message Flow Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                     USER INTERACTION                             │
└──────────────────────────────────────────────────────────────────┘
                              │
         ┌────────────────────┼────────────────────┐
         ▼                    ▼                    ▼
    ┌─────────┐         ┌──────────┐      ┌──────────────┐
    │ Input   │         │Position  │      │Sync Button   │
    │Field    │         │Toggle    │      │              │
    └────┬────┘         └────┬─────┘      └──────┬───────┘
         │                   │                    │
         │                   │                    │
         ▼                   ▼                    ▼
    ┌────────────────────────────────────────────────┐
    │    TerminalPanel Message Handling              │
    │                                                │
    │  • CommandInput → Update input state          │
    │  • CommandSubmit → Execute command            │
    │  • TogglePosition → Toggle layout             │
    │  • SyncDirectory → Update working dir         │
    └────────────┬─────────────────────────────────┘
                 │
                 ▼
    ┌────────────────────────────────────────────────┐
    │    Message::TerminalPanelMessage              │
    │    (Maps to App message enum)                 │
    └────────────┬─────────────────────────────────┘
                 │
                 ▼
    ┌────────────────────────────────────────────────┐
    │    App::update()                              │
    │    - Receives mapped message                  │
    │    - Routes to terminal.update()              │
    │    - Requests re-render                       │
    └────────────┬─────────────────────────────────┘
                 │
                 ▼
    ┌────────────────────────────────────────────────┐
    │    TerminalPanel::update()                    │
    │    - Updates internal state                   │
    │    - Executes commands                        │
    │    - Updates output buffer                    │
    └────────────┬─────────────────────────────────┘
                 │
                 ▼
    ┌────────────────────────────────────────────────┐
    │    TerminalSessionManager (async)             │
    │    - Command execution                        │
    │    - Output capture                           │
    │    - History tracking                         │
    └────────────┬─────────────────────────────────┘
                 │
                 ▼
    ┌────────────────────────────────────────────────┐
    │    FallbackStrategy                           │
    │    tokio::process::Command                    │
    └────────────┬─────────────────────────────────┘
                 │
                 ▼
    ┌────────────────────────────────────────────────┐
    │    System Shell (/bin/sh)                     │
    │    - bash, zsh, dash, etc.                    │
    └────────────┬─────────────────────────────────┘
                 │
                 ▼
    ┌────────────────────────────────────────────────┐
    │    Command Output                             │
    │    - stdout                                   │
    │    - stderr                                   │
    │    - exit code                                │
    └────────────┬─────────────────────────────────┘
                 │
                 ▼
    ┌────────────────────────────────────────────────┐
    │    Output Buffer                              │
    │    (TerminalOutputLine vec)                   │
    └────────────┬─────────────────────────────────┘
                 │
                 ▼
    ┌────────────────────────────────────────────────┐
    │    View Re-render                             │
    │    Terminal displays new output               │
    └────────────────────────────────────────────────┘
```

---

## Component Structure

```
TerminalPanel (Main Widget)
├── toolbar: TerminalToolbar
│   ├── position: TerminalPosition (Bottom/Right)
│   ├── current_path: String
│   └── is_synced: bool
│
├── strategy: Box<dyn TerminalStrategy>
│   └── FallbackStrategy
│       └── output_buffer: Vec<String>
│
├── session_manager: Arc<Mutex<TerminalSessionManager>>
│   ├── session: Option<TerminalSession>
│   ├── process: Option<Child>
│   ├── output_lines: Vec<TerminalOutputLine>
│   └── command_history: Vec<String>
│
├── position: TerminalPosition
├── is_visible: bool
├── current_dir: PathBuf
├── input_mode: TerminalInputMode
├── command_input: String
├── path_input: String
├── output_buffer: Vec<TerminalOutputLine>
└── scroll_offset: usize
```

---

## State Machine

```
                    START
                      │
                      ▼
          ┌───────────────────────┐
          │   Terminal Hidden     │◄──────┐
          │  terminal_visible=    │       │
          │  false                │       │
          └──────────┬────────────┘       │
                     │                    │
          TerminalToggle (Ctrl+`)         │
                     │                    │
                     ▼                    │
          ┌───────────────────────┐       │
          │  Terminal Visible     │       │
          │  Create TerminalPanel │       │
          │  terminal_visible=    │       │
          │  true                 │       │
          └──────────┬────────────┘       │
                     │                    │
        ┌────────────┼────────────┐       │
        │            │            │       │
        ▼            ▼            ▼       │
    ┌─────┐   ┌──────────┐  ┌────────┐   │
    │Input│   │Toolbar   │  │Display │   │
    │Text │   │Controls  │  │Output  │   │
    └──┬──┘   └────┬─────┘  └───┬────┘   │
       │           │            │        │
       │(Enter)    │(Click)     │(Render)
       │           │            │        │
       ▼           ▼            ▼        │
    ┌─────────────────────────────────┐  │
    │  Execute Command               │  │
    │  - Async (Tokio)              │  │
    │  - Update Buffer              │  │
    │  - Clear Input                │  │
    └──────────────┬─────────────────┘  │
                   │                    │
                   ▼                    │
    ┌───────────────────────────────────┐│
    │  Output Buffer Updated            ││
    │  - Add command echo              ││
    │  - Add stdout/stderr             ││
    │  - Ready for re-render           ││
    └──────────────┬────────────────────┘│
                   │                    │
                   └────────────────────┘
                   (Loop ready for next
                    command)
```

---

## Class Hierarchy

```
TerminalStrategy (Trait)
├── WeztermStrategy (impl)
│   └── Available: wezterm --version
│       → Currently falls back to Fallback
│
├── AlacrittyStrategy (impl)
│   └── Available: alacritty --version
│       → Currently falls back to Fallback
│
└── FallbackStrategy (impl) ✅ ACTIVE
    └── Available: Always
        → Uses tokio::process::Command

App (Main)
├── terminal_visible: bool
├── terminal_has_focus: bool
└── terminal_panel: Option<TerminalPanel>

TerminalPanel (Widget)
├── strategy: Box<dyn TerminalStrategy>
├── toolbar: TerminalToolbar
└── session_manager: Arc<Mutex<TerminalSessionManager>>

Message Enum
└── TerminalPanelMessage(TerminalMessage)

TerminalMessage Enum
├── CommandInput(String)
├── CommandSubmit
├── ExecuteCommand(String)
├── TogglePosition
├── SyncDirectory
└── ... (more variants)
```

---

## Async Execution Flow

```
UI Thread                          Tokio Runtime
─────────────────────────────────────────────────────

User presses Enter
│
├─→ CommandSubmit message
│
├─→ TerminalPanel::update()
│
├─→ clone(session_manager)
│
└─→ tokio::spawn(async {
       │
       ├─→ [Background]
       │   ├─→ Acquire lock on session_manager
       │   ├─→ Call execute_command()
       │   ├─→ Execute: sh -c "user_command"
       │   ├─→ Capture stdout/stderr
       │   ├─→ Get exit code
       │   ├─→ Update output_buffer
       │   └─→ Release lock
       │
       └─→ [Queue update for UI thread]
           └─→ View re-renders with new output

No blocking! UI stays responsive.
```

---

## Data Types

```
TerminalPosition
├── Bottom (panel below file browser)
└── Right (panel to the right)

TerminalInputMode
├── Command (typing shell commands)
└── Path (navigating directories)

TerminalBackend
├── Wezterm
├── Alacritty
└── Fallback ✅

TerminalMessage
├── CommandInput(String)
├── CommandSubmit
├── ExecuteCommand(String)
├── TogglePosition
├── SyncDirectory
├── ToggleInputMode
├── SetInputMode(TerminalInputMode)
├── OutputReceived(String)
├── SessionStarted
├── SessionEnded
├── Error(String)
├── PathInput(String)
└── PathSubmit

TerminalOutputLine
├── content: String
├── is_error: bool
└── timestamp: SystemTime

TerminalSession
├── id: String
├── working_directory: PathBuf
├── is_busy: bool
├── backend: TerminalBackend
└── process_id: Option<u32>
```

---

## Key Design Decisions

1. **Fallback Strategy Default**
   - Reason: Reliable, works everywhere
   - Benefit: User gets working terminal immediately
   - Future: Can add Wezterm/Alacritty embedding later

2. **Async Execution with Tokio**
   - Reason: Prevents UI blocking
   - Benefit: Smooth user experience
   - Architecture: Message-driven updates

3. **Separate Toolbar**
   - Reason: Keep concerns separated
   - Benefit: Easy to modify toolbar independently
   - Architecture: Composed into TerminalPanel

4. **Arc<Mutex<>> Session Manager**
   - Reason: Shared async access
   - Benefit: Can clone for spawned tasks
   - Architecture: Thread-safe state management

5. **Output Buffer Instead of Direct Display**
   - Reason: Maintains history
   - Benefit: Scroll through previous commands
   - Architecture: Re-renderable state

---

This architecture ensures:
✅ Responsive UI
✅ Proper message flow
✅ Scalable async execution
✅ Clean component separation
✅ Future extensibility
