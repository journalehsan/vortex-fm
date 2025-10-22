#!/bin/bash
# Debug script for Vortex FM - Ribbon Toolbar

echo "╔════════════════════════════════════════════════════════════╗"
echo "║       Vortex FM - Ribbon Toolbar Debug Runner             ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Building with debug logging enabled..."
echo ""

# Set environment variable for debug logging
export RUST_LOG=debug

# Optional: filter to only ribbon-related logs
if [ "$1" = "filter" ]; then
    echo "🔍 Running with filtered output (ribbon logs only)..."
    echo ""
    RUST_LOG=debug cargo run 2>&1 | grep -E "📥|🔧|🔄|✅|📤|📌|⚠️|RibbonMessage|ToggleView|ToggleSort|TabView|TabMessage"
elif [ "$1" = "save" ]; then
    echo "💾 Running and saving logs to debug.log..."
    echo ""
    RUST_LOG=debug cargo run 2>&1 | tee debug.log
    echo ""
    echo "✅ Logs saved to debug.log"
    echo ""
    echo "View logs with:"
    echo "  cat debug.log"
    echo "  cat debug.log | grep ToggleView"
    echo "  cat debug.log | grep '✅'"
else
    echo "🚀 Running with full debug output..."
    echo ""
    echo "When you see this, try:"
    echo "  1. Click the view toggle button [🔲] or [☰]"
    echo "  2. Click the sort button [⇅]"
    echo ""
    echo "Check the logs for these markers:"
    echo "  📥 = Button clicked"
    echo "  🔄 = State changed"
    echo "  ✅ = Handler processing"
    echo "  📤 = Message emitted"
    echo "  📌 = TabView/TabMessage handler"
    echo "  ⚠️  = Warnings"
    echo ""
    echo "Press Ctrl+C to exit"
    echo "────────────────────────────────────────────────────────────"
    echo ""
    RUST_LOG=debug cargo run
fi
